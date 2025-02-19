## Brollup Session
Session protocol for covenant emulation & forfeiting.

    +------------+                               +-------------+                               +-------------+ 
    |            |--(1)--   NSessionCommit    -->|             |                               |             |
    |            |<-(2)--  CSessionCommitAck  ---|             |                               |             |
    |            |                               |             |--(1)--   RequestStateSigs  -->|             |
    |            |                               |             |<-(2)--      StateSigs      ---|             | 
    |            |--(3)--   NSessionUphold    -->|             |                               |             |
    |    Node    |<-(4)--  CSessionUpholdAck  ---| Coordinator |                               |   Operator  |
    |            |                               |             |--(1)--   RequestCovSigs    -->|             |
    |            |                               |             |<-(2)--      CovSigs        ---|             | 
    |            |--(5)--   NSessionForfeit   -->|             |                               |             |
    |            |<-(6)--  CSessionForfeitAck ---|             |                               |             |
    |            |                               |             |--(1)--   RequestPoolSig    -->|             |
    |            |                               |             |<-(2)--      PoolSig        ---|             | 
    +------------+                               +-------------+                               +-------------+ 