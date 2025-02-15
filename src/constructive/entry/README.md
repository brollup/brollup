# Entries
`Brollup` employs of 5 types of entries:

| Entry Type       |  Description                                                                                  |
|:-----------------|:----------------------------------------------------------------------------------------------|
| Liftup ⬆️        | Turns `Lift` into a `VTXO`.                                                                   |
| Recharge 🔋      | Refreshes `Channel` liquidity into a fresh, new `VTXO`.                                       |
| Transfer 💸      | Transfers sats.                                                                               |
| Call 📡          | Calls a smart contract. This may internally involve `Transfer`.                               |
| Reserved 📁      | Fails the entry. Reserved for future upgrades.                                                |