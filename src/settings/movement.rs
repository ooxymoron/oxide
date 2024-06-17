use serde::{Deserialize, Serialize};

use crate::util::arcm::Arcm;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovementSettings {
    pub bhop: Arcm<bool>,
    pub autostrafe: Arcm<bool>,
    pub no_push: Arcm<bool>,
    pub momentum_compensation: Arcm<bool>,
}

impl MovementSettings {
    pub fn new() -> MovementSettings {
        MovementSettings {
            bhop: Arcm::new(false),
            autostrafe: Arcm::new(false),
            no_push: Arcm::new(false),
            momentum_compensation: Arcm::new(false),
        }
    }
}
