use crate::{dkgops::DKGOps, into::IntoScalar, CSESSION_CTX, DKG_MANAGER, PEER_MANAGER};
use std::time::Duration;

// covr <height> <msg>
pub async fn command(
    parts: Vec<&str>,
    peer_manager: &mut PEER_MANAGER,
    dkg_manager: &DKG_MANAGER,
    session_ctx: &mut CSESSION_CTX,
) {

    // Set session stage on.

    // Await & insert commits.

    // Set session stage locked. This will set ctxes.

    // Return respective commitacks.

    // Await & insert upholds.

    // Return upholdacks.
}
