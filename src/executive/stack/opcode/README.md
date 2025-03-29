# Opcode Wishlist
Feel free to open PRs to add new opcodes.

## Global State

| Opcode         | Bytecode | Input                      | Output                 | Description                                                                     |
|----------------|----------|----------------------------|------------------------|---------------------------------------------------------------------------------|
| SSTORE         | 0xb8     | x1 x2                      | Nothing                | Pops the storage key and value, and writes the value to the contract's storage. |
| SLOAD          | 0xb9     | x1                         | x1                     | Pops the storage key, and reads the value from the contract's storage.          |

## Memory

| Opcode         | Bytecode | Input                      | Output                 | Description                                                                   |
|----------------|----------|----------------------------|------------------------|-------------------------------------------------------------------------------|
| TOALTSTACK_1   | 0xb0     | x1                         | (alt)x1                | Puts the input onto the top of the alt stack. Removes it from the main stack. |
| FROMALTSTACK_1 | 0xb3     | (alt)x1                    | x1                     | Puts the input onto the top of the main stack. Removes it from the alt stack. |
| TOALTSTACK_2   | 0xb4     | x1                         | (alt)x1                | Puts the input onto the top of the alt stack. Removes it from the main stack. |
| FROMALTSTACK_2 | 0xb5     | (alt)x1                    | x1                     | Puts the input onto the top of the main stack. Removes it from the alt stack. |
| TOALTSTACK_3   | 0xb6     | x1                         | (alt)x1                | Puts the input onto the top of the alt stack. Removes it from the main stack. |
| FROMALTSTACK_3 | 0xb7     | (alt)x1                    | x1                     | Puts the input onto the top of the main stack. Removes it from the alt stack. |

## Stack Element Manipulation

| Opcode   | Bytecode | Input             | Output       | Description                                                          |
|----------|----------|-------------------|--------------|----------------------------------------------------------------------|
| CAT      | 0x7e     | x1 x2             | out          | Concatenates two strings.                                            |
| SUBSTR   | 0x7f     | in begin size     | out          | Returns a section of a string.                                       |
| LEFT     | 0x80     | in size           | out          | Keeps only characters left of the specified point in a string.       |
| RIGHT    | 0x81     | in size           | out          | Keeps only characters right of the specified point in a string.      |
| SIZE     | 0x82     | in                | in size      | Pushes the string length of the top element of the stack (without popping it). |

## Stack Operations

| Opcode         | Bytecode | Input                      | Output                 | Description                                                                  |
|----------------|----------|----------------------------|------------------------|------------------------------------------------------------------------------|
| IFDUP          | 0x73     | x                          | x / x x                | If the top stack value is not 0, duplicate it.                               |
| DEPTH          | 0x74     | Nothing                    | <Stack size>           | Puts the number of stack items onto the stack.                               |
| DROP           | 0x75     | x                          | Nothing                | Removes the top stack item.                                                  |
| DUP            | 0x76     | x                          | x x                    | Duplicates the top stack item.                                               |
| NIP            | 0x77     | x1 x2                      | x2                     | Removes the second-to-top stack item.                                        |
| OVER           | 0x78     | x1 x2                      | x1 x2 x1               | Copies the second-to-top stack item to the top.                              |
| PICK           | 0x79     | xn ... x2 x1 x0 <n>        | xn ... x2 x1 x0 xn     | The item n back in the stack is copied to the top.                           |
| ROLL           | 0x7a     | xn ... x2 x1 x0 <n>        | ... x2 x1 x0 xn        | The item n back in the stack is moved to the top.                            |
| ROT            | 0x7b     | x1 x2 x3                   | x2 x3 x1               | The 3rd item down the stack is moved to the top.                             |
| SWAP           | 0x7c     | x1 x2                      | x2 x1                  | The top two items on the stack are swapped.                                  |
| TUCK           | 0x7d     | x1 x2                      | x2 x1 x2               | The item at the top of the stack is copied and inserted before the second-to-top item. |
| 2DROP          | 0x6d     | x1 x2                      | Nothing                | Removes the top two stack items.                                             |
| 2DUP           | 0x6e     | x1 x2                      | x1 x2 x1 x2            | Duplicates the top two stack items.                                          |
| 3DUP           | 0x6f     | x1 x2 x3                   | x1 x2 x3 x1 x2 x3      | Duplicates the top three stack items.                                        |
| 2OVER          | 0x70     | x1 x2 x3 x4                | x1 x2 x3 x4 x1 x2      | Copies the pair of items two spaces back in the stack to the front.          |
| 2ROT           | 0x71     | x1 x2 x3 x4 x5 x6          | x3 x4 x5 x6 x1 x2      | The fifth and sixth items back are moved to the top of the stack.            |
| 2SWAP          | 0x72     | x1 x2 x3 x4                | x3 x4 x1 x2            | Swaps the top two pairs of items.                                            |

## Arithmetics..







