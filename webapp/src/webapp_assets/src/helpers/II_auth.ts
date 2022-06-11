import { Actor, HttpAgent, ActorSubclass, HttpAgentOptions } from "@dfinity/agent";
import { AuthClient } from "@dfinity/auth-client";
import { idlFactory, canisterId } from "../../../declarations/webapp/index";
import type { _SERVICE } from "../../../declarations/webapp/webapp.did";
  

export const getBackendActor = (canisterId: string, options) => {
  const agent = new HttpAgent({ ...options?.agentOptions });

  if (process.env.NODE_ENV !== "production")
  {
    agent.fetchRootKey().catch(err => {
      console.warn("Unable to fetch root key.");
      console.error(err);
    });
  }

  return Actor.createActor(idlFactory, {
    agent,
    canisterId,
    ...options?.agentOptions
  });
};

  // process.env.CANISTER_ID = canisterId;
  
  // let agentOptions = {};
  // if (process.env.NODE_ENV !== "production") {
  //   agentOptions = { ...agentOptions, host: "http://127.0.0.1:8000" };
  // }
  
  // export async function getBackendActor(
  //   authClient: AuthClient | null
  // ): Promise<ActorSubclass<_SERVICE>> {
  //   if (authClient instanceof AuthClient) {
  //     const identity = authClient.getIdentity();
  //     agentOptions = { ...agentOptions, identity: identity as any };
  //   }
  //   const agent = new HttpAgent(agentOptions);
  //   // for local development only, this must not be used for production
  //   if (process.env.NODE_ENV === "development") {
  //     await agent.fetchRootKey();
  //   }
  
  //   const backend = Actor.createActor<_SERVICE>(idlFactory, {
  //     agent,
  //     canisterId: process.env.INTERNET_IDENTITY_CANISTER_ID,
  //   });
  
  //   return backend;
  // }