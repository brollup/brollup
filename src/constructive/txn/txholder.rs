use crate::valtype::atomic::AtomicVal;
use bitcoin::{OutPoint, Transaction};

/// Default number of inputs: `Payload`.
const DEFAULT_NUM_INS: u32 = 1;

/// Default number of outputs: `Payload`, `VTXO Projector`, `Connector Projector`.
const DEFAULT_NUM_OUTS: u32 = 3;

/// A holder for a transaction, its extra inputs and outputs, and its input and output iterators.
#[derive(Debug, Clone)]
pub struct TxHolder {
    /// The transaction.
    tx: Transaction,
    /// The number of extra inputs.
    extra_in: AtomicVal,
    /// The number of extra outputs.
    extra_out: AtomicVal,
    /// The current input iterator.
    iterator_in: u32,
    /// The current output iterator.
    iterator_out: u32,
}

impl TxHolder {
    /// Creates a new `TxHolder`.
    pub fn new(tx: Transaction, extra_in: u8, extra_out: u8) -> Option<TxHolder> {
        // Get initial iterator positions.
        let (iterator_in, iterator_out) = (
            // Default number of inputs plus the number of extra inputs.
            DEFAULT_NUM_INS + extra_in as u32,
            // Default number of outputs plus the number of extra outputs.
            DEFAULT_NUM_OUTS + extra_out as u32,
        );

        // Convert the extra inputs/outputs to their respective atomic values.
        let (extra_in, extra_out) = (AtomicVal::new_u8(extra_in)?, AtomicVal::new_u8(extra_out)?);

        let tx_holder = TxHolder {
            tx,
            extra_in,
            extra_out,
            iterator_in,
            iterator_out,
        };

        Some(tx_holder)
    }

    /// Returns the transaction.
    pub fn tx(&self) -> Transaction {
        self.tx.clone()
    }

    /// Returns the number of extra inputs.
    pub fn extra_in(&self) -> u8 {
        self.extra_in.value_as_u8()
    }

    /// Returns the number of extra outputs.
    pub fn extra_out(&self) -> u8 {
        self.extra_out.value_as_u8()
    }

    /// Returns the current input iterator.
    pub fn iterator_in(&self) -> u32 {
        self.iterator_in
    }

    /// Returns the current output iterator.
    pub fn iterator_out(&self) -> u32 {
        self.iterator_out
    }

    /// Iterates the input iterator by one.
    pub fn iterate_in(&mut self) {
        self.iterator_in += 1;
    }

    /// Iterates the output iterator by one.
    pub fn iterate_out(&mut self) {
        self.iterator_out += 1;
    }

    /// Returns whether both input and output iterators are at the end of the transaction.
    pub fn iterators_final(&self) -> bool {
        // Check if the input iterator is at the end of the transaction.
        if self.iterator_in != self.tx.input.len() as u32 {
            return false;
        }

        // Check if the output iterator is at the end of the transaction.
        if self.iterator_out != self.tx.output.len() as u32 {
            return false;
        }

        true
    }

    /// Returns the current input outpoint.
    pub fn current_in_outpoint(&self) -> OutPoint {
        let current_iter = self.iterator_in as usize;
        self.tx.input[current_iter].previous_output
    }
}
