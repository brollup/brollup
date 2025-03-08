## Session
Session protocol for the rollup state transition.

    +----------+                                      +-------------+                                      +----------+ 
    |          |                                      |             |                                      |          |
    |          |--(1)--           Commit           -->|             |                                      |          |
    |          |<-(1a)-   (possibly CommitNack)    ---|             |                                      |          |
    |          |                                      |             |                                      |          |
    |          |          .. AWAITING COMMITS ..      |             |                                      |          |
    |          |                                      |             |--(2)--          StateUp           -->|          |
    |          |                                      |             |<-(3)--  StateUpAck or StateUpNack ---|          | 
    |          |                                      |             |                                      |          |
    |          |<-(6)-   CommitAck (or CommitNack) ---|             |--(4)-            OpCov            -->|          |
    |   Node   |--(7)-     Uphold (or UpholdErr)   -->| Coordinator |<-(5)-          OpCovAck           ---| Operator |
    |          |<-(7a)    (possibly UpholdINack)   ---|             |                                      |          |
    |          |                                      |             |                                      |          |
    |          |          .. AWAITING UPHOLDS ..      |             |                                      |          |
    |          |                                      |             |                                      |          |
    |          |<-(8)-- UpholdAck (or UpholdONack) ---|             |                                      |          |
    |          |--(9)--   Forfeit (or ForfeitErr)  -->|             |                                      |          |
    |          |                                      |             |--(10)--          Advance          -->|          |
    |          |                                      |             |<-(11)--(AdvanceAck or AdvanceNack)---|          | 
    |          |<-(12)- ForfeitAck (or ForfeitNack)---|             |                                      |          |
    |          |                                      |             |                                      |          |
    +----------+                                      +-------------+                                      +----------+ 