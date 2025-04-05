// Stack operations
pub const OP_PICK_OPS: u32 = 1;
pub const OP_ROLL_OPS: u32 = 1;
pub const OP_DROP_OPS: u32 = 1;
pub const OP_2DROP_OPS: u32 = 2;
pub const OP_DUP_OPS: u32 = 1;
pub const OP_2DUP_OPS: u32 = 2;

// Bitwise operations
pub const OP_EQUAL_OPS: u32 = 1;
pub const OP_EQUALVERIFY_OPS: u32 = 2;

// Flow
pub const OP_VERIFY_OPS: u32 = 1;

// Alt stack operations
pub const OP_FROMALTSTACK_OPS: u32 = 1;
pub const OP_TOALTSTACK_OPS: u32 = 1;

// Memory operations
pub const OP_MREAD_OPS: u32 = 5;
pub const OP_MWRITE_OPS: u32 = 5;
pub const OP_MFREE_OPS: u32 = 1;

// Splice operations
pub const OP_CAT_OPS: u32 = 10;

// Arithmetic operations
pub const OP_ADD_OPS: u32 = 3;
pub const OP_SUB_OPS: u32 = 3;
pub const OP_MUL_OPS: u32 = 10;
pub const OP_DIV_OPS: u32 = 10;
pub const OP_ADDMOD_OPS: u32 = 3;
pub const OP_MULMOD_OPS: u32 = 10;
