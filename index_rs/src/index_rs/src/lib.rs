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
    pub donation_receiver_canister_id: Principal,
}

#[query]
fn print() {
    ic_cdk::print("Hello World from DFINITY!");
}

#[update]
fn register_client(client: Client) -> u128 {
    let donor = Donor {
        donor: ic_cdk::caller(),
    };
    ic_cdk::print(format!(
        "Registering client {:?} for donor {:?}",
        client, donor
    ));
    let mut map_len = 0;
    STATE.with(|s| {
        let mut donor_to_client_map = s.donor_to_client_map.borrow_mut();
        donor_to_client_map.insert(donor, client);
        map_len = donor_to_client_map.len();
    });
    map_len as u128
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

#[update]
fn donate(receiver: DonationReceiver) {
    ic_cdk::print(format!("Called donate for {:?}", receiver));
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
