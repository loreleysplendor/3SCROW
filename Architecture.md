# Describes The Architecture of The 3SCROW Platform

Escrow Controller:

- Creates the escrow canisters,
- Destroys escrow canisters once used,
- Includes end points to interact with escrow canisters from a single canister, forwarding the requests to it,

Escrow Canister:

- Manages escrows,
- Endpoints for authorising, depositing, withdrawing, cancelling and topping up,
- No front end,
- Consider adding external authorisations. Start with dates, so once a date strikes, the escrow is authorised automatically,
- Also consider adding a pre-determined escrow amount or else the escrow will not authorise.

Font-End Canister:

- Interacts with the escrow controller,
- Provides the front-end functionality for the escrow via the escrow controller,
- Allows people to create escrows, retrieve deposit address, withdraw funds, authorise escrow and add external authorisations,
- Displays the items currently within escrow,
