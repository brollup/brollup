use bls_on_arkworks::{self as bls, errors::BLSError, types::Signature};

/// Aggregate a list of BLS signatures.
///
/// # Arguments
///
/// * `signatures` - A list of BLS signatures.
///
pub fn bls_aggregate(signatures: Vec<[u8; 96]>) -> Result<[u8; 96], BLSError> {
    // Convert the signatures to a vector of vectors.
    let signatures: Vec<Signature> = signatures.into_iter().map(|sig| sig.to_vec()).collect();

    // Aggregate the signatures.
    let aggregate_sig: [u8; 96] = bls::aggregate(&signatures)?
        .try_into()
        .expect("Unexpected BLS aggregate signature conversion error.");

    // Return the aggregate signature.
    Ok(aggregate_sig)
}
