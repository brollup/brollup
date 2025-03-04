use super::{upholdinack::CSessionUpholdINack, upholdonack::CSessionUpholdONack};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub enum CSessionUpholdNack {
    UpholdINack(CSessionUpholdINack),
    UpholdONack(CSessionUpholdONack),
}
