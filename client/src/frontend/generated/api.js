export const idlFactory = ({ IDL }) => {
  const DonationReceiver = IDL.Record({
    'beneficiaries' : IDL.Vec(IDL.Principal),
    'receiver' : IDL.Principal,
  });
  const PendingDonation = IDL.Record({
    'count' : IDL.Nat,
    'receiver' : DonationReceiver,
  });
  const DonationAmount = IDL.Record({ 'amount' : IDL.Nat });
  const PendingDonations = IDL.Record({
    'balance' : IDL.Nat,
    'pending' : IDL.Vec(PendingDonation),
    'amount' : DonationAmount,
  });
  return IDL.Service({
    'approve_donations' : IDL.Func([], [], []),
    'donate' : IDL.Func([DonationReceiver], [], []),
    'list_donations' : IDL.Func([], [PendingDonations], ['query']),
    'register' : IDL.Func([], [], []),
    'set_donation_amount' : IDL.Func([DonationAmount], [], []),
  });
};
export const init = ({ IDL }) => { return []; };
