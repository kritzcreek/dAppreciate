export const idlFactory = ({ IDL }) => {
  const ApprovedClient = IDL.Record({ 'client_canister_id' : IDL.Principal });
  const Client = IDL.Record({ 'client_canister_id' : IDL.Principal });
  const DonationReceiver = IDL.Record({
    'beneficiaries' : IDL.Vec(IDL.Principal),
    'receiver' : IDL.Principal,
  });
  return IDL.Service({
    'approve_client' : IDL.Func([ApprovedClient], [], []),
    'current_client' : IDL.Func([], [IDL.Opt(Client)], ['query']),
    'donate' : IDL.Func([DonationReceiver], [], []),
    'print' : IDL.Func([], [], ['query']),
    'register_client' : IDL.Func([Client], [IDL.Nat], []),
  });
};
export const init = ({ IDL }) => { return []; };
