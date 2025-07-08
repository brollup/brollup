use crate::{
    constructive::entry::combinator::combinators::recharge::{
        codec::cpe::decode::decode_error::RechargeCPEDecodingError, recharge::Recharge,
    },
    inscriptive::set::vtxo_set::VTXO_SET,
};
use secp::Point;

impl Recharge {
    /// Decodes `Recharge` from a compact bit stream.
    pub async fn decode_cpe<'a>(
        _bit_stream: &mut bit_vec::Iter<'a>,
        account_key: Point,
        vtxo_set: &VTXO_SET,
        current_bitcoin_height: u32,
    ) -> Result<Recharge, RechargeCPEDecodingError> {
        // Decoding Recharage does not involve any bit stream iteration.
        // We simply retrieve *all* rechargeable VTXOs associated with the account from the local storage.

        // Get the VTXOs to recharge.
        let rechargeable_vtxos = {
            let _vtxo_set = vtxo_set.lock().await;
            _vtxo_set.vtxos_to_recharge(&account_key, current_bitcoin_height)
        };

        // Check if there are any rechargeable VTXOs.
        if rechargeable_vtxos.is_empty() {
            return Err(RechargeCPEDecodingError::NoRechargeableVTXOsFound);
        }

        // Construct the recharge.
        let recharge = Recharge::new(rechargeable_vtxos)
            .ok_or(RechargeCPEDecodingError::RechargeConstructionError)?;

        // Return the recharge.
        Ok(recharge)
    }
}
