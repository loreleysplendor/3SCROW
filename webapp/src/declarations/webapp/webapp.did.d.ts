import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface _SERVICE {
  'get_canister_principal' : ActorMethod<[], Principal>,
  'get_principal' : ActorMethod<[], Principal>,
}
