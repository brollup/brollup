use musig2::{
    secp256k1::{self, PublicKey, XOnlyPublicKey},
    KeyAggContext,
};

pub trait KeyAgg {
    fn key_agg_ctx(&self, tweak: Option<[u8; 32]>) -> Result<KeyAggContext, secp256k1::Error>;
    fn agg_key(&self, tweak: Option<[u8; 32]>) -> Result<XOnlyPublicKey, secp256k1::Error>;
}

impl KeyAgg for Vec<XOnlyPublicKey> {
    fn key_agg_ctx(
        &self,
        taproot_tweak: Option<[u8; 32]>,
    ) -> Result<KeyAggContext, secp256k1::Error> {
        // Lift keys
        let mut keys_lifted = Vec::<PublicKey>::new();

        for key in self {
            keys_lifted.push(key.public_key(secp256k1::Parity::Even));
        }

        // Sort the keys
        keys_lifted.sort();

        // Create and return the key aggregation context.
        let ctx: KeyAggContext = match taproot_tweak {
            None => KeyAggContext::new(keys_lifted.into_iter())
                .map_err(|_| secp256k1::Error::InvalidPublicKey)?,
            Some(taproot_tweak) => KeyAggContext::new(keys_lifted.into_iter())
                .map_err(|_| secp256k1::Error::InvalidPublicKey)?
                .with_taproot_tweak(&taproot_tweak)
                .map_err(|_| secp256k1::Error::InvalidTweak)?,
        };

        Ok(ctx)
    }

    fn agg_key(&self, tweak: Option<[u8; 32]>) -> Result<XOnlyPublicKey, secp256k1::Error> {
        Ok(self.key_agg_ctx(tweak)?.aggregated_pubkey())
    }
}
