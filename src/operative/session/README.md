## Session
Brollup session protocol for covenant emulation & forfeiting.

    +------------+                               +-------------+                               +-------------+ 
    |            |--(1)--   NSessionCommit    -->|             |                               |             |
    |            |<-(2)--  CSessionCommitAck  ---|             |                               |             |
    |            |                               |             |--(3)--   RequestStateSigs  -->|             |
    |            |                               |             |<-(4)--      StateSigs      ---|             | 
    |            |--(5)--   NSessionUphold    -->|             |                               |             |
    |    Node    |<-(6)--  CSessionUpholdAck  ---| Coordinator |                               |   Operator  |
    |            |                               |             |--(7)--   RequestCovSigs    -->|             |
    |            |                               |             |<-(8)--      CovSigs        ---|             | 
    |            |--(9)--   NSessionForfeit   -->|             |                               |             |
    |            |<-(10)-- CSessionForfeitAck ---|             |                               |             |
    |            |                               |             |--(11)--  RequestPoolSig    -->|             |
    |            |                               |             |<-(12)--     PoolSig        ---|             | 
    +------------+                               +-------------+                               +-------------+ 