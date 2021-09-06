use ic_cdk::export::{candid::{CandidType, Deserialize}, Principal};
use ic_cdk_macros::*;

#[derive(Clone, Debug, CandidType, Deserialize)]
struct Client {
    pub client_canister_id: Principal,
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
fn register_client(client: Client) {
    ic_cdk::print(format!("Called register_client for {:?}", client));
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
