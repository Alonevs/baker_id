use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct EventData {
    pub id: u64,
    pub name: String,
}
