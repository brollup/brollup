use crate::executive::exec::accountant::payment::Payment;
use std::collections::HashMap;

type AccountKey = [u8; 32];
type PayableAllocAmount = u32;

/// A keeper for payments.
pub struct Accountant {
    allocs: HashMap<AccountKey, PayableAllocAmount>,
    payments: Vec<Payment>,
    payments_backup: Vec<Payment>,
}

impl Accountant {
    /// Creates a new accountant.
    pub fn new() -> Self {
        Self {
            allocs: HashMap::<AccountKey, PayableAllocAmount>::new(),
            payments: Vec::new(),
            payments_backup: Vec::new(),
        }
    }

    /// Backups the checks.
    pub fn backup(&mut self) {
        self.payments_backup = self.payments.clone();
    }

    /// Inserts an allocation. No overlapping allocations are allowed.
    pub fn insert_alloc(&mut self, key: AccountKey, amount: PayableAllocAmount) -> bool {
        // If the allocation already exists, return false.
        if self.allocs.contains_key(&key) {
            return false;
        }

        // Insert the allocation.
        self.allocs.insert(key, amount);

        // Return true if the allocation was inserted.
        true
    }

    /// Inserts a check.
    pub fn insert_payment(&mut self, payment: Payment) -> bool {
        // Check if from belongs to allocs.
        if !self.allocs.contains_key(&payment.from()) {
            return false;
        }

        self.payments.push(payment);

        true
    }

    /// Restores the checks by swapping the checks and backup vectors.
    pub fn reverse_last(&mut self) {
        self.payments = self.payments_backup.clone();
    }

    /// Reverses all checks by emptying the checks and backup vectors.
    pub fn reverse_all(&mut self) {
        self.payments = Vec::<Payment>::new();
        self.payments_backup = Vec::<Payment>::new();
    }

    /// Returns list of account and amount pairs who are allocated money.
    pub fn spends(&self) -> HashMap<[u8; 32], u32> {
        self.allocs.clone()
    }

    /// Returns list of account and amount pairs who are owed money.
    pub fn paids_sum(&self) -> HashMap<[u8; 32], u32> {
        // Create a new HashMap to store sum of payments.
        let mut paid_list_ = HashMap::<[u8; 32], i32>::new();

        // Iterate allocs, for each account collect their change.
        for (key, amount) in self.allocs.iter() {
            match paid_list_.get(key) {
                Some(balance) => {
                    paid_list_.insert(*key, balance + *amount as i32);
                }
                None => {
                    paid_list_.insert(*key, *amount as i32);
                }
            }
        }

        // Iterate checks, for each account collect their change.
        for payment in self.payments.iter() {
            let from_key = payment.from();
            let to_key = payment.to();
            let amount = payment.amount();

            // Deduct from payers.
            match paid_list_.get(&from_key) {
                Some(balance) => {
                    paid_list_.insert(from_key, balance - amount as i32);
                }
                None => {
                    paid_list_.insert(from_key, -(amount as i32));
                }
            }

            // Add to payees.
            match paid_list_.get(&to_key) {
                Some(balance) => {
                    paid_list_.insert(to_key, balance + amount as i32);
                }
                None => {
                    paid_list_.insert(to_key, amount as i32);
                }
            }
        }

        // Prune the negative or zero balances.
        paid_list_.retain(|_, balance| *balance > 0);

        // Convert the balances to u32.
        let paid_list = paid_list_
            .iter()
            .map(|(key, balance)| (*key, *balance as u32))
            .collect();

        // Return the final paid list.
        paid_list
    }
}
