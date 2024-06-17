use serde::{Deserialize, Serialize};

use crate::util::arcm::Arcm;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpreadReductionSettings {
    pub seed_prediction: Arcm<bool>,
    pub tapfire: Arcm<bool>,
    pub tapfire_on_manual_shots: Arcm<bool>,
    pub tapfire_only_minigun: Arcm<bool>,
}

impl SpreadReductionSettings {
    pub fn new() -> SpreadReductionSettings {
        SpreadReductionSettings {
            seed_prediction: Arcm::new(false),
            tapfire: Arcm::new(false),
            tapfire_on_manual_shots: Arcm::new(false),
            tapfire_only_minigun: Arcm::new(false),
        }
    }
}
