use crate::executive::exec::accountant::{
    accountant_error::{InsertAllocError, InsertPaymentError, PayListError},
    accountant_record::AccountantRecord,
};
use std::collections::HashMap;

/// The type of account key.
type AccountKey = [u8; 32];

/// The type of payable allocation amount.
type PayableAllocAmount = u32;

/// A keeper for payments.
pub struct Accountant {
    allocs: HashMap<AccountKey, PayableAllocAmount>,
    records: Vec<AccountantRecord>,
    records_backup: Vec<AccountantRecord>,
}

impl Accountant {
    /// Creates a new accountant.
    pub fn new() -> Self {
        Self {
            allocs: HashMap::<AccountKey, PayableAllocAmount>::new(),
            records: Vec::new(),
            records_backup: Vec::new(),
        }
    }

    /// Backups the checks.
    pub fn backup(&mut self) {
        self.records_backup = self.records.clone();
    }

    /// Inserts an allocation. No overlapping allocations are allowed.
    pub fn insert_alloc(&mut self, key: [u8; 32], amount: u32) -> Result<(), InsertAllocError> {
        // Insert the allocation.
        if let Some(_) = self.allocs.insert(key, amount) {
            return Err(InsertAllocError::MoreThanOneAllocationError);
        }

        Ok(())
    }

    /// Returns the total spent by an account.
    fn total_spent_by_account(&self, key: [u8; 32]) -> u32 {
        // Iterate payments and sum the amount of money spent by the account.
        let mut total = 0;

        // Iterate payments and sum the amount of money spent by the account.
        for record in self.records.iter() {
            if record.from() == key {
                total += record.amount();
            }
        }

        // Return the total spent by the account.
        total
    }

    /// Inserts a check.
    pub fn insert_record(&mut self, record: AccountantRecord) -> Result<(), InsertPaymentError> {
        // Retrieve the allocation for the account.
        let allocation = match self.allocs.get(&record.from()) {
            Some(amount) => *amount,
            // If the account is not allocated, return an error.
            None => return Err(InsertPaymentError::NonAllocatedPaymentError),
        };

        // Get the total spent by the account.
        let total_spent = self.total_spent_by_account(record.from());

        // Check if the allocation exceeds.
        if allocation < total_spent + record.amount() {
            return Err(InsertPaymentError::AllocationExceededError);
        }

        // Insert the payment.
        self.records.push(record);

        Ok(())
    }

    /// Restores the checks by swapping the checks and backup vectors.
    pub fn reverse_last(&mut self) {
        self.records = self.records_backup.clone();
    }

    /// Reverses all checks by emptying the checks and backup vectors.
    pub fn reverse_all(&mut self) {
        self.records = Vec::<AccountantRecord>::new();
        self.records_backup = Vec::<AccountantRecord>::new();
    }

    /// Returns list of account and amount pairs who are allocated money.
    pub fn allocs(&self) -> HashMap<[u8; 32], u32> {
        self.allocs.clone()
    }

    /// Returns list of account and amount pairs who are owed money.
    pub fn pay_list(&self) -> Result<HashMap<[u8; 32], u32>, PayListError> {
        // Create a new HashMap to store sum of payments.
        let mut pay_list_ = HashMap::<[u8; 32], i32>::new();

        // Iterate allocs, for each account collect their change.
        for (key, amount) in self.allocs.iter() {
            match pay_list_.get(key) {
                Some(balance) => {
                    pay_list_.insert(*key, balance + *amount as i32);
                }
                None => {
                    pay_list_.insert(*key, *amount as i32);
                }
            }
        }

        // Iterate checks, for each account collect their change.
        for record in self.records.iter() {
            let from_key = record.from();
            let to_key = record.to();
            let amount = record.amount();

            // Deduct from payers.
            match pay_list_.get(&from_key) {
                Some(balance) => {
                    pay_list_.insert(from_key, balance - amount as i32);
                }
                None => {
                    pay_list_.insert(from_key, -(amount as i32));
                }
            }

            // Add to payees.
            match pay_list_.get(&to_key) {
                Some(balance) => {
                    pay_list_.insert(to_key, balance + amount as i32);
                }
                None => {
                    pay_list_.insert(to_key, amount as i32);
                }
            }
        }

        // If at least one negative balance is encountered, return an inflation error.
        if pay_list_.values().any(|balance| *balance < 0) {
            return Err(PayListError::InflationEncounteredError);
        }

        // Prune the zero balances.
        pay_list_.retain(|_, balance| *balance != 0);

        // Convert the balances to u32.
        let pay_list = pay_list_
            .iter()
            .map(|(key, balance)| (*key, *balance as u32))
            .collect();

        // Return the final paid list.
        Ok(pay_list)
    }
}
