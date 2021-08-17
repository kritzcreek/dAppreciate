export const idlFactory = ({ IDL }) => {
  const DAppr = IDL.Record({ 'name' : IDL.Text, 'canisterId' : IDL.Principal });
  const DApprClient = IDL.Service({ 'acceptDapr' : IDL.Func([DAppr], [], []) });
  return IDL.Service({
    'forwardDAppr' : IDL.Func([DAppr, DApprClient], [], []),
  });
};
export const init = ({ IDL }) => { return []; };
