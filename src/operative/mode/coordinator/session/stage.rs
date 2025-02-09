#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CSessionStage {
    On,        // collect keys, hiding and binding nonces.
    Locked,    // no longer accepting remote. MusigNestingCtx ready.
    Ready,     // full musig conetxt is ready
    Finalized, // collected all partial sigs.
    Off,
}
