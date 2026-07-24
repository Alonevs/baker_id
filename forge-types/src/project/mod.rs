use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct ProjectData {
    pub name: String,
    pub version: String,
}
