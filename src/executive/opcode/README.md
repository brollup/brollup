# Opcode Wishlist
Feel free to open PRs to add new opcodes.

## Memory

| Opcode         | Bytecode | Ops | Input                | Output                 | Description                                                                     |
|:---------------|:---------|:----|:---------------------|:-----------------------|:--------------------------------------------------------------------------------|
| MWRITE         | 0xb2     | 5   | x1 x2                | x1                     | Pops the memory key and value, and writes the value to the contract's memory.   |
| MREAD          | 0xb3     | 5   | x1                   | x1                     | Pops the memory key, and reads the value from the contract's memory.            |
| MFREE          | 0xb4     | 1   | x1                   | x1                     | Pops the memory key, and frees the key/value from the contract's memory.        |

## Storage

| Opcode         | Bytecode | Ops | Input                | Output                 | Description                                                                     |
|:---------------|:---------|:----|:---------------------|:-----------------------|:--------------------------------------------------------------------------------|
| SWRITE         | 0xb5     | 50  | x1 x2                | x1                     | Pops the storage key and value, and writes the value to the contract's storage. |
| SREAD          | 0xb6     | 50  | x1                   | x1                     | Pops the storage key, and reads the value from the contract's storage.          |
| SFREE          | 0xb7     | 1   | x1                   | x1                     | Pops the storage key, and frees the key/value from the contract's storage.      |

## Stack Element Manipulation

| Opcode   | Bytecode | Ops | Input             | Output       | Description                                                          |
|:---------|----------|:----|:------------------|:-------------|:---------------------------------------------------------------------|
| CAT      | 0x7e     | 10  | x1 x2             | out          | Concatenates two strings.                                            |
| SUBSTR   | 0x7f     | 10  | in begin size     | out          | Returns a section of a string.                                       |
| LEFT     | 0x80     | 10  | in size           | out          | Keeps only characters left of the specified point in a string.       |
| RIGHT    | 0x81     | 10  | in size           | out          | Keeps only characters right of the specified point in a string.      |
| SIZE     | 0x82     | 10  | in                | in size      | Pushes the string length of the top element of the stack (without popping it). |

## Stack Operations

| Opcode         | Ops | Bytecode | Input                   | Output                 | Description                                                                  |
|:---------------|:----|:---------|:------------------------|:-----------------------|:-----------------------------------------------------------------------------|
| IFDUP          | 1   | 0x73     | x                       | x / x x                | If the top stack value is not 0, duplicate it.                               |
| DEPTH          | 1   | 0x74     | Nothing                 | <Stack size>           | Puts the number of stack items onto the stack.                               |
| DROP           | 1   | 0x75     | x                       | Nothing                | Removes the top stack item.                                                  |
| DUP            | 1   | 0x76     | x                       | x x                    | Duplicates the top stack item.                                               |
| NIP            | 1   | 0x77     | x1 x2                   | x2                     | Removes the second-to-top stack item.                                        |
| OVER           | 1   | 0x78     | x1 x2                   | x1 x2 x1               | Copies the second-to-top stack item to the top.                              |
| PICK           | 1   | 0x79     | xn ... x2 x1 x0 <n>     | xn ... x2 x1 x0 xn     | The item n back in the stack is copied to the top.                           |
| ROLL           | 1   | 0x7a     | xn ... x2 x1 x0 <n>     | ... x2 x1 x0 xn        | The item n back in the stack is moved to the top.                            |
| ROT            | 1   | 0x7b     | x1 x2 x3                | x2 x3 x1               | The 3rd item down the stack is moved to the top.                             |
| SWAP           | 1   | 0x7c     | x1 x2                   | x2 x1                  | The top two items on the stack are swapped.                                  |
| TUCK           | 1   | 0x7d     | x1 x2                   | x2 x1 x2               | The item at the top of the stack is copied and inserted before the second-to-top item. |
| 2DROP          | 1   | 0x6d     | x1 x2                   | Nothing                | Removes the top two stack items.                                             |
| 2DUP           | 1   | 0x6e     | x1 x2                   | x1 x2 x1 x2            | Duplicates the top two stack items.                                          |
| 3DUP           | 1   | 0x6f     | x1 x2 x3                | x1 x2 x3 x1 x2 x3      | Duplicates the top three stack items.                                        |
| 2OVER          | 1   | 0x70     | x1 x2 x3 x4             | x1 x2 x3 x4 x1 x2      | Copies the pair of items two spaces back in the stack to the front.          |
| 2ROT           | 1   | 0x71     | x1 x2 x3 x4 x5 x6       | x3 x4 x5 x6 x1 x2      | The fifth and sixth items back are moved to the top of the stack.            |
| 2SWAP          | 1   | 0x72     | x1 x2 x3 x4             | x3 x4 x1 x2            | Swaps the top two pairs of items.                                            |
| TOALTSTACK     | 1   | 0xb0     | x1                      | (alt)x1                | Puts the input onto the top of the alt stack. Removes it from the main stack. |
| FROMALTSTACK   | 1   | 0xb1     | (alt)x1                 | x1                     | Puts the input onto the top of the main stack. Removes it from the alt stack. |

## Arithmetics

| Opcode         | Ops | Bytecode | Input          | Output                                  | Description                                                                  |
|:---------------|:----|:---------|:---------------|:----------------------------------------|:-----------------------------------------------------------------------------|
| ADD            | 3   | 0x93     | x1 x2          | (x1 + x2) True or x1 x2 False           | x1 is added to x2.                                                           |
| SUB            | 3   | 0x94     | x1 x2          | (x1 - x2) True or x1 x2 False           | x1 is subtracted from x2.                                                    |
| MUL            | 10  | 0x95     | x1 x2          | (x1 * x2) True or x1 x2 False           | x1 is multiplied by x2.                                                      |
| DIV            | 10  | 0x96     | x1 x2          | (x1 % x2) (x1 / x2) True or x1 x2 False | x1 is divided by x2.                                                         |
| ADDMOD         | 3   | 0x8d     | x1 x2          | (x1 + x2) % MAX::U256                   | x1 is added to x2 modulo MAX::U256.                                          |
| MULMOD         | 10  | 0x8e     | x1 x2          | (x1 * x2) % MAX::U256                   | x1 is multiplied by x2 modulo MAX::U256.                                     |




