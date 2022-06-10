import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface Controller { 'whoami' : ActorMethod<[], string> }
export interface _SERVICE extends Controller {}
