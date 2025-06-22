use crate::executive::{
    exec::{
        accountant::{accountant::Accountant, accountant_record::AccountantRecord},
        caller::Caller,
    },
    stack::{
        stack_error::{PaymentError, StackError, StackUintError},
        stack_holder::StackHolder,
        stack_uint::StackItemUintExt,
    },
};

/// Pays one or more accounts the specified amounts.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub struct OP_PAY;

/// The number of ops for the `OP_PAY` opcode.
pub const PAY_OPS: u32 = 10;

impl OP_PAY {
    pub fn execute(
        stack_holder: &mut StackHolder,
        accountant: &mut Accountant,
    ) -> Result<(), StackError> {
        // If this is not the active execution, return immediately.
        if !stack_holder.active_execution() {
            return Ok(());
        }

        // Get from key.
        let from_key = match stack_holder.caller() {
            Caller::Account(key) => key,
            Caller::Contract(_) => {
                return Err(StackError::PaymentError(PaymentError::CallerIsNotAnAccount));
            }
        };

        // Pop the amount from the stack.
        let amount_item = stack_holder.pop()?;

        // Pop the key from the stack.
        let to_key_item = stack_holder.pop()?;

        // Convert the amount to a `StackUint`.
        let amount_as_stack_uint =
            amount_item
                .to_stack_uint()
                .ok_or(StackError::StackUintError(
                    StackUintError::StackUintConversionError,
                ))?;

        let amount = amount_as_stack_uint.as_u32();

        // Convert the key to [u8; 32]
        let to_key: [u8; 32] = to_key_item
            .bytes()
            .try_into()
            .map_err(|_| StackError::Key32BytesConversionError)?;

        // Increment the payable spent value.
        if !stack_holder.increment_payable_spent(amount) {
            return Err(StackError::PaymentError(
                PaymentError::PayableAllocationExceeded,
            ));
        }

        // Construct a new payment.
        let record = AccountantRecord::new(from_key, to_key, amount);

        // Insert the payment into the accountant.
        if let Err(error) = accountant.insert_record(record) {
            return Err(StackError::PaymentError(
                PaymentError::AccountantPaymentInsertionError(error),
            ));
        }

        // Increment the ops counter.
        stack_holder.increment_ops(PAY_OPS)?;

        Ok(())
    }

    /// Returns the bytecode for the `OP_PAY` opcode (0xc3).
    pub fn bytecode() -> Vec<u8> {
        vec![0xc3]
    }
}
