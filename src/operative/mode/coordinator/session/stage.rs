#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CSessionStage {
    On,        // collect keys, hiding and binding nonces.
    Locked,    // no longer accepting remote. MusigNestingCtx ready.
    Finalized, // collected all partial sigs.
    Off,
}
