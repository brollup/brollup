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

| Opcode          | Bytecode | Ops | Input                   | Output                 | Description                                                                  |
|:----------------|:---------|:----|:------------------------|:-----------------------|:-----------------------------------------------------------------------------|
| OP_TOALTSTACK   | 0x6b     | 1   | x1                      | (alt)x1                | Puts the input onto the top of the alt stack. Removes it from the main stack. |
| OP_FROMALTSTACK | 0x6c     | 1   | (alt)x1                 | x1                     | Puts the input onto the top of the main stack. Removes it from the alt stack. |

## Stack Operations

| Opcode          | Bytecode | Ops | Input                   | Output                 | Description                                                                  |
|:----------------|:---------|:----|:------------------------|:-----------------------|:-----------------------------------------------------------------------------|
| OP_2DROP        | 0x6d     | 1   | x1 x2                   | Nothing                | Removes the top two stack items.                                             |
| OP_2DUP         | 0x6e     | 1   | x1 x2                   | x1 x2 x1 x2            | Duplicates the top two stack items.                                          |
| OP_3DUP         | 0x6f     | 1   | x1 x2 x3                | x1 x2 x3 x1 x2 x3      | Duplicates the top three stack items.                                        |
| OP_2OVER        | 0x70     | 1   | x1 x2 x3 x4             | x1 x2 x3 x4 x1 x2      | Copies the pair of items two spaces back in the stack to the front.          |
| OP_2ROT         | 0x71     | 1   | x1 x2 x3 x4 x5 x6       | x3 x4 x5 x6 x1 x2      | The fifth and sixth items back are moved to the top of the stack.            |
| OP_2SWAP        | 0x72     | 1   | x1 x2 x3 x4             | x3 x4 x1 x2            | Swaps the top two pairs of items.                                            |
| OP_IFDUP        | 0x73     | 1   | x                       | x / x x                | If the top stack value is not 0, duplicate it.                               |
| OP_DEPTH        | 0x74     | 1   | Nothing                 | <Stack size>           | Puts the number of stack items onto the stack.                               |
| OP_DROP         | 0x75     | 1   | x                       | Nothing                | Removes the top stack item.                                                  |
| OP_DUP          | 0x76     | 1   | x                       | x x                    | Duplicates the top stack item.                                               |
| OP_NIP          | 0x77     | 1   | x1 x2                   | x2                     | Removes the second-to-top stack item.                                        |
| OP_OVER         | 0x78     | 1   | x1 x2                   | x1 x2 x1               | Copies the second-to-top stack item to the top.                              |
| OP_PICK         | 0x79     | 1   | xn ... x2 x1 x0 <n>     | xn ... x2 x1 x0 xn     | The item n back in the stack is copied to the top.                           |
| OP_ROLL         | 0x7a     | 1   | xn ... x2 x1 x0 <n>     | ... x2 x1 x0 xn        | The item n back in the stack is moved to the top.                            |
| OP_ROT          | 0x7b     | 1   | x1 x2 x3                | x2 x3 x1               | The 3rd item down the stack is moved to the top.                             |
| OP_SWAP         | 0x7c     | 1   | x1 x2                   | x2 x1                  | The top two items on the stack are swapped.                                  |
| OP_TUCK         | 0x7d     | 1   | x1 x2                   | x2 x1 x2               | The item at the top of the stack is copied and inserted before the second-to-top item. |

## Splice

| Opcode          | Bytecode | Ops | Input             | Output       | Description                                                          |
|:----------------|----------|:----|:------------------|:-------------|:---------------------------------------------------------------------|
| OP_CAT          | 0x7e     | 2   | x1 x2             | out          | Concatenates two strings.                                            |
| OP_SPLIT        | 0x7f     | 2   | in index          | out out      | Splits the byte array into two stack items at the index.             |
| OP_LEFT         | 0x80     | 2   | in size           | out          | Keeps only characters left of the specified point in a string.       |
| OP_RIGHT        | 0x81     | 2   | in size           | out          | Keeps only characters right of the specified point in a string.      |
| OP_SIZE         | 0x82     | 2   | in                | in size      | Pushes the string length of the top element of the stack (without popping it). |

## Bitwise

| Opcode          | Bytecode | Ops | Input          | Output                                  | Description                                                                  |
|:----------------|:---------|:----|:---------------|:----------------------------------------|:-----------------------------------------------------------------------------|
| OP_INVERT       | 0x83     | 2   | in             | out                                     | Flips all of the bits in the input.                                          |
| OP_AND          | 0x84     | 2   | x1 x2          | out                                     | Boolean and between each bit in the inputs.                                  |
| OP_OR           | 0x85     | 2   | x1 x2          | out                                     | Boolean or between each bit in the inputs.                                   |
| OP_XOR          | 0x86     | 2   | x1 x2          | out                                     | Boolean exclusive or between each bit in the inputs.                         |
| OP_EQUAL        | 0x87     | 1   | x1 x2          | True / false                            | Returns 1 if the inputs are exactly equal, 0 otherwise.                      |
| OP_EQUALVERIFY  | 0x88     | 2   | x1 x2          | Nothing / fail                          | Same as OP_EQUAL, but runs OP_VERIFY afterward.                              |
| OP_REVERSE      | 0x89     | 3   | in             | out                                     | Pop the top item from the stack and reverses the byte order.                 |

## Arithmetic

| Opcode         | Bytecode | Ops | Input          | Output                                  | Description                                                                  |
|:---------------|:---------|:----|:---------------|:----------------------------------------|:-----------------------------------------------------------------------------|
| OP_1ADD        | 0x8b     | 3   | in             | out                                     | 1 is added to the input.                                                     |
| OP_1SUB        | 0x8c     | 3   | in             | out                                     | 1 is subtracted from the input.                                              |
| OP_2MUL        | 0x8d     | 5   | in             | out                                     | The input is multiplied by 2.                                                |
| OP_2DIV        | 0x8e     | 5   | in             | out                                     | The input is divided by 2.                                                   |
| OP_ADDMOD      | 0x8f     | 3   | a b            | out                                     | a is added to b modulo MAX::U256.                                            |
| OP_MULMOD      | 0x90     | 3   | a b            | out                                     | a is multiplied by b modulo MAX::U256.                                       |
| OP_NOT         | 0x91     | 1   | in             | out                                     | If the input is 0 or 1, it is flipped. Otherwise the output will be 0.       |
| OP_0NOTEQUAL   | 0x92     | 1   | in             | out                                     | Returns 0 if the input is 0. 1 otherwise.                                    |
| OP_ADD         | 0x93     | 3   | a b            | out                                     | a is added to b.                                                             |
| OP_SUB         | 0x94     | 3   | a b            | out                                     | a is subtracted from b.                                                      |
| OP_MUL         | 0x95     | 5   | a b            | out                                     | a is multiplied by b.                                                        |
| OP_DIV         | 0x96     | 5   | a b            | out                                     | a is divided by b.                                                           |
| OP_LSHIFT      | 0x98     | 3   | a b            | out                                     | Shifts a left b bits.                                                        |
| OP_RSHIFT      | 0x99     | 3   | a b            | out                                     | Shifts a right b bits.                                                       |

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

## Reserved

| Opcode         | Bytecode | Ops | Input                | Output                 | Description                                                                     |
|:---------------|:---------|:----|:---------------------|:-----------------------|:--------------------------------------------------------------------------------|
| OP_RESERVED1   | 0x4e     | 0   | Nothing.             | Fail.                  | Fails the execution.                                                            |
| OP_RESERVED2   | 0x4f     | 0   | Nothing.             | Fail.                  | Fails the execution.                                                            |
| OP_RESERVED3   | 0x50     | 0   | Nothing.             | Fail.                  | Fails the execution.                                                            |
| OP_RESERVED4   | 0x8a     | 0   | Nothing.             | Fail.                  | Fails the execution.                                                            |
| OP_RESERVED5   | 0x97     | 0   | Nothing.             | Fail.                  | Fails the execution.                                                            |