# Opcodes
Brollup uses an extended Bitcoin script with splicing, better memory management, and global state opcodes.

## Data push

| Opcode        | Bytecode  | Ops | Input       | Output         | Description                                                                                        |
|:--------------|:----------|:----|:------------|:---------------|:---------------------------------------------------------------------------------------------------|
| OP_FALSE OP_0 | 0x00      | 1   | Nothing.    | (empty value)  | An empty array of bytes is pushed onto the stack.                                                  |
| OP_N/A        | 0x01-0x4b | 2   | (special)   | data           | *Bytecode* number of following bytes pushed onto the stack.                                        |
| OP_PUSHDATA1  | 0x4c      | 2   | (special)   | data           | The next byte contains the number of bytes to push onto the stack.                                 |
| OP_PUSHDATA2  | 0x4d      | 2   | (special)   | data           | The next two bytes contain the number of bytes to be pushed onto the stack in little endian order. |
| OP_TRUE OP_1  | 0x51      | 1   | (special)   | 1              | The number 1 is pushed onto the stack.                                                             |
| OP_2-OP_16    | 0x52-0x60 | 1   | 2-16        | 1              | The number in the word name (2-16) is pushed onto the stack.                                       |

## Flow control

| Opcode        | Bytecode  | Ops | Input       | Output         | Description                                                                                        |
|:--------------|:----------|:----|:------------|:---------------|:---------------------------------------------------------------------------------------------------|
| OP_NOP        | 0x61      | 1   | Nothing.    | Nothing.       | Does nothing.                                                                                      |
| OP_RETURNERR  | 0x62      | 1   | Nothing.    | Return.        | Pops the top stack item and returns it as error message.                                           |
| OP_IF         | 0x63      | 1   | True/false  | Nothing.       | If the top stack value is not False, the statements are executed. The top stack value is removed.  |
| OP_NOTIF      | 0x64      | 1   | True/false  | Nothing.       | If the top stack value is False, the statements are executed. The top stack value is removed.      |
| OP_RETURNALL  | 0x65      | 1   | x1..xn      | Return.        | All stack items are popped and returned.                                                           |
| OP_RETURNSOME | 0x66      | 1   | x1..xn      | Return.        | Pops the stack item *count* and returns *count* number of items.                                   |
| OP_ELSE       | 0x67      | 1   | Nothing.    | Nothing.       | If the preceding OP_IF or OP_NOTIF or OP_ELSE was not executed then these statements are.          |
| OP_ENDIF      | 0x68      | 1   | Nothing.    | Nothing.       | Ends an if/else block. All blocks must end, or the transaction is invalid.                         |
| OP_VERIFY     | 0x69      | 1   | True/false  | Nothing/Fail   | Pops the top stack item and marks transaction as invalid if top stack value is not true.           |
| OP_FAIL       | 0x6a      | 1   | (special)   | Fail.          | Fails the entry.                                                                                   |

## Alstack Operations
| OP_TOALTSTACK   | 1   | 0x6b     | x1                      | (alt)x1                | Puts the input onto the top of the alt stack. Removes it from the main stack. |
| OP_FROMALTSTACK | 1   | 0x6c     | (alt)x1                 | x1                     | Puts the input onto the top of the main stack. Removes it from the alt stack. |

## Stack Operations

| Opcode          | Ops | Bytecode | Input                   | Output                 | Description                                                                  |
|:----------------|:----|:---------|:------------------------|:-----------------------|:-----------------------------------------------------------------------------|
| OP_IFDUP        | 1   | 0x73     | x                       | x / x x                | If the top stack value is not 0, duplicate it.                               |
| OP_DEPTH        | 1   | 0x74     | Nothing                 | <Stack size>           | Puts the number of stack items onto the stack.                               |
| OP_DROP         | 1   | 0x75     | x                       | Nothing                | Removes the top stack item.                                                  |
| OP_DUP          | 1   | 0x76     | x                       | x x                    | Duplicates the top stack item.                                               |
| OP_NIP          | 1   | 0x77     | x1 x2                   | x2                     | Removes the second-to-top stack item.                                        |
| OP_OVER         | 1   | 0x78     | x1 x2                   | x1 x2 x1               | Copies the second-to-top stack item to the top.                              |
| OP_PICK         | 1   | 0x79     | xn ... x2 x1 x0 <n>     | xn ... x2 x1 x0 xn     | The item n back in the stack is copied to the top.                           |
| OP_ROLL         | 1   | 0x7a     | xn ... x2 x1 x0 <n>     | ... x2 x1 x0 xn        | The item n back in the stack is moved to the top.                            |
| OP_ROT          | 1   | 0x7b     | x1 x2 x3                | x2 x3 x1               | The 3rd item down the stack is moved to the top.                             |
| OP_SWAP         | 1   | 0x7c     | x1 x2                   | x2 x1                  | The top two items on the stack are swapped.                                  |
| OP_TUCK         | 1   | 0x7d     | x1 x2                   | x2 x1 x2               | The item at the top of the stack is copied and inserted before the second-to-top item. |
| OP_2DROP        | 1   | 0x6d     | x1 x2                   | Nothing                | Removes the top two stack items.                                             |
| OP_2DUP         | 1   | 0x6e     | x1 x2                   | x1 x2 x1 x2            | Duplicates the top two stack items.                                          |
| OP_3DUP         | 1   | 0x6f     | x1 x2 x3                | x1 x2 x3 x1 x2 x3      | Duplicates the top three stack items.                                        |
| OP_2OVER        | 1   | 0x70     | x1 x2 x3 x4             | x1 x2 x3 x4 x1 x2      | Copies the pair of items two spaces back in the stack to the front.          |
| OP_2ROT         | 1   | 0x71     | x1 x2 x3 x4 x5 x6       | x3 x4 x5 x6 x1 x2      | The fifth and sixth items back are moved to the top of the stack.            |
| OP_2SWAP        | 1   | 0x72     | x1 x2 x3 x4             | x3 x4 x1 x2            | Swaps the top two pairs of items.                                            |



## Splice

| Opcode    | Bytecode | Ops | Input             | Output       | Description                                                          |
|:----------|----------|:----|:------------------|:-------------|:---------------------------------------------------------------------|
| OP_CAT    | 0x7e     | 5  | x1 x2             | out          | Concatenates two strings.                                            |
| OP_SUBSTR | 0x7f     | 5  | in begin size     | out          | Returns a section of a string.                                       |
| OP_LEFT   | 0x80     | 5  | in size           | out          | Keeps only characters left of the specified point in a string.       |
| OP_RIGHT  | 0x81     | 5  | in size           | out          | Keeps only characters right of the specified point in a string.      |
| OP_SIZE   | 0x82     | 5  | in                | in size      | Pushes the string length of the top element of the stack (without popping it). |

## Arithmetic

| Opcode         | Ops | Bytecode | Input          | Output                                  | Description                                                                  |
|:---------------|:----|:---------|:---------------|:----------------------------------------|:-----------------------------------------------------------------------------|
| OP_ADD         | 3   | 0x93     | x1 x2          | (x1 + x2) True or x1 x2 False           | x1 is added to x2.                                                           |
| OP_SUB         | 3   | 0x94     | x1 x2          | (x1 - x2) True or x1 x2 False           | x1 is subtracted from x2.                                                    |
| OP_MUL         | 10  | 0x95     | x1 x2          | (x1 * x2) True or x1 x2 False           | x1 is multiplied by x2.                                                      |
| OP_DIV         | 10  | 0x96     | x1 x2          | (x1 % x2) (x1 / x2) True or x1 x2 False | x1 is divided by x2.                                                         |
| OP_ADDMOD      | 3   | 0x8d     | x1 x2          | (x1 + x2) % MAX::U256                   | x1 is added to x2 modulo MAX::U256.                                          |
| OP_MULMOD      | 10  | 0x8e     | x1 x2          | (x1 * x2) % MAX::U256                   | x1 is multiplied by x2 modulo MAX::U256.                                     |

## Memory

| Opcode         | Bytecode | Ops | Input                | Output                 | Description                                                                     |
|:---------------|:---------|:----|:---------------------|:-----------------------|:--------------------------------------------------------------------------------|
| OP_MWRITE      | 0xb2     | 5   | x1 x2                | x1                     | Pops the memory key and value, and writes the value to the contract's memory.   |
| OP_MREAD       | 0xb3     | 5   | x1                   | x1                     | Pops the memory key, and reads the value from the contract's memory.            |
| OP_MFREE       | 0xb4     | 1   | x1                   | x1                     | Pops the memory key, and frees the key/value from the contract's memory.        |

## Storage

| Opcode         | Bytecode | Ops | Input                | Output                 | Description                                                                     |
|:---------------|:---------|:----|:---------------------|:-----------------------|:--------------------------------------------------------------------------------|
| OP_SWRITE      | 0xb5     | 50  | x1 x2                | x1                     | Pops the storage key and value, and writes the value to the contract's storage. |
| OP_SREAD       | 0xb6     | 50  | x1                   | x1                     | Pops the storage key, and reads the value from the contract's storage.          |
| OP_SFREE       | 0xb7     | 1   | x1                   | x1                     | Pops the storage key, and frees the key/value from the contract's storage.      |

