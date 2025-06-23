use crate::{
    constructive::{
        cpe::cpe::CompactPayloadEncoding,
        entry::combinator::combinators::call::{
            call::Call, codec::cpe::encode_error::CPECallEncodeError,
        },
        valtype::{atomic_val::AtomicVal, short_val::ShortVal},
    },
    inscriptive::{registery::contract_registery::CONTRACT_REGISTERY, repo::repo::PROGRAMS_REPO},
};
use bit_vec::BitVec;

impl Call {
    /// Encodes the call as a bit vector.
    pub async fn encode_cpe(
        &self,
        account_key: [u8; 32],
        contract_registery: &CONTRACT_REGISTERY,
        repo: &PROGRAMS_REPO,
        ops_price_base: u32,
    ) -> Result<BitVec, CPECallEncodeError> {
        // Initialize empty bit vector.
        let mut bits = BitVec::new();

        // Match the account key.
        if account_key != self.account_key {
            return Err(CPECallEncodeError::AccountKeyMismatch);
        }

        // Contract rank
        let contract_rank = {
            let _contract_registery = contract_registery.lock().await;
            _contract_registery.rank_by_contract_id(self.contract_id)
        }
        .ok_or(CPECallEncodeError::ContractRankNotFoundAtContractId(
            self.contract_id,
        ))?;

        // Contract rank as shortval
        let contract_rank_as_shortval = ShortVal::new(contract_rank as u32);

        // Extend the contract rank as shortval.
        bits.extend(
            contract_rank_as_shortval
                .encode_cpe()
                .ok_or(CPECallEncodeError::ContractRankCPEEncodeError)?,
        );

        // Methods length
        let contract_methods_count = {
            let _repo = repo.lock().await;
            _repo.methods_len_by_contract_id(&self.contract_id)
        }
        .ok_or(CPECallEncodeError::ContractMethodCountNotFoundAtContractId(
            self.contract_id,
        ))?;

        // Method index as atomic value
        let method_index_as_atomicval = AtomicVal::new(self.method_index, contract_methods_count);

        // Extend the method index.
        bits.extend(
            method_index_as_atomicval
                .encode_cpe()
                .ok_or(CPECallEncodeError::MethodIndexCPEEncodeError)?,
        );

        // Extend the args.
        // No need to encode the args length.
        for arg in self.args.iter() {
            bits.extend(
                arg.encode_cpe()
                    .ok_or(CPECallEncodeError::ArgsCPEEncodeError)?,
            );
        }

        // Ops budget as shortval
        let ops_budget_as_shortval = ShortVal::new(self.ops_budget as u32);

        // Extend the ops budget.
        bits.extend(
            ops_budget_as_shortval
                .encode_cpe()
                .ok_or(CPECallEncodeError::OpsBudgetCPEEncodeError)?,
        );

        // Match the ops price base.
        if ops_price_base != self.ops_price_base {
            return Err(CPECallEncodeError::BaseOpsPriceMismatch);
        }

        // Match ops price extra in.
        match self.ops_price_extra_in {
            None => {
                // Push false for this field being absent.
                bits.push(false);
            }
            Some(ops_price_extra_in) => {
                // Push true for this field being present.
                bits.push(true);

                // Convert the ops price extra in to a shortval.
                let ops_price_extra_in_as_shortval = ShortVal::new(ops_price_extra_in as u32);

                // Extend the ops price extra in.
                bits.extend(
                    ops_price_extra_in_as_shortval
                        .encode_cpe()
                        .ok_or(CPECallEncodeError::OpsPriceExtraInCPEEncodeError)?,
                );
            }
        }

        // Return the bits.
        Ok(bits)
    }
}
