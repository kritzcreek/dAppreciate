import type { Principal } from '@dfinity/principal';
export interface ApprovedClient { 'client_canister_id' : Principal }
export interface Client { 'client_canister_id' : Principal }
export interface DonationReceiver {
  'beneficiaries' : Array<Principal>,
  'receiver' : Principal,
}
export interface _SERVICE {
  'approve_client' : (arg_0: ApprovedClient) => Promise<undefined>,
  'current_client' : () => Promise<[] | [Client]>,
  'donate' : (arg_0: DonationReceiver) => Promise<undefined>,
  'print' : () => Promise<undefined>,
  'register_client' : (arg_0: Client) => Promise<bigint>,
}
