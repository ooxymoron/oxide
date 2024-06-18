
use serde::{Deserialize, Serialize};

use crate::util::arcm::Arcm;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriggerSettings {
    pub sticky: Arcm<bool>,
}

impl TriggerSettings {
    pub fn new() -> TriggerSettings {
        TriggerSettings {
            sticky: Arcm::new(false),
        }
    }
}
