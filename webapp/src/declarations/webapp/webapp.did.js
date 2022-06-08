export const idlFactory = ({ IDL }) => {
  return IDL.Service({
    'get_canister_principal' : IDL.Func([], [IDL.Principal], ['query']),
    'get_principal' : IDL.Func([], [IDL.Principal], ['query']),
  });
};
export const init = ({ IDL }) => { return []; };
