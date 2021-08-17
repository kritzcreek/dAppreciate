export const idlFactory = ({ IDL }) => {
  const DAppr = IDL.Record({ 'name' : IDL.Text, 'canisterId' : IDL.Principal });
  const Today = IDL.Record({
    'balance' : IDL.Nat,
    'dailyBudget' : IDL.Nat,
    'dApprs' : IDL.Vec(DAppr),
  });
  const MyDAppr = IDL.Service({
    'acceptDapr' : IDL.Func([DAppr], [], ['oneway']),
    'today' : IDL.Func([], [Today], ['query']),
  });
  return MyDAppr;
};
export const init = ({ IDL }) => { return [IDL.Principal]; };
