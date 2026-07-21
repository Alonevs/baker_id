use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct LoadError(String);

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct Signal {
    pub name: String,
    pub arguments: Vec<String>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ECS {
    pub entities: Vec<Entity>,
}

#[allow(dead_code)]
impl ECS {
    pub fn new() -> Self {
        Self { entities: Vec::new() }
    }

    pub fn update(&mut self, _delta: f64) {
        // Update loop
    }

    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.push(entity);
    }

    pub fn get_entity(&self, id: &Uuid) -> Option<&Entity> {
        self.entities.iter().find(|e| &e.id == id)
    }
}
