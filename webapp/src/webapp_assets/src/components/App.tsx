import React, { useEffect, useState, useCallback } from "react";

import { getBackendActor } from "../helpers/II_auth";
import { AuthClient } from "@dfinity/auth-client";

import { Actor, Identity } from "@dfinity/agent";

function App() {
  const [logged, setLogged] = useState(false);
  const [user, setUser] = useState("");
  const canisterId = process.env.WEBAPP_CANISTER_ID;

  const days = BigInt(1);
  const hours = BigInt(24);
  const nanoseconds = BigInt(3600000000000);

  const setIdentity = async (authClient: AuthClient) => {
    const identity = await authClient.getIdentity();
    const princ = await identity.getPrincipal();
    setUser(princ.toString());
  };

  const isAuthenticated = async () => {
    const authClient = await AuthClient.create();
    const isLogged = await authClient.isAuthenticated();
    setLogged(isLogged);
    setIdentity(authClient);
  };

  useEffect(() => {
    isAuthenticated();
    // if (logged) {
    //   console.log(user);
    // }
  });

  const handleLogin = async (e: React.FormEvent<HTMLButtonElement>) => {
    e.preventDefault();
    const authClient = await AuthClient.create();
    if (!logged) {
      await authClient.login({
        onSuccess: async () => {
          setIdentity(authClient);
          setLogged(true);
        },
        identityProvider:
          process.env.DFX_NETWORK === "ic"
            ? "https://identity.ic0.app/#authorize"
            : `http://${process.env.INTERNET_IDENTITY_CANISTER_ID}.localhost:8000`, // for some reason, it hated using just the canister. It needs to be a subdomain...
        maxTimeToLive: days * hours * nanoseconds,
      });
    } else {
      await authClient.logout();
      setLogged(false);
    }
  };
  return (
    <div>
      {!logged && (
        <div>
          <div>
            <div>
              <h2>Sign in to your account</h2>
            </div>
            <div>
              <button onClick={handleLogin}>
                Sign in with Internet Identity
              </button>
            </div>
          </div>
        </div>
      )}
      {logged && (
        <div>
          <button onClick={handleLogin}>logout</button>
          <p>{user}</p>
        </div>
      )}
    </div>
  );
}

export default App;
