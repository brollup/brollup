// A payment instance for OP_PAY.
#[derive(Clone)]
pub struct AccountantRecord {
    from: [u8; 32],
    to: [u8; 32],
    amount: u32,
}

impl AccountantRecord {
    /// Creates a new payment instance.
    pub fn new(from: [u8; 32], to: [u8; 32], amount: u32) -> Self {
        Self { from, to, amount }
    }

    /// Returns the from address.
    pub fn from(&self) -> [u8; 32] {
        self.from
    }

    /// Returns the to address.
    pub fn to(&self) -> [u8; 32] {
        self.to
    }

    /// Returns the amount.
    pub fn amount(&self) -> u32 {
        self.amount
    }
}
