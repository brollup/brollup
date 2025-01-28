# Transaction Outputs
`Bitcoin Virtual Machine` employs of nine types of transaction outputs (TXOs):

| TXO Type               | Kind           |  Spending Condition                                        |
|:-----------------------|:---------------|:-----------------------------------------------------------|
| Lift 🛗                | Bare           | `(Self + Operator) or (Self after 3 months)`               | 
| VTXO 💵                | Virtual        | `(Self + Operator) or (Self after 3 months)`               |
| VTXO Projector 🎥      | Bare           | `(msg.senders[] + Operator) or (Operator after 3 months)`  |
| Channel 👥             | Virtual        | `(Self + Operator) after degrading timelock`               |
| Connector 🔌           | Virtual        | `(Self + Operator)`                                        |
| Connector Projector 🎥 | Bare           | `(msg.senders[] + Operator) or (Operator after 3 months)`  |
| Payload 📦             | Bare           | `(msg.senders[] after 1 week) or (Operator with hashlocks)`|
| Self 👨‍💻                | Bare & Virtual | `(Self)`                                                   |
| Operator 🏭            | Bare & Virtual | `(Operator)`                                               |

Four of the transaction output types are bare, meaning they are literal, on-chain transaction outputs that consume block space, while the other five are virtual, meaning they are committed but not yet revealed transaction outputs that optimistically consume no block space.

The `Bitcoin Virtual Machine` advances the rollup state by chaining `Pool Transactions` at regular intervals. Three output types—`VTXO Projector`, `Connector Projector`, and `Payload`—and optionally one or more `Bare Connectors` are contained in the `Pool Transaction`.

                                                                              ⋰
                                                                            ⋰  ┌────────────────┐   ┌────────────────┐
                                                                          ⋰    │     VTXO #0    │-->│   Channel #0   │ 
                 Prevouts                       Outs                    ⋰      └────────────────┘   └────────────────┘
           ┌───────────────────┐      ┌─────────────────────┐         ⋰                 ┊                   
        #0 │      Payload      │   #0 │       Payload       │       ⋰          ┌────────────────┐   ┌────────────────┐
           └───────────────────┘      └─────────────────────┘     ⋰            │     VTXO #y    │-->│   Channel #y   │ 
           ┌───────────────────┐      ┌─────────────────────┐   ⋰              └────────────────┘   └────────────────┘
        #1 │      Lift #0      │   #1 │    VTXO Projector   │ 🎥 ┈ ┈ ┈ ┈ ┈ ┈ ┈ ┈      
           └───────────────────┘      └─────────────────────┘         
                     ┊                ┌─────────────────────┐                          
           ┌───────────────────┐   #2 │ Connector Projector │ 🎥 ┈ ┈ ┈ ┈ ┈ ┈ ┈ ┈            
      #1+n │      Lift #n      │      └─────────────────────┘   ⋱              ┌────────────────┐  
           └───────────────────┘      ┌─────────────────────┐     ⋱            │  Connector #0  │       
                     ┊             #3 │       Lift #0       │       ⋱          └────────────────┘
                                      └─────────────────────┘         ⋱                 ┊
                                                 ┊                      ⋱      ┌────────────────┐   
                                      ┌─────────────────────┐             ⋱    │  Connector #z  │
                                 #x+3 │       Lift #x       │               ⋱  └────────────────┘
                                      └─────────────────────┘                 ⋱                    
      
                         Pool Transaction     

## Lift 🛗
`Lift` is a bare, on-chain transaction output type used for onboarding to the `Bitcoin VM`. When a `Lift` output is funded, it can be swapped out for a 1:1 `VTXO` in a process known as lifting. In short, `Lift` lifts itself up to a `VTXO`.

`Lift` carries two  spending conditions:
`(Self + Operator) or (Self after 3 months)`

-  **Lift Path:** `Self` and `Operator` sign from the collaborative path `(Self + Operator)` to swap the `Lift` output in exchange for a 1:1 `VTXO`. `Self` swaps out the `Lift` output with the provided `Bare Connector` to receive a `VTXO` in return.
    
-   **Exit Path:** In case the `Operator` is non-collaborative and does not sign from the collaborative path, `Self` can trigger the exit path `(Self after 3 months)` to reclaim their funds.

### External Funding
`Lift` is an on-chain P2TR address, so it can be funded by a third-party wallet, such as an exchange, a payroll service, or an individual. When a `Lift` output is funded by an external source, it must receive at least two on-chain confirmations to be considered valid.
                                                            
                                Prevouts                       Outs    
                         ┌────────────────────┐       ┌────────────────────┐  
                     #0  │     Third Party    │   #0  │         ...        │     
                         └────────────────────┘       └────────────────────┘
                                    ┊                            ┊
                         ┌────────────────────┐       ┌────────────────────┐ 
                     #x  │     Third Party    │   #y  │        Lift        │--->Pool Transaction
                         └────────────────────┘       └────────────────────┘                    
                                                                 ┊
                                                      ┌────────────────────┐
                                                  #z  │         ...        │
                                                      └────────────────────┘
      
                               A Third Party Payout Transaction 

### Internal Funding
`Lift` is can also be funded internally, within a pool transaction. When a `Lift` output is funded internally it can be spent in another pool transaction immediately.
                                                            
                                Prevouts                       Outs    

                                   ┊                            ┊ 
                                                      ┌────────────────────┐ 
                                                  #3  │       Lift #0      │--->Pool Transaction
                                                      └────────────────────┘                    
                                                                 ┊
                                                      ┌────────────────────┐
                                                #x+3  │       Lift #x      │--->Pool Transaction
                                                      └────────────────────┘
      
                                       Pool Transaction 

## VTXO 💵
`VTXO` is a virtual, off-chain transaction output that holds the `Self` funds. `VTXOs` are projected by the `VTXO Projector` and can be unilaterally redeemed on-chain. A `VTXO` expires three months after its creation, or, in other words, three months after its projector `VTXO Projector` hits on-chain. 

Once a `VTXO` expires, it can no longer be redeemed or claimed on-chain; therefore, `Self` must either spend them entirely or refresh the `VTXOs` into new ones on a monthly basis. It is the client software's burden to abstract the refresh UX away for `Self`. At the protocol level, however, refreshes are interpreted differently from regular transfers, and the `Operator` is not allowed to charge liquidity fees when `VTXOs` are refreshed.

`VTXO` carries two spending conditions:
`(Self + Operator) or (Self after 3 month)`

-   **Channel Path:** `Self` and `Operator` sign from the channel path `(Self + Operator)` to establish a `Channel` from which they can sign state updates to send and receive payments.
    
-   **Exit Path:** In case the `Operator` is non-collaborative and does not sign from the channel path, `Self` can trigger the exit path `(Self after 3 month)` to unilaterally claim the `VTXO`.

## VTXO Projector 🎥
`VTXO Projector` is a bare, on-chain transaction output type contained in each pool transaction. `VTXO Projector` projects `VTXOs` into a covenant template.
                                                      
                                           ⋰ ┌──────────────────┐
                                         ⋰   │      VTXO #0     │
                                       ⋰     └──────────────────┘
                                     ⋰       ┌──────────────────┐
                                   ⋰         │      VTXO #1     │
        ┌──────────────────┐     ⋰           └──────────────────┘
        │  VTXO Projector  │ 🎥 ⋮                        
        └──────────────────┘     ⋱                    ┊
                                   ⋱                
                                     ⋱       ┌──────────────────┐
                                       ⋱     │      VTXO #n     │
                                         ⋱   └──────────────────┘
                                           ⋱

`VTXO Projector` carries two spending conditions:
`(msg.senders[] + Operator) or (Operator after 3 months)`

-   **Reveal Path:** The aggregated [MuSig2](https://github.com/bitcoin/bips/blob/master/bip-0327.mediawiki) key of msg.senders[] and `Operator` pre-sign from the reveal path `(msg.senders[] + Operator)` to constrain `VTXOs` in a pseudo-covenant manner.
    
-  **Sweep Path:** `VTXO Projector` expires in three months, at which point all `VTXOs` contained within the projector also expire. Upon expiry, the `Operator` triggers the sweep path `(Operator after 3 months)` to reclaim all expired `VTXOs` directly from the projector root, in a footprint-minimal way, without claiming `VTXOs` one by one.          

## Channel 👥
`Channel` turns its parent `VTXO` into a state channel that operates as a 2-of-2 between `Self` and `Remote`. 

`Self` and `Remote` sign from the 2-of-2 `(Self + Operator)` to update the channel state, where each state update overwrites the previous one with higher precedence. `Channel` uses a degrading relative timelock scheme to ensure that each new channel state takes precedence over the previous one and therefore overwrites the older state. By design `Bitcoin VM` employs 128 degrading periods.

`Channel` is a TapTree with 128 leaves, where each TapLead corresponds to a degrading period. Each period is a 2-of-2 between `Self` and `Operator` with a relative timelock, where the duration starts at 141 days and degrades by one with each subsequent period.

                                                    ┌───────────────────┐
    -Lv 7                                           │  Channel TapRoot  │                                  
                                                    └───────────────────┘       
                                                 ⋰                        ⋱
    -Lv 2..6                                   ⋰                            ⋱         
                                             ⋰                                ⋱
                         ┌───────────────────┐                                  ┌───────────────────┐
    -Lv 1                │    TapBranch 1    │                ┄                 │    TapBranch 64   │  
                         └───────────────────┘                                  └───────────────────┘
                      ┌─────┘             └─────┐                            ┌─────┘             └─────┐
             ┌─────────────────┐      ┌─────────────────┐           ┌─────────────────┐      ┌─────────────────┐
             │    TapLeaf 1    │      │     TapLeaf 2   │           │   TapLeaf 127   │      │   TapLeaf 128   │
    -Lv 0    │(Self + Operator)│      │(Self + Operator)│     ┄     │(Self + Operator)│      │(Self + Operator)│
             │  After 141 days │      │  After 140 days │           │  After 15 days  │      │  After 14 days  │
             └─────────────────┘      └─────────────────┘           └─────────────────┘      └─────────────────┘
                     ⬇                         ⬇                           ⬇                        ⬇
          ┌──────────┐┌──────────┐  ┌──────────┐┌──────────┐     ┌──────────┐┌──────────┐  ┌──────────┐┌──────────┐
          │  State 1 ││  State 1 │  │  State 2 ││  State 2 │  ┄  │ State 127││ State 127│  │ State 128││ State 128│
          │  (Self)  ││(Operator)│  │  (Self)  ││(Operator)│     │  (Self)  ││(Operator)│  │  (Self)  ││(Operator)│
          └──────────┘└──────────┘  └──────────┘└──────────┘     └──────────┘└──────────┘  └──────────┘└──────────┘
       
`Channel` completes its lifetime either when its parent `VTXO` expires (in three months) or after 128 state transitions have occurred. When `Channel`completes its lifetime, `Self` refreshes its parent `VTXO` into a new one and establishes a fresh `Channel` from there.

In contrast to the state channel design employed by Lightning Network, `Channel` has:
-  **No revocation:** Each new channel state overwrites the previous one with higher precedence.
-  **No basepoints:** It’s always the same key for `Self` and `Operator`. Keys are re-used without involving any point tweaking.
-  **No assymetry:** Channel state is symmetric, reproducible, and always descend from the channel root.
-  **No middle-stages:** No in-flight HTLCs or PTLCs. It is always about `Self` and `Operator`. Payments are linked by connectors.

## Connector 🔌
`Connector` is a virtual, off-chain transaction output type used for updating `Channel` states. `Connector` is a 2-of-2 `(Self + Operator)` between `Self` and the `Operator`, and carries dust a value of `450 sats`. A series of `Connectors` can be included in a `Connector Projector` and provided to `Self` by the `Operator`.                          
                                                            
                                Prevouts                        Outs          
                         ┌─────────────────────┐       ┌─────────────────────┐ 
                     #0  │       Channel       │   #0  │        Self         │
                         └─────────────────────┘       └─────────────────────┘                    
      From Connector     ┌─────────────────────┐       ┌─────────────────────┐ 
      Projector ---- #1->│      Connector      │   #1  │       Operator      │
                         └─────────────────────┘       └─────────────────────┘
      
                                       Channel State Update 

## Connector Projector 🎥
`Connector Projector` is the same as `VTXO Projector`, but for `Connectors` instead. `Connector Projector` is a bare, on-chain transaction output type contained in each pool transaction, and projects `Connectors` into a covenant template.
                                                      
                                                ⋰ ┌──────────────────┐
                                              ⋰   │   Connector #0   │
                                            ⋰     └──────────────────┘
                                          ⋰       ┌──────────────────┐
                                        ⋰         │   Connector #1   │
        ┌───────────────────────┐     ⋰           └──────────────────┘
        │  Connector Projector  │ 🎥 ⋮                        
        └───────────────────────┘     ⋱                    ┊
                                        ⋱                
                                          ⋱       ┌──────────────────┐
                                            ⋱     │   Connector #n   │
                                              ⋱   └──────────────────┘
                                                ⋱

`Connector Projector` carries two spending conditions:
`(msg.senders[] + Operator) or (Operator after 3 months)`

-   **Reveal Path:** The aggregated [MuSig2](https://github.com/bitcoin/bips/blob/master/bip-0327.mediawiki) key of msg.senders[] and `Operator` pre-sign from the reveal path `(msg.senders[] + Operator)` to constrain `Virtual Connectors` in a pseudo-covenant manner.

-  **Sweep Path:** `Connector Projector` expires in three months, at which point all `Virtual Connectors` contained within the projector also expire. Upon expiry, the `Operator` triggers the sweep path `(Operator after 3 months)` to reclaim all expired `Virtual Connectors` directly from the projector root, in a footprint-minimal way, without claiming `Virtual Connectors` one by one.          

## Payload 📦
`Payload` is a bare, on-chain transaction output type contained in each pool transaction.  `Payload` stores entries, projector signatures, s commitments, and the fresh operator key of the session.

## Self 👨‍💻
`Self` is a virtual P2TR output containing the self inner-key with no script-path involved.

## Operator 🏭
`Operator` is a virtual P2TR output containing the operator inner-key with no script-path involved.