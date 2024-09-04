# Value Types
`Brollup` employs of 8+ value types:

| Value Type             |  Description                                                            |
|:-----------------------|:------------------------------------------------------------------------|
| Short Val              | Succinct value representation for integers: UInt8-16-24-32.             |
| Long Val               | Succinct value representation for integers: UInt8-16-24-32-40-48-56-64. |
| Account                | Possibly compact account representation.                                |
| Contract               | Possibly compact contract representation.                               |
| MaybeCommon Account    | Possibly common `Account` representation.                               |
| MaybeCommon Contract   | Possibly common `Contract` representation.                              |
| MaybeCommon Short Val  | Possibly common `Short Val` representation.                             |
| MaybeCommon Long Val   | Possibly common `Long Val` representation.                              |