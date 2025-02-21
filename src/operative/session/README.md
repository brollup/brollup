## Session
Brollup session protocol for covenant emulation & forfeiting.

    +------------+                                    +-------------+                               +-------------+ 
    |            |                                    |             |                               |             |
    |            |--(1)--          Commit          -->|             |                               |             |
    |            |                                    |             |--(2)--    StateSigsAsk     -->|             |
    |            |                                    |             |<-(3)--      StateSigs      ---|             | 
    |            |                                    |             |                               |             |
    |            |<-(4a)-  CommitErr or CommitAck  ---|             |--(4b)-    CovSigsAsk       -->|             |
    |            |--(5a)-         Uphold           -->|             |<-(5b)-      CovSigs        ---|             |
    |    Node    |                                    | Coordinator |                               |   Operator  |
    |            |<-(6)--  UpholdErr or UpholdAck  ---|             |                               |             |
    |            |--(7)--          Forfeit         -->|             |                               |             |
    |            |                                    |             |--(8)--    AdvanceSigAsk    -->|             |
    |            |                                    |             |<-(9)--     AdvanceSig      ---|             | 
    |            |<-(10)-- ForfeitErr or ForfeitAck --|             |                               |             |
    |            |                                    |             |                               |             |
    +------------+                                    +-------------+                               +-------------+ 