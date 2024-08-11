use musig2::{
    errors::KeyAggError,
    secp256k1::{self, PublicKey, XOnlyPublicKey},
    KeyAggContext,
};

pub trait KeyAgg {
    fn key_agg_ctx(&self) -> Result<KeyAggContext, KeyAggError>;
    fn agg_key(&self) -> Result<XOnlyPublicKey, KeyAggError>;
}

impl KeyAgg for Vec<XOnlyPublicKey> {
    fn key_agg_ctx(&self) -> Result<KeyAggContext, KeyAggError> {
        // Lift keys
        let mut keys_lifted = Vec::<PublicKey>::new();

        for key in self {
            keys_lifted.push(key.public_key(secp256k1::Parity::Even));
        }

        // Sort the keys
        keys_lifted.sort();

        // Create and return the key aggregation context.
        KeyAggContext::new(keys_lifted.into_iter())
    }

    fn agg_key(&self) -> Result<XOnlyPublicKey, KeyAggError> {
        Ok(self.key_agg_ctx()?.aggregated_pubkey())
    }
}
