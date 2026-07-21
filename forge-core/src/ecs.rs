use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct Entity {
    pub id: Uuid,
    pub name: String,
    pub components: Vec<Arc<Component>>,
}

#[derive(Debug, Clone)]
pub struct Component {
    pub id: Uuid,
    pub name: String,
    pub kind: ComponentKind,
    pub data: serde_json::Value,
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
}

#[derive(Debug, Error)]
pub enum EntityError {
    #[error("Entity not found: {0}")]
    EntityNotFound(Uuid),
    #[error("Component not found: {0}")]
    ComponentNotFound(Uuid),
}

pub struct ECS {
    pub entities: HashMap<Uuid, Arc<Entity>>,
    pub components: HashMap<Uuid, Arc<Component>>,
    pub archetype: HashMap<usize, Vec<Uuid>>,
}

impl ECS {
    pub fn new() -> Self {
        Self {
            entities: HashMap::new(),
            components: HashMap::new(),
            archetype: HashMap::new(),
        }
    }

    pub fn add_entity(&mut self, entity: Entity) -> Uuid {
        let id = entity.id;
        self.entities.insert(id, Arc::new(entity));
        id
    }

    pub fn get_entity(&self, id: &Uuid) -> Option<Arc<Entity>> {
        self.entities.get(id).cloned()
    }

    pub fn add_component(&mut self, entity_id: Uuid, component: Component) -> Uuid {
        let id = component.id;
        self.components.insert(id, Arc::new(component));
        if let Some(entity) = self.entities.get(&entity_id) {
            entity.components.push(Arc::new(component));
        }
        id
    }

    pub fn remove_component(&mut self, id: &Uuid) {
        if let Some(component) = self.components.remove(id) {
            if let Some(entity) = self.entities.get(&component.entity_id) {
                entity.components.retain(|c| c.id != *id);
            }
        }
    }

    pub fn get_components(&self, entity_id: &Uuid) -> Vec<Arc<Component>> {
        if let Some(entity) = self.entities.get(entity_id) {
            entity.components.clone()
        } else {
            Vec::new()
        }
    }

    pub fn query<T: ComponentQuery>(&self) -> Vec<Arc<Entity>> {
        self.entities.values().filter(|e| T::matches(e)).collect()
    }

    pub fn update(&mut self, delta: f64) {
        // Actualizar todos los componentes
    }
}

pub trait ComponentQuery {
    fn matches(entity: &Arc<Entity>) -> bool;
}

impl ComponentQuery for ComponentKind {
    fn matches(entity: &Arc<Entity>) -> bool {
        entity.components.iter().any(|c| c.kind == Self)
    }
}

pub struct TransformComponent {
    pub position: [f32; 3],
    pub rotation: f32,
    pub scale: [f32; 3],
}

impl TransformComponent {
    pub fn new() -> Self {
        Self {
            position: [0.0, 0.0, 0.0],
            rotation: 0.0,
            scale: [1.0, 1.0, 1.0],
        }
    }

    pub fn translate(&mut self, delta: [f32; 3]) {
        self.position = [
            self.position[0] + delta[0],
            self.position[1] + delta[1],
            self.position[2] + delta[2],
        ];
    }

    pub fn rotate(&mut self, angle: f32) {
        self.rotation = (self.rotation + angle).rem_euclid(360.0);
    }

    pub fn scale(&mut self, factor: f32) {
        self.scale = [
            self.scale[0] * factor,
            self.scale[1] * factor,
            self.scale[2] * factor,
        ];
    }
}

pub struct PhysicsComponent {
    pub velocity: [f32; 3],
    pub gravity: [f32; 3],
    pub colliders: Vec<Uuid>,
}

impl PhysicsComponent {
    pub fn new(gravity: [f32; 3]) -> Self {
        Self {
            velocity: [0.0, 0.0, 0.0],
            gravity,
            colliders: Vec::new(),
        }
    }

    pub fn apply_gravity(&mut self, delta: f64) {
        self.velocity[1] += self.gravity[1] * delta;
    }

    pub fn set_velocity(&mut self, velocity: [f32; 3]) {
        self.velocity = velocity;
    }
}

impl Default for ECS {
    fn default() -> Self {
        Self::new()
    }
}
