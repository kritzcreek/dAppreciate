import type { Principal } from '@dfinity/principal';
export interface DAppr { 'name' : string, 'canisterId' : Principal }
export interface DApprClient {
  'acceptDapr' : (arg_0: DAppr) => Promise<undefined>,
}
export interface _SERVICE {
  'forwardDAppr' : (arg_0: DAppr, arg_1: Principal) => Promise<undefined>,
}
