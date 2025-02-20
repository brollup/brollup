## Session
Brollup session protocol for covenant emulation & forfeiting.

    +------------+                               +-------------+                               +-------------+ 
    |            |                               |             |                               |             |
    |            |--(1)--   NSessionCommit    -->|             |                               |             |
    |            |                               |             |--(2)--   RequestStateSigs  -->|             |
    |            |                               |             |<-(3)--      StateSigs      ---|             | 
    |            |                               |             |                               |             |
    |            |<-(4a)-  CSessionCommitAck  ---|             |--(4b)-   RequestCovSigs    -->|             |
    |            |--(5a)-   NSessionUphold    -->|             |<-(5b)-      CovSigs        ---|             |
    |    Node    |                               | Coordinator |                               |   Operator  |
    |            |<-(6)--  CSessionUpholdAck  ---|             |                               |             |
    |            |--(7)--   NSessionForfeit   -->|             |                               |             |
    |            |                               |             |--(8)--   RequestPoolSig    -->|             |
    |            |                               |             |<-(9)--      PoolSig        ---|             | 
    |            |<-(10)-- CSessionForfeitAck ---|             |                               |             |
    |            |                               |             |                               |             |
    +------------+                               +-------------+                               +-------------+ 