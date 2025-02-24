## Session
Session protocol for the rollup state transition

    +------------+                                      +-------------+                                      +-------------+ 
    |            |                                      |             |                                      |             |
    |            |--(1)--          Commit            -->|             |                                      |             |
    |            |                                      |             |--(2)--          StateUp           -->|             |
    |            |                                      |             |<-(3)-- StateUpAck (or StateUpErr) ---|             | 
    |            |                                      |             |                                      |             |
    |            |<-(4a)-  CommitAck (or CommitErr)  ---|             |--(4b)-           OpCov            -->|             |
    |            |--(5a)-          Uphold            -->|             |<-(5b)-   OpCovAck (or OpCovErr)   ---|             |
    |    Node    |                                      | Coordinator |                                      |   Operator  |
    |            |<-(6)--  UpholdAck (or UpholdErr)  ---|             |                                      |             |
    |            |--(7)--          Forfeit           -->|             |                                      |             |
    |            |                                      |             |--(8)--          Advance           -->|             |
    |            |                                      |             |<-(9)-- AdvanceAck (or AdvanceErr) ---|             | 
    |            |<-(10)- ForfeitAck (or ForfeitErr) ---|             |                                      |             |
    |            |                                      |             |                                      |             |
    +------------+                                      +-------------+                                      +-------------+ 