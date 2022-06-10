export const idlFactory = ({ IDL }) => {
  const Controller = IDL.Service({ 'whoami' : IDL.Func([], [IDL.Text], []) });
  return Controller;
};
export const init = ({ IDL }) => { return [IDL.Opt(IDL.Principal)]; };
