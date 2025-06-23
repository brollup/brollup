use crate::constructive::entry::combinator::combinator_type::CombinatorType;
use crate::constructive::entry::combinator::combinators::call::call::Call;
use crate::transmutative::hash::Hash;
use crate::transmutative::{hash::HashTag, secp::authenticable::AuthSighash};

impl AuthSighash for Call {
    fn auth_sighash(&self) -> [u8; 32] {
        let mut preimage: Vec<u8> = Vec::<u8>::new();

        // Account key
        preimage.extend(self.account_key);

        // Contract id
        preimage.extend(self.contract_id);

        // Method index as u32
        preimage.extend((self.method_index as u32).to_le_bytes());

        // Number of args as u32
        preimage.extend((self.args.len() as u32).to_le_bytes());

        // Args
        for arg in self.args.iter() {
            preimage.extend(arg.into_stack_item().bytes());
        }

        // Ops budget as u32
        preimage.extend((self.ops_budget as u32).to_le_bytes());

        // Base ops price as u32
        preimage.extend((self.ops_price_base as u32).to_le_bytes());

        // Extra ops price.
        match self.ops_price_extra_in {
            Some(extra_ops_price) => {
                preimage.push(0x01);
                preimage.extend((extra_ops_price as u32).to_le_bytes());
            }
            None => {
                preimage.push(0x00);
            }
        }

        // Hash the preimage
        preimage.hash(Some(HashTag::SighashCombinator(CombinatorType::Call)))
    }
}
