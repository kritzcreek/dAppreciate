use certified_map::{AsHashTree, RbTree};
use hashtree::{Hash};
use ic_cdk::api::call::CallResult;
use ic_cdk::api::{set_certified_data, data_certificate};
use ic_cdk::export::{
    candid::{CandidType, Deserialize},
    Principal,
};
use ic_cdk_macros::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::borrow::Cow;
use ic_cdk::trap;
use serde::Serialize;
use serde_bytes::{ByteBuf, Bytes};
use ic_cdk::export::candid::Func;

mod assets;

const LABEL_ASSETS: &[u8] = b"http_assets";

thread_local! {
    static STATE: State = State {
        donor_to_client_map: RefCell::new(DonorToClientMap::new()),
        asset_hashes: RefCell::new(AssetHashes::default()),
    };
    static ASSETS: RefCell<HashMap<&'static str, (Vec<HeaderField>, &'static [u8])>> = RefCell::new(HashMap::default());
}

struct State {
    donor_to_client_map: RefCell<DonorToClientMap>,
    asset_hashes: RefCell<AssetHashes>,
}

type HeaderField = (String, String);

type AssetHashes = RbTree<&'static str, Hash>;

#[derive(Clone, Debug, CandidType, Deserialize)]
struct HttpRequest {
    method: String,
    url: String,
    headers: Vec<(String, String)>,
    body: ByteBuf,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
struct HttpResponse {
    status_code: u16,
    headers: Vec<HeaderField>,
    body: Cow<'static, Bytes>,
    streaming_strategy: Option<StreamingStrategy>,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
enum StreamingStrategy {
    Callback { callback: Func, token: Token },
}

#[derive(Clone, Debug, CandidType, Deserialize)]
struct Token {}

struct DonorToClientMap {
    map: BTreeMap<Donor, Client>,
}

impl DonorToClientMap {
    pub fn new() -> Self {
        Self {
            map: BTreeMap::new(),
        }
    }

    pub fn get(&self, donor: &Donor) -> Option<&Client> {
        self.map.get(&donor)
    }

    pub fn insert(&mut self, donor: Donor, client: Client) {
        self.map.insert(donor, client);
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }
}

#[derive(Clone, Debug, CandidType, Deserialize)]
struct Client {
    pub client_canister_id: Principal,
}

#[derive(Clone, Debug, PartialOrd, Ord, Eq, PartialEq, Deserialize)]
struct Donor {
    pub donor: Principal,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
struct ApprovedClient {
    pub client_canister_id: Principal,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
struct DonationReceiver {
    pub receiver: Principal,
    pub beneficiaries: Vec<Principal>,
}

#[query]
fn print() {
    ic_cdk::print("Hello World from DFINITY!");
}

#[update]
fn register_client(client: Client) -> u128 {
    trap_if_caller_not_authenticated();

    let donor = Donor {
        donor: ic_cdk::caller(),
    };
    ic_cdk::print(format!(
        "Registering client {:?} for donor {:?}",
        client, donor
    ));
    STATE.with(|s| {
        let mut donor_to_client_map = s.donor_to_client_map.borrow_mut();
        donor_to_client_map.insert(donor, client);
        donor_to_client_map.len() as u128
    })
}

#[update]
fn approve_client(client: ApprovedClient) {
    ic_cdk::print(format!("Called approve_client for {:?}", client));
}

// Returns the `Client` that the self-authenticating principal issuing this call
// is currently mapped to. Returns `None` if there is no mapping.
//
// Authentication: the call must be made using a self-authenticating principal
//
// Traps if the caller is anonymous.
#[query]
fn current_client() -> Option<Client> {
    trap_if_caller_not_authenticated();

    let donor = Donor {
        donor: ic_cdk::caller(),
    };

    STATE.with(|state| state.donor_to_client_map.borrow().get(&donor).cloned())
}


fn trap_if_caller_not_authenticated() {
    let caller = ic_cdk::caller();
    let blob = caller.as_slice();
    if blob.len() != 28 + 1 {
        ic_cdk::trap(&format!("{} could not be authenticated.", caller));
    }
    if blob.last() != Some(&0x02) {
        ic_cdk::trap(&format!("{} could not be authenticated.", caller));
    }
}

// Issues an inter canister call to the `Client` that is registered for the
// self-authenticating principal issuing this call. The mapping to the client
// must be active.
//
// Authentication: the call must be made using a self-authenticating principal
//
// Traps if the caller is anonymous.
#[update]
async fn donate(receiver: DonationReceiver) {
    ic_cdk::print(format!("Called donate for {:?}", receiver));
    trap_if_caller_not_authenticated();
    let donor_principal = Donor {
        donor: ic_cdk::caller(),
    };
    let donor_client = STATE.with(|s| {
        s.donor_to_client_map
            .borrow()
            .get(&donor_principal)
            .cloned()
    });
    if let Some(client) = donor_client {
        let result: CallResult<()> =
            ic_cdk::call(client.client_canister_id, "donate", (receiver,)).await;
        match result {
            Err(e) => ic_cdk::trap(&format!(
                "Call to client {:?} was not successful: {:?}",
                client.client_canister_id, e
            )),
            Ok(_) => {}
        }
    } else {
        // TODO: change this to an error so that we can handle this case in the UI
        ic_cdk::trap(&format!("Found no mapping for this donor."))
    }
}

/// This makes this Candid service self-describing, so that for example Candid UI, but also other
/// tools, can seamlessly integrate with it. The concrete interface (method name etc.) is
/// provisional, but works.
///
/// This is needed by the ic-repl tests, see
/// https://github.com/chenyan2002/ic-repl#notes-for-rust-canisters
#[query]
fn __get_candid_interface_tmp_hack() -> String {
    include_str!("../index_rs.did").to_string()
}

#[query]
fn http_request(req: HttpRequest) -> HttpResponse {
    let parts: Vec<&str> = req.url.split('?').collect();
    match parts[0] {
        probably_an_asset => {
            let certificate_header = STATE.with(|s| {
                make_asset_certificate_header(
                    &s.asset_hashes.borrow(),
                    probably_an_asset,
                )
            });

            ASSETS.with(|a| match a.borrow().get(probably_an_asset) {
                Some((headers, value)) => {
                    let mut headers = headers.clone();
                    headers.push(certificate_header);

                    HttpResponse {
                        status_code: 200,
                        headers,
                        body: Cow::Borrowed(Bytes::new(value)),
                        streaming_strategy: None,
                    }
                }
                None => HttpResponse {
                    status_code: 404,
                    headers: vec![certificate_header],
                    body: Cow::Owned(ByteBuf::from(format!(
                        "Asset {} not found.",
                        probably_an_asset
                    ))),
                    streaming_strategy: None,
                },
            })
        }
    }
}

fn make_asset_certificate_header(
    asset_hashes: &AssetHashes,
    asset_name: &str,
) -> (String, String) {
    let certificate = data_certificate().unwrap_or_else(|| {
        trap("data certificate is only available in query calls");
    });
    let witness = asset_hashes.witness(asset_name.as_bytes());
    let tree = hashtree::labeled(LABEL_ASSETS, witness);
    let mut serializer = serde_cbor::ser::Serializer::new(vec![]);
    serializer.self_describe().unwrap();
    tree.serialize(&mut serializer)
        .unwrap_or_else(|e| trap(&format!("failed to serialize a hash tree: {}", e)));
    (
        "IC-Certificate".to_string(),
        format!(
            "certificate=:{}:, tree=:{}:",
            base64::encode(&certificate),
            base64::encode(&serializer.into_inner())
        ),
    )
}

#[post_upgrade]
fn retrieve_data() {
    init_assets();
    STATE.with(|s| {
        update_root_hash(&s.asset_hashes.borrow());
    });
}

#[init]
fn init() {
    init_assets();
    STATE.with(|state| {
        update_root_hash(&state.asset_hashes.borrow());
    });
}

// used both in init and post_upgrade
fn init_assets() {
    use assets::ContentEncoding;

    STATE.with(|s| {
        let mut asset_hashes = s.asset_hashes.borrow_mut();

        ASSETS.with(|a| {
            let mut assets = a.borrow_mut();
            assets::for_each_asset(|name, encoding, contents, hash| {
                asset_hashes.insert(name, *hash);
                let headers = match encoding {
                    ContentEncoding::Identity => vec![],
                    ContentEncoding::GZip => {
                        vec![("Content-Encoding".to_string(), "gzip".to_string())]
                    }
                };
                assets.insert(name, (headers, contents));
            });
        });
    });
}

fn update_root_hash(a: &AssetHashes) {
    use hashtree::labeled_hash;

    let prefixed_root_hash = labeled_hash(LABEL_ASSETS, &a.root_hash());
    set_certified_data(&prefixed_root_hash[..]);
}