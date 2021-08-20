export const idlFactory = ({ IDL }) => {
  return IDL.Service({
    'accept_cycles' : IDL.Func([IDL.Opt(IDL.Principal)], [], ['oneway']),
    'balance' : IDL.Func([], [IDL.Nat], []),
  });
};
export const init = ({ IDL }) => { return []; };
