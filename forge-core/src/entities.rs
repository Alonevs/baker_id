use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct Entity {
    pub id: Uuid,
    pub name: String,
    pub kind: EntityKind,
    pub properties: HashMap<String, serde_json::Value>,
    pub components: Vec<Arc<Component>>,
    pub parent_id: Option<Uuid>,
    pub world_id: Option<Uuid>,
}

#[derive(Debug, Clone)]
pub enum EntityKind {
    Node,
    GameObject,
    Character,
    Enemy,
    Item,
    Projectile,
    Effect,
    Particle,
    Light,
    Camera,
    Audio,
    AI,
    Inventory,
    UI,
    Scene,
}

#[derive(Debug, Clone)]
pub struct Component {
    pub id: Uuid,
    pub name: String,
    pub kind: ComponentKind,
    pub data: serde_json::Value,
    pub entity_id: Option<Uuid>,
}

#[derive(Debug, Clone)]
pub enum ComponentKind {
    Transform,
    Collider,
    PhysicsBody,
    Renderer,
    AudioSource,
    ParticleEmitter,
    Light,
    Camera,
    Input,
    Animation,
    AI,
    Inventory,
    UIElement,
    Script,
    Custom(String),
}

#[derive(Debug, Error)]
pub enum EntityError {
    #[error("Entity not found: {0}")]
    EntityNotFound(Uuid),
    #[error("Component not found: {0}")]
    ComponentNotFound(Uuid),
    #[error("Invalid entity operation")]
    InvalidOperation,
}

pub struct World {
    pub entities: HashMap<Uuid, Arc<Entity>>,
    pub components: HashMap<Uuid, Arc<Component>>,
    pub entity_by_name: HashMap<String, Uuid>,
    pub parent_map: HashMap<Uuid, Vec<Uuid>>,
    pub world_id: Uuid,
}

impl World {
    pub fn new() -> Self {
        let world_id = Uuid::new_v4();
        Self {
            entities: HashMap::new(),
            components: HashMap::new(),
            entity_by_name: HashMap::new(),
            parent_map: HashMap::new(),
            world_id,
        }
    }

    pub fn add_entity(&mut self, entity: Entity) -> Uuid {
        let id = entity.id;
        self.entities.insert(id, Arc::new(entity));
        self.entity_by_name
            .insert(entity.name.clone(), id);
        id
    }

    pub fn get_entity(&self, id: &Uuid) -> Option<Arc<Entity>> {
        self.entities.get(id).cloned()
    }

    pub fn get_entity_by_name(&self, name: &str) -> Option<Arc<Entity>> {
        if let Some(&id) = self.entity_by_name.get(name) {
            self.entities.get(&id).cloned()
        } else {
            None
        }
    }

    pub fn remove_entity(&mut self, id: &Uuid) {
        if let Some(entity) = self.entities.get(id) {
            for component in &entity.components {
                self.components.remove(&component.id);
            }
            if let Some(parent_id) = entity.parent_id {
                self.parent_map
                    .get_mut(&parent_id)
                    .map(|children| children.retain(|&c| c != *id));
            }
            self.entities.remove(id);
            self.entity_by_name.remove(&entity.name);
        }
    }

    pub fn add_component(&mut self, entity_id: Uuid, component: Component) {
        let id = component.id;
        self.components.insert(id, Arc::new(component));
        if let Some(entity) = self.entities.get(&entity_id) {
            entity.components.push(Arc::new(component));
        }
    }

    pub fn remove_component(&mut self, component_id: &Uuid) {
        if let Some(component) = self.components.get(component_id) {
            if let Some(entity) = self.entities.get(&component.entity_id) {
                entity.components.retain(|c| c.id != *component_id);
            }
            self.components.remove(component_id);
        }
    }

    pub fn get_components(&self, entity_id: &Uuid) -> Vec<Arc<Component>> {
        if let Some(entity) = self.entities.get(entity_id) {
            entity.components.clone()
        } else {
            Vec::new()
        }
    }

    pub fn get_parent(&self, entity_id: &Uuid) -> Option<Arc<Entity>> {
        if let Some(entity) = self.entities.get(entity_id) {
            entity.parent_id.and_then(|parent_id| self.entities.get(&parent_id).cloned())
        } else {
            None
        }
    }

    pub fn get_children(&self, parent_id: &Uuid) -> Vec<Arc<Entity>> {
        self.parent_map
            .get(parent_id)
            .map(|children| children.iter().filter_map(|&id| self.entities.get(&id).cloned()).collect())
            .unwrap_or_default()
    }

    pub fn set_parent(&mut self, entity_id: Uuid, parent_id: Option<Uuid>) {
        if let Some(entity) = self.entities.get_mut(&entity_id) {
            entity.parent_id = parent_id;
            if let Some(parent) = parent_id.and_then(|id| self.entities.get(&id)) {
                self.parent_map
                    .entry(parent.id)
                    .or_default()
                    .push(entity_id);
            }
        }
    }

    pub fn update(&mut self, delta: f64) {
        for component in self.components.values() {
            // Implementar lógica de actualización
        }
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}
