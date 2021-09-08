use certified_map::{AsHashTree, RbTree};
use hashtree::Hash;
use ic_cdk::api::call::call_with_payment;
use ic_cdk::api::call::CallResult;
use ic_cdk::api::{data_certificate, set_certified_data};
use ic_cdk::export::candid::Func;
use ic_cdk::export::{
    candid::{CandidType, Deserialize},
    Principal,
};
use ic_cdk::trap;
use ic_cdk_macros::*;
use serde::Serialize;
use serde_bytes::{ByteBuf, Bytes};
use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap};
use std::convert::TryFrom;

mod assets;

const LABEL_ASSETS: &[u8] = b"http_assets";

thread_local! {
    static STATE: State = State {
        daily_budget: RefCell::new(Cycles(100)),
        receivers: RefCell::new(vec![]),
        asset_hashes: RefCell::new(AssetHashes::default()),
    };
    static ASSETS: RefCell<HashMap<&'static str, (Vec<HeaderField>, &'static [u8])>> = RefCell::new(HashMap::default());
}

struct State {
    daily_budget: RefCell<Cycles>,
    receivers: RefCell<Vec<DonationReceiver>>,
    asset_hashes: RefCell<AssetHashes>,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
struct Cycles(pub u128);

#[derive(Clone, Debug, CandidType, Deserialize)]
struct DonationReceiver {
    pub receiver: Principal,
    pub beneficiaries: Vec<Principal>,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
struct PendingDonations {
    pending: Vec<PendingDonation>,
    amount: Cycles,  // for the UI to display
    balance: Cycles, // for the UI to display
}

#[derive(Clone, Debug, CandidType, Deserialize)]
struct PendingDonation {
    // the receiver of the donation
    receiver: Principal,
    // the number of button clicks for this receiver
    count: u32,
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

// Accepts a donation request from the index canister.
//
// Authentication: The call must be made by the index canister, or the canister owner
#[update]
async fn donate(receiver: DonationReceiver) {
    // TODO: Check this call is coming from the index
    STATE.with(|state| {
        let mut receivers = state.receivers.borrow_mut();
        receivers.push(receiver);
    });
}

// Pay out the pending donations. This will distribute the DonationAmount of
// cycles set using `set_donation_amount` according to `PendingDonations`.
// The amount of paid out cylces are weighted according to the counts in
// `PendingDonation`.
//
// TODO: maybe return an error if the client canister does not have enough
// cycles?
//
// Authentication: the call must be made using a the self-authenticating
// principal that was used for `register`.
//
// Traps if the caller is not the registered one, also if it is anonymous.
#[update]
async fn approve_donations() {
    ////////////////////////////////
    // TODO: add authentication!!!
    ///////////////////////////////

    let (count_receivers, total_num_clicks, total_amount_to_spend) = STATE.with(|state| {
        let count_receivers = calculate_receiver_counts(&state.receivers.borrow());
        let total_num_clicks = state.receivers.borrow().len();
        let total_amount_to_spend = state.daily_budget.borrow();
        (
            count_receivers.clone(),
            total_num_clicks,
            total_amount_to_spend.clone(),
        )
    });

    let canister_balance: u64 = ic_cdk::api::canister_balance();
    let total_amount_to_spend_u64 = u64::try_from(total_amount_to_spend.0).unwrap_or_else(|_| {
        ic_cdk::trap("Converting total amount to spend to u64 failed.");
    });
    if canister_balance < total_amount_to_spend_u64 {
        ic_cdk::trap(&format!(
            "Current canister balance ({}) not sufficient to pay out the total amount ({:?})",
            canister_balance, total_amount_to_spend_u64
        ))
    }

    for (receiver, count) in count_receivers {
        let amount_to_send_to_receiver_u64 =
            (total_amount_to_spend_u64 / total_num_clicks as u64) * count as u64;

        let result: CallResult<()> = call_with_payment(
            receiver,
            "accept_cycles",
            (receiver,),
            amount_to_send_to_receiver_u64,
        )
        .await;
        match result {
            Err(e) => ic_cdk::print(&format!(
                "Call to client {:?} to accept_cycles failed: {:?}",
                receiver, e
            )),
            Ok(_) => {}
        }
    }

    // Now that the cycles have been sent, clear the receivers
    STATE.with(|state| {
        state.receivers.borrow_mut().clear();
    });
}

fn balance() -> Cycles {
    Cycles(ic_cdk::api::canister_balance() as u128)
}

#[query]
async fn list_donations() -> PendingDonations {
    STATE.with(|state| {
        let count_receivers = calculate_receiver_counts(&state.receivers.borrow());

        let pending = count_receivers
            .iter()
            .map(|(r, c)| PendingDonation {
                receiver: *r,
                count: *c,
            })
            .collect();

        PendingDonations {
            pending,
            amount: Cycles(0),
            balance: balance(),
        }
    })
}

fn calculate_receiver_counts(receivers: &Vec<DonationReceiver>) -> BTreeMap<Principal, u32> {
    let mut count_receivers: BTreeMap<Principal, u32> = BTreeMap::new();

    for receiver in receivers.iter() {
        let principal = receiver.receiver;
        count_receivers
            .entry(principal)
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }
    count_receivers
}

#[update]
async fn set_donation_amount(amount: Cycles) {
    STATE.with(|state| {
        state.daily_budget.replace(amount);
    })
}

/// This makes this Candid service self-describing, so that for example Candid UI, but also other
/// tools, can seamlessly integrate with it. The concrete interface (method name etc.) is
/// provisional, but works.
///
/// This is needed by the ic-repl tests, see
/// https://github.com/chenyan2002/ic-repl#notes-for-rust-canisters
#[query]
fn __get_candid_interface_tmp_hack() -> String {
    include_str!("../client_rs.did").to_string()
}

#[query]
fn http_request(req: HttpRequest) -> HttpResponse {
    let parts: Vec<&str> = req.url.split('?').collect();
    match parts[0] {
        probably_an_asset => {
            let certificate_header = STATE.with(|s| {
                make_asset_certificate_header(&s.asset_hashes.borrow(), probably_an_asset)
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

fn make_asset_certificate_header(asset_hashes: &AssetHashes, asset_name: &str) -> (String, String) {
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
