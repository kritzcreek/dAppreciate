import type { Principal } from '@dfinity/principal';
export interface _SERVICE {
  'accept_cycles' : (arg_0: [] | [Principal]) => Promise<undefined>,
  'balance' : () => Promise<bigint>,
}
