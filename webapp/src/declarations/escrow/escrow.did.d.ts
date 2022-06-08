import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export type Address = Principal;
export type Amount = bigint;
export type DepositErr = { 'TransferFailure' : null } |
  { 'BalanceLow' : null };
export type DepositReceipt = { 'Ok' : bigint } |
  { 'Err' : DepositErr };
export interface Escrow {
  'deposit' : ActorMethod<[TokenCanister], DepositReceipt>,
  'getDepositAddress' : ActorMethod<[], Array<number>>,
  'ledger' : ActorMethod<[], string>,
  'string_address' : ActorMethod<[], string>,
  'whoami' : ActorMethod<[], string>,
  'withdraw' : ActorMethod<[Amount, TokenCanister, Principal], WithdrawReceipt>,
}
export type TokenCanister = Principal;
export type WithdrawErr = { 'TransferFailure' : null } |
  { 'BalanceLow' : null };
export type WithdrawReceipt = { 'Ok' : bigint } |
  { 'Err' : WithdrawErr };
export interface _SERVICE extends Escrow {}
