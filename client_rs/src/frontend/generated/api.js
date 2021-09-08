export const idlFactory = ({ IDL }) => {
  const DonationReceiver = IDL.Record({
    'beneficiaries' : IDL.Vec(IDL.Principal),
    'receiver' : IDL.Principal,
  });
  const PendingDonation = IDL.Record({
    'count' : IDL.Nat32,
    'receiver' : IDL.Principal,
  });
  const PendingDonations = IDL.Record({
    'balance' : IDL.Nat,
    'pending' : IDL.Vec(PendingDonation),
    'amount' : IDL.Nat,
  });
  return IDL.Service({
    'approve_donations' : IDL.Func([], [], []),
    'donate' : IDL.Func([DonationReceiver], [], []),
    'list_donations' : IDL.Func([], [PendingDonations], ['query']),
    'register' : IDL.Func([], [], []),
    'set_donation_amount' : IDL.Func([IDL.Nat], [], []),
  });
};
export const init = ({ IDL }) => { return []; };
