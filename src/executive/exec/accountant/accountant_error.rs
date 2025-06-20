/// Error type for inserting allocations.
#[derive(Debug, Clone)]
pub enum InsertAllocError {
    /// The allocation already exists.
    MoreThanOneAllocationError,
}

/// Error type for inserting payments.
#[derive(Debug, Clone)]
pub enum InsertPaymentError {
    /// The payment is not valid.
    NonAllocatedPaymentError,
    /// The allocation is insufficient.
    AllocationExceededError,
}

/// Error type for summing payments.
#[derive(Debug, Clone)]
pub enum PayListError {
    /// The payment is not valid.
    InflationEncounteredError,
}
