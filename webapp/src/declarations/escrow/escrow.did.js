export const idlFactory = ({ IDL }) => {
  const TokenCanister = IDL.Principal;
  const DepositErr = IDL.Variant({
    'TransferFailure' : IDL.Null,
    'BalanceLow' : IDL.Null,
  });
  const DepositReceipt = IDL.Variant({ 'Ok' : IDL.Nat, 'Err' : DepositErr });
  const Amount = IDL.Nat;
  const WithdrawErr = IDL.Variant({
    'TransferFailure' : IDL.Null,
    'BalanceLow' : IDL.Null,
  });
  const WithdrawReceipt = IDL.Variant({ 'Ok' : IDL.Nat, 'Err' : WithdrawErr });
  const Escrow = IDL.Service({
    'deposit' : IDL.Func([TokenCanister], [DepositReceipt], []),
    'getDepositAddress' : IDL.Func([], [IDL.Vec(IDL.Nat8)], []),
    'ledger' : IDL.Func([], [IDL.Text], []),
    'string_address' : IDL.Func([], [IDL.Text], []),
    'whoami' : IDL.Func([], [IDL.Text], []),
    'withdraw' : IDL.Func(
        [Amount, TokenCanister, IDL.Principal],
        [WithdrawReceipt],
        [],
      ),
  });
  return Escrow;
};
export const init = ({ IDL }) => { return [IDL.Opt(IDL.Principal)]; };
