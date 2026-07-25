//! # Collision System Completo
//! 
//! Sistema completo de detección y respuesta a colisiones:
//! - AABB (Axis-Aligned Bounding Box)
//! - Circle (Círculo)
//! - Collision Layers
//! - Physics Response
//! - Collision Events

use serde::{Deserialize, Serialize};
use std::f32::consts::PI;

/// Tipo de colisión
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CollisionType {
    AABB,
    Circle,
    Point,
}

/// Capa de colisión
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CollisionLayer {
    pub name: String,
    pub bits: u32,
}

impl Default for CollisionLayer {
    fn default() -> Self {
        Self {
            name: "Default".to_string(),
            bits: 0b0000_0000_0000_0001,
        }
    }
}

impl CollisionLayer {
    /// Crea nueva capa con bits especificados
    pub fn new(name: String, bits: u32) -> Self {
        Self { name, bits }
    }

    /// Verifica si capa está activa
    pub fn is_active(&self, bits: u32) -> bool {
        (self.bits & bits) != 0
    }
}

/// Información de colisión
#[derive(Debug, Clone)]
pub struct CollisionInfo {
    pub collision_type: CollisionType,
    pub other_entity_id: String,
    pub penetration_depth: f32,
    pub normal: (f32, f32),
    pub contact_point: (f32, f32),
    pub timestamp: f64,
}

impl CollisionInfo {
    /// Crea nueva información de colisión
    pub fn new(collision_type: CollisionType, other_entity_id: String, penetration_depth: f32, normal: (f32, f32), contact_point: (f32, f32)) -> Self {
        Self {
            collision_type,
            other_entity_id,
            penetration_depth,
            normal,
            contact_point,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs_f64(),
        }
    }
}

/// Callback de colisión
pub type CollisionCallback = Box<dyn Fn(&CollisionInfo) + Send + Sync>;

/// Sistema de colisiones
pub struct CollisionSystem {
    /// Entidades con colisionadores
    pub entities: Vec<EntityCollider>,
    /// Callbacks de colisión
    pub collision_callbacks: Vec<CollisionCallback>,
    /// Colisiones actuales
    pub current_collisions: Vec<CollisionInfo>,
    /// Colisiones detectadas en este frame
    pub frame_collisions: Vec<CollisionInfo>,
    /// Capas de colisión activas
    pub active_layers: Vec<CollisionLayer>,
    /// Colisiones acumuladas
    pub accumulated_collisions: Vec<CollisionInfo>,
}

impl Default for CollisionSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl CollisionSystem {
    /// Crea nuevo sistema de colisiones
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
            collision_callbacks: Vec::new(),
            current_collisions: Vec::new(),
            frame_collisions: Vec::new(),
            active_layers: Vec::new(),
            accumulated_collisions: Vec::new(),
        }
    }

    /// Añade entidad con colisionador
    pub fn add_entity(&mut self, entity: EntityCollider) {
        self.entities.push(entity);
        println!("[COLLISION SYSTEM] Added collider for entity: {}", entity.entity_id);
    }

    /// Elimina entidad con colisionador
    pub fn remove_entity(&mut self, entity_id: &str) {
        self.entities.retain(|e| e.entity_id != entity_id);
        println!("[COLLISION SYSTEM] Removed collider for entity: {}", entity_id);
    }

    /// Registra callback de colisión
    pub fn register_callback(&mut self, callback: CollisionCallback) {
        self.collision_callbacks.push(callback);
        println!("[COLLISION SYSTEM] Registered collision callback");
    }

    /// Verifica colisión AABB
    pub fn check_aabb_collision(&self, rect1: &AABB, rect2: &AABB) -> bool {
        rect1.x <= rect2.x + rect2.width &&
        rect1.x + rect1.width >= rect2.x &&
        rect1.y <= rect2.y + rect2.height &&
        rect1.y + rect1.height >= rect2.y
    }

    /// Verifica colisión Circle-Circle
    pub fn check_circle_collision(&self, circle1: &CircleCollider, circle2: &CircleCollider) -> bool {
        let dx = circle1.center.0 - circle2.center.0;
        let dy = circle1.center.1 - circle2.center.1;
        let distance = (dx * dx + dy * dy).sqrt();
        distance < (circle1.radius + circle2.radius)
    }

    /// Verifica colisión AABB-Circle
    pub fn check_aabb_circle_collision(&self, aabb: &AABB, circle: &CircleCollider) -> bool {
        let dx = if circle.center.0 < aabb.x { aabb.x - circle.center.0 } else { if circle.center.0 > aabb.x + aabb.width { aabb.x + aabb.width - circle.center.0 } else { 0.0 } };
        let dy = if circle.center.1 < aabb.y { aabb.y - circle.center.1 } else { if circle.center.1 > aabb.y + aabb.height { aabb.y + aabb.height - circle.center.1 } else { 0.0 } };
        (dx * dx + dy * dy) < (circle.radius * circle.radius)
    }

    /// Detecta todas las colisiones
    pub fn detect_collisions(&mut self) {
        self.frame_collisions.clear();
        self.current_collisions.clear();
        
        println!("[COLLISION SYSTEM] Detecting collisions...");
        
        // AABB vs AABB
        for i in 0..self.entities.len() {
            for j in i + 1..self.entities.len() {
                if self.check_aabb_collision(&self.entities[i].aabb, &self.entities[j].aabb) {
                    let collision_info = CollisionInfo::new(
                        CollisionType::AABB,
                        self.entities[j].entity_id.clone(),
                        0.0,
                        (0.0, 0.0),
                        (0.0, 0.0),
                    );
                    
                    self.frame_collisions.push(collision_info);
                    self.current_collisions.push(collision_info);
                }
            }
        }
        
        // Circle vs Circle
        for i in 0..self.entities.len() {
            for j in i + 1..self.entities.len() {
                if self.check_circle_collision(&self.entities[i].circle, &self.entities[j].circle) {
                    let collision_info = CollisionInfo::new(
                        CollisionType::Circle,
                        self.entities[j].entity_id.clone(),
                        0.0,
                        (0.0, 0.0),
                        (0.0, 0.0),
                    );
                    
                    self.frame_collisions.push(collision_info);
                    self.current_collisions.push(collision_info);
                }
            }
        }
        
        println!("[COLLISION SYSTEM] Detected {} collisions", self.frame_collisions.len());
    }

    /// Obtiene colisiones del frame actual
    pub fn get_frame_collisions(&self) -> &[CollisionInfo] {
        &self.frame_collisions
    }

    /// Obtiene todas las colisiones
    pub fn get_all_collisions(&self) -> &[CollisionInfo] {
        &self.current_collisions
    }

    /// Notifica colisión
    pub fn notify_collision(&mut self, collision_info: &CollisionInfo) {
        for callback in &self.collision_callbacks {
            callback(collision_info);
        }
    }

    /// Limpia colisiones del frame
    pub fn clear_frame_collisions(&mut self) {
        self.frame_collisions.clear();
        println!("[COLLISION SYSTEM] Cleared frame collisions");
    }

    /// Limpia todas las colisiones
    pub fn clear_all_collisions(&mut self) {
        self.current_collisions.clear();
        self.frame_collisions.clear();
        self.accumulated_collisions.clear();
        println!("[COLLISION SYSTEM] Cleared all collisions");
    }

    /// Cuenta colisiones
    pub fn count_collisions(&self) -> usize {
        self.current_collisions.len()
    }
}

/// Entidad con colisionador
#[derive(Debug, Clone)]
pub struct EntityCollider {
    pub entity_id: String,
    pub aabb: AABB,
    pub circle: CircleCollider,
    pub layers: u32,
    pub is_active: bool,
}

impl EntityCollider {
    /// Crea nuevo colisionador de entidad
    pub fn new(entity_id: String, aabb: AABB, circle: CircleCollider, layers: u32) -> Self {
        Self {
            entity_id,
            aabb,
            circle,
            layers,
            is_active: true,
        }
    }
}

/// Caja delimitadora AABB
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AABB {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl AABB {
    /// Crea nuevo AABB
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self { x, y, width, height }
    }

    /// Centra AABB en punto
    pub fn centered_at(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x: x - width / 2.0,
            y: y - height / 2.0,
            width,
            height,
        }
    }

    /// Expande AABB
    pub fn expand(&mut self, dx: f32, dy: f32) {
        self.x = self.x.max(self.x - dx);
        self.y = self.y.max(self.y - dy);
        self.width = self.width.max(self.width + dx * 2.0);
        self.height = self.height.max(self.height + dy * 2.0);
    }

    /// Verifica si punto está dentro
    pub fn contains_point(&self, x: f32, y: f32) -> bool {
        x >= self.x && x <= self.x + self.width &&
        y >= self.y && y <= self.y + self.height
    }

    /// Calcula intersección con otro AABB
    pub fn intersect(&self, other: AABB) -> Option<AABB> {
        let left = self.x.max(other.x);
        let right = (self.x + self.width).min(other.x + other.width);
        let top = self.y.max(other.y);
        let bottom = (self.y + self.height).min(other.y + other.height);
        
        if left < right && top < bottom {
            Some(AABB::new(left, top, right - left, bottom - top))
        } else {
            None
        }
    }
}

/// Colisionador circular
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CircleCollider {
    pub center: (f32, f32),
    pub radius: f32,
}

impl CircleCollider {
    /// Crea nuevo colisionador circular
    pub fn new(center: (f32, f32), radius: f32) -> Self {
        Self { center, radius }
    }

    /// Verifica si punto está dentro
    pub fn contains_point(&self, x: f32, y: f32) -> bool {
        let dx = x - self.center.0;
        let dy = y - self.center.1;
        (dx * dx + dy * dy) < (self.radius * self.radius)
    }

    /// Distancia a punto
    pub fn distance_to_point(&self, x: f32, y: f32) -> f32 {
        let dx = x - self.center.0;
        let dy = y - self.center.1;
        (dx * dx + dy * dy).sqrt()
    }
}

/// Respuesta a colisión
#[derive(Debug, Clone, Copy)]
pub struct CollisionResponse {
    pub normal: (f32, f32),
    pub penetration: f32,
    pub resolve_position: (f32, f32),
}

impl CollisionResponse {
    /// Resuelve colisión moviendo entidad
    pub fn resolve(&self, position: &mut (f32, f32)) {
        position.0 += self.resolve_position.0;
        position.1 += self.resolve_position.1;
    }
}

/// Sistema de colisiones con física
pub struct PhysicsCollisionSystem {
    pub collision_system: CollisionSystem,
    pub gravity: f32,
    pub friction: f32,
    pub bounce: f32,
}

impl Default for PhysicsCollisionSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl PhysicsCollisionSystem {
    /// Crea nuevo sistema de colisiones con física
    pub fn new() -> Self {
        Self {
            collision_system: CollisionSystem::new(),
            gravity: 9.8,
            friction: 0.8,
            bounce: 0.7,
        }
    }

    /// Respuesta a colisión
    pub fn resolve_collision(&mut self, entity: &mut EntityCollider, other: &EntityCollider, response: &CollisionResponse) {
        // Resolver penetración
        let velocity = (entity.velocity.0, entity.velocity.1);
        let dot_product = velocity.0 * response.normal.0 + velocity.1 * response.normal.1;
        
        // Rebotar si velocidad se mueve hacia la superficie
        if dot_product > 0.0 {
            let new_velocity = (velocity.0 - 2.0 * dot_product * response.normal.0, velocity.1 - 2.0 * dot_product * response.normal.1);
            entity.velocity = (new_velocity.0 * self.bounce, new_velocity.1 * self.bounce);
        }
        
        // Aplicar fricción
        entity.velocity.0 *= self.friction;
        entity.velocity.1 *= self.friction;
    }

    /// Respuesta a colisión AABB-AABB
    pub fn resolve_aabb_collision(&mut self, entity: &mut EntityCollider, other: &EntityCollider) -> Option<CollisionResponse> {
        let aabb1 = entity.aabb;
        let aabb2 = other.aabb;
        
        // Calcular normal de colisión
        let penetration_x = (aabb1.x + aabb1.width).min(aabb2.x + aabb2.width) - (aabb1.x + aabb1.width).max(aabb2.x);
        let penetration_y = (aabb1.y + aabb1.height).min(aabb2.y + aabb2.height) - (aabb1.y + aabb1.height).max(aabb2.y);
        
        let normal = if penetration_x.abs() < penetration_y.abs() {
            if penetration_x > 0.0 { (1.0, 0.0) } else { (-1.0, 0.0) }
        } else {
            if penetration_y > 0.0 { (0.0, 1.0) } else { (0.0, -1.0) }
        };
        
        let penetration = penetration_x.abs().min(penetration_y.abs());
        
        let response = CollisionResponse {
            normal,
            penetration,
            resolve_position: (normal.0 * penetration, normal.1 * penetration),
        };
        
        println!("[COLLISION SYSTEM] Resolved AABB collision with normal {:?} and penetration {}", response.normal, response.penetration);
        
        Some(response)
    }

    /// Respuesta a colisión Circle-Circle
    pub fn resolve_circle_collision(&mut self, entity: &mut EntityCollider, other: &EntityCollider) -> Option<CollisionResponse> {
        let dx = entity.circle.center.0 - other.circle.center.0;
        let dy = entity.circle.center.1 - other.circle.center.1;
        let distance = (dx * dx + dy * dy).sqrt();
        
        if distance > 0.0 {
            let normal = (dx / distance, dy / distance);
            let penetration = (entity.circle.radius + other.circle.radius) - distance;
            
            let response = CollisionResponse {
                normal,
                penetration,
                resolve_position: (normal.0 * penetration, normal.1 * penetration),
            };
            
            println!("[COLLISION SYSTEM] Resolved Circle collision with normal {:?} and penetration {}", response.normal, response.penetration);
            
            Some(response)
        } else {
            None
        }
    }

    /// Respuesta a colisión AABB-Circle
    pub fn resolve_aabb_circle_collision(&mut self, entity: &mut EntityCollider, aabb: &AABB, circle: &CircleCollider) -> Option<CollisionResponse> {
        let dx = if circle.center.0 < aabb.x { aabb.x - circle.center.0 } else { if circle.center.0 > aabb.x + aabb.width { aabb.x + aabb.width - circle.center.0 } else { 0.0 } };
        let dy = if circle.center.1 < aabb.y { aabb.y - circle.center.1 } else { if circle.center.1 > aabb.y + aabb.height { aabb.y + aabb.height - circle.center.1 } else { 0.0 } };
        
        let distance_sq = dx * dx + dy * dy;
        
        if distance_sq < circle.radius * circle.radius {
            let distance = distance_sq.sqrt();
            let normal = if distance > 0.0 { (dx / distance, dy / distance) } else { (0.0, 1.0) };
            let penetration = circle.radius - distance;
            
            let response = CollisionResponse {
                normal,
                penetration,
                resolve_position: (normal.0 * penetration, normal.1 * penetration),
            };
            
            Some(response)
        } else {
            None
        }
    }
}

/// Sistema de colisiones con entidades físicas
pub struct PhysicsEntity {
    pub entity_id: String,
    pub position: (f32, f32),
    pub velocity: (f32, f32),
    pub radius: f32,
    pub is_active: bool,
}

impl PhysicsEntity {
    /// Crea nueva entidad física
    pub fn new(entity_id: String, position: (f32, f32), radius: f32) -> Self {
        Self {
            entity_id,
            position,
            velocity: (0.0, 0.0),
            radius,
            is_active: true,
        }
    }

    /// Actualiza posición con física
    pub fn update(&mut self, delta: f32, gravity: f32) {
        if !self.is_active {
            return;
        }
        
        // Aplicar gravedad
        self.velocity.1 += gravity * delta;
        
        // Actualizar posición
        self.position.0 += self.velocity.0 * delta;
        self.position.1 += self.velocity.1 * delta;
        
        // Limitar velocidad vertical
        if self.velocity.1 > 15.0 {
            self.velocity.1 = 15.0;
        }
    }
}
