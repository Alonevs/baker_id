use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct SceneData {
    pub entities: Vec<u64>,
    pub layers: Vec<u32>,
}
