use ic_cdk::api::call::{CallResult, RejectionCode};
use ic_cdk::export::{
    candid::{CandidType, Deserialize},
    Principal,
};
use ic_cdk_macros::*;
use std::cell::RefCell;
use std::collections::BTreeMap;

thread_local! {
    static STATE: State = State {
        donor_to_client_map: RefCell::new(DonorToClientMap::new())
    }
}

struct State {
    donor_to_client_map: RefCell<DonorToClientMap>,
}

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
    // TODO: trap if the principal is not self-authenticating
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

#[query]
fn current_client() -> Option<Client> {
    ic_cdk::print("Called current_client");
    None
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
    trap_if_caller_anonymous();
    let donor_principal = Donor {
        donor: ic_cdk::caller(),
    };
    let donor_client = STATE.with(|s| {
        s.donor_to_client_map
            .borrow()
            .get(&donor_principal)
            .cloned()
    });
    if let Some(donor) = donor_client {
        let result: CallResult<()> =
            ic_cdk::call(donor.client_canister_id, "donate", (receiver,)).await;
        match result {
            Err(e) => ic_cdk::trap(&format!(
                "Call to client {:?} was not successful: {:?}",
                donor.client_canister_id, e
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

// Traps if the caller is anonymous.
fn trap_if_caller_anonymous() {
    if ic_cdk::api::caller() == Principal::anonymous() {
        ic_cdk::trap(&format!("Caller must not be anonymous."))
    }
}
