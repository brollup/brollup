## Session
Brollup session protocol for covenant emulation & forfeiting.

    +------------+                               +-------------+                               +-------------+ 
    |            |                               |             |                               |             |
    |            |--(1)--   NSessionCommit    -->|             |                               |             |
    |            |                               |             |--(2)--   RequestStateSigs  -->|             |
    |            |                               |             |<-(3)--      StateSigs      ---|             | 
    |            |<-(4)--  CSessionCommitAck  ---|             |                               |             |
    |            |--(5)--   NSessionUphold    -->|             |                               |             |
    |    Node    |                               | Coordinator |--(6)--   RequestCovSigs    -->|   Operator  |
    |            |                               |             |<-(7)--      CovSigs        ---|             | 
    |            |<-(8)--  CSessionUpholdAck  ---|             |                               |             |
    |            |--(9)--   NSessionForfeit   -->|             |                               |             |
    |            |                               |             |--(10)--  RequestPoolSig    -->|             |
    |            |                               |             |<-(11)--     PoolSig        ---|             | 
    |            |<-(12)-- CSessionForfeitAck ---|             |                               |             |
    |            |                               |             |                               |             |
    +------------+                               +-------------+                               +-------------+ 