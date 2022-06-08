import React, { useEffect, useState, useCallback } from "react";

import { getBackendActor } from "../helpers/auth";
import { AuthClient } from "@dfinity/auth-client";

function App() {
  const [logged, setLogged] = useState(false);
  const [remember, setRemember] = useState(true);
  const [identity, setIdentity] = useState("");

  const handleRemember = () => {
    setRemember(!remember);
  };

  const isAuth = async () => {
    const authClient = await AuthClient.create();
    const isAuth = await authClient.isAuthenticated();
    if (isAuth) {
      setLogged(true);
    }
  };

  const whoami = async () => {
    const authClient = await AuthClient.create();
    const ba = await getBackendActor(authClient);
    const value = await ba.get_principal();
    setIdentity(value.toText());
  };

  useEffect(() => {
    isAuth();
    if (logged) {
      whoami();
    }
  });

  const handleLogin = async (e: React.FormEvent<HTMLButtonElement>) => {
    e.preventDefault();
    const daysToAdd = 1;
    const expiry = Date.now() + daysToAdd * 86400000;
    const authClient = await AuthClient.create();
    if (!logged) {
      await authClient.login({
        onSuccess: async () => {
          const ba = await getBackendActor(authClient);
          const principal = await ba.get_principal();
          const ident = principal.toText();
          setIdentity(ident);
          setLogged(true);
        },
        identityProvider:
          "http://localhost:8000?canisterId=" +
          process.env.INTERNET_IDENTITY_CANISTER_ID,
      });
    } else {
      await authClient.logout();
      setLogged(false);
    }
  };
  <button onClick={handleLogin}>Sign In</button>;
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
        </div>
      )}
    </div>
  );
}

export default App;
