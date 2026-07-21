use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;
use serde::{Serialize, Deserialize};

/// Representa un bloque físico en el viewport 2D
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicsBlock {
    pub id: String,
    pub position: Vector2,
    pub size: Vector2,
    pub mass: f32,
    pub velocity: Vector2,
    pub friction: f32,
    pub restitution: f32,
    pub is_static: bool,
    pub collider_type: ColliderType,
}

/// Vector 2D para posiciones, velocidades, etc.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    
    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

/// Tipos de colisionadores
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ColliderType {
    Circle,
    Rectangle,
}

/// Motor de física 2D
#[derive(Debug, Clone)]
pub struct Physics2D {
    pub blocks: Arc<RwLock<HashMap<String, PhysicsBlock>>>,
    pub gravity: Vector2,
    pub dt: f32,
    pub step_size: f32,
}

impl Default for Physics2D {
    fn default() -> Self {
        Self {
            blocks: Arc::new(RwLock::new(HashMap::new())),
            gravity: Vector2::new(0.0, 9.8),
            dt: 1.0 / 60.0,
            step_size: 1.0 / 60.0,
        }
    }
}

impl Physics2D {
    /// Crear un bloque físico
    pub fn create_block(&self, id: &str, position: Vector2, size: Vector2, mass: f32) -> PhysicsBlock {
        PhysicsBlock {
            id: id.to_string(),
            position,
            size,
            mass,
            velocity: Vector2::zero(),
            friction: 0.5,
            restitution: 0.7,
            is_static: false,
            collider_type: ColliderType::Rectangle,
        }
    }

    /// Obtener referencia a un bloque
    pub fn get_block(&self, id: &str) -> Option<PhysicsBlock> {
        self.blocks.read().unwrap().get(id).cloned()
    }

    /// Obtener referencia mutada a un bloque
    pub fn get_block_mut(&mut self, id: &str) -> Option<PhysicsBlock> {
        self.blocks.write().unwrap().get_mut(id).cloned()
    }

    /// Añadir bloque
    pub fn add_block(&mut self, block: PhysicsBlock) {
        let id = block.id.clone();
        self.blocks.write().unwrap().insert(id, block);
    }

    /// Remover bloque
    pub fn remove_block(&mut self, id: &str) -> bool {
        self.blocks.write().unwrap().remove(id).is_some()
    }

    /// Resolver colisiones entre bloques
    pub fn resolve_collisions(&mut self, ids: &[String], blocks_vec: &[PhysicsBlock]) {
        for i in 0..ids.len() {
            for j in (i + 1)..ids.len() {
                let block_a = &blocks_vec[i];
                let block_b = &blocks_vec[j];
                let block_a_id = ids[i].clone();
                let block_b_id = ids[j].clone();
                
                // Verificar colisión
                let a_left = block_a.position.x;
                let a_right = block_a.position.x + block_a.size.x;
                let a_top = block_a.position.y;
                let a_bottom = block_a.position.y + block_a.size.y;

                let b_left = block_b.position.x;
                let b_right = block_b.position.x + block_b.size.x;
                let b_top = block_b.position.y;
                let b_bottom = block_b.position.y + block_b.size.y;

                if a_left < b_right && a_right > b_left && a_top < b_bottom && a_bottom > b_top {
                    // Segunda pasada: resolver colisión
                    let min_overlap_x = (a_right - b_left).min(b_right - a_left);
                    let min_overlap_y = (a_bottom - b_top).min(b_bottom - a_top);
                    
                    let mut blocks = self.blocks.write().unwrap();
                    
                    if min_overlap_x < min_overlap_y {
                        // Colisión horizontal
                        let a_x = blocks.get_mut(&block_a_id).unwrap().position.x;
                        let b_x = blocks.get_mut(&block_b_id).unwrap().position.x;
                        if a_x + block_a.size.x < b_x {
                            blocks.get_mut(&block_a_id).unwrap().position.x += min_overlap_x;
                        } else {
                            blocks.get_mut(&block_b_id).unwrap().position.x -= min_overlap_x;
                        }
                    } else {
                        // Colisión vertical
                        let a_y = blocks.get_mut(&block_a_id).unwrap().position.y;
                        let b_y = blocks.get_mut(&block_b_id).unwrap().position.y;
                        if a_y + block_a.size.y < b_y {
                            blocks.get_mut(&block_a_id).unwrap().position.y += min_overlap_y;
                        } else {
                            blocks.get_mut(&block_b_id).unwrap().position.y -= min_overlap_y;
                        }
                    }
                    
                    drop(blocks);
                }
            }
        }
    }

    /// Verificar colisión entre dos bloques
    fn _check_collision(&self, a: &PhysicsBlock, b: &PhysicsBlock) -> Option<CollisionResult> {
        // Verificar superposición
        let a_left = a.position.x;
        let a_right = a.position.x + a.size.x;
        let a_top = a.position.y;
        let a_bottom = a.position.y + a.size.y;

        let b_left = b.position.x;
        let b_right = b.position.x + b.size.x;
        let b_top = b.position.y;
        let b_bottom = b.position.y + b.size.y;

        // No hay superposición
        if a_right <= b_left || a_left >= b_right || a_bottom <= b_top || a_top >= b_bottom {
            return None;
        }

        // Calcular dirección de colisión
        let overlap_x = (a_right - b_left).min(b_right - a_left);
        let overlap_y = (a_bottom - b_top).min(b_bottom - a_top);

        Some(CollisionResult {
            overlap_x,
            overlap_y,
            normal_x: if overlap_x < overlap_y { 1.0 } else { 0.0 },
            normal_y: if overlap_x < overlap_y { 0.0 } else { 1.0 },
        })
    }

    /// Resolver colisión
    fn _resolve_collision(&mut self, a: &PhysicsBlock, b: &PhysicsBlock, collision: CollisionResult) {
        let mut blocks = self.blocks.write().unwrap();

        if collision.normal_x != 0.0 {
            // Colisión horizontal
            let penetration = collision.overlap_x;
            
            if a.mass < b.mass {
                // Mover bloque A
                let vel_x = -collision.normal_x * penetration * 0.5;
                if let Some(block_a) = blocks.get_mut(&a.id) {
                    block_a.velocity.x += vel_x;
                }
            } else {
                // Mover bloque B
                let vel_x = collision.normal_x * penetration * 0.5;
                if let Some(block_b) = blocks.get_mut(&b.id) {
                    block_b.velocity.x += vel_x;
                }
            }
        } else {
            // Colisión vertical
            let penetration = collision.overlap_y;
            
            if a.mass < b.mass {
                // Mover bloque A
                let vel_y = -collision.normal_y * penetration * 0.5;
                if let Some(block_a) = blocks.get_mut(&a.id) {
                    block_a.velocity.y += vel_y;
                }
            } else {
                // Mover bloque B
                let vel_y = collision.normal_y * penetration * 0.5;
                if let Some(block_b) = blocks.get_mut(&b.id) {
                    block_b.velocity.y += vel_y;
                }
            }
        }
    }

    /// Aplicar fuerzas a un bloque
    pub fn apply_force(&mut self, id: &str, force: Vector2) {
        if let Some(block) = self.blocks.write().unwrap().get_mut(id) {
            if !block.is_static {
                let acceleration_x = force.x / block.mass;
                let acceleration_y = force.y / block.mass;
                block.velocity.x += acceleration_x * self.dt;
                block.velocity.y += acceleration_y * self.dt;
            }
        }
    }

    /// Aplicar gravedad
    pub fn apply_gravity(&mut self) {
        let mut blocks = self.blocks.write().unwrap();
        let gravity_y = self.gravity.y;
        let dt = self.dt;
        
        for block in blocks.values_mut() {
            if !block.is_static {
                block.velocity.y += gravity_y * dt;
            }
        }
    }

    /// Actualizar posiciones
    pub fn update(&mut self) {
        let dt = self.dt;
        let friction = 0.99;
        let _gravity_y = self.gravity.y;
        let blocks = self.blocks.read().unwrap();
        let ids: Vec<String> = blocks.keys().map(|k| k.clone()).collect();
        let mut blocks_vec: Vec<PhysicsBlock> = blocks.values().map(|b| b.clone()).collect();
        drop(blocks);
        
        for block in blocks_vec.iter_mut() {
            if !block.is_static {
                block.position.x += block.velocity.x * dt;
                block.position.y += block.velocity.y * dt;
                
                // Aplicar fricción
                block.velocity.x *= friction;
            }
        }
        
        self.resolve_collisions(&ids, &blocks_vec);
    }

    /// Exportar a formato .map
    pub fn export_to_map(&self) -> String {
        let blocks = self.blocks.read().unwrap();
        let mut content = String::new();
        
        content.push_str("PHYSICS_DATA\n");
        content.push_str(format!("BLOCK_COUNT: {}\n", blocks.len()).as_str());
        
        for (id, block) in blocks.iter() {
            content.push_str(&format!(
                "BLOCK:{},{},{},{},{},{},{},{},{}\n",
                id,
                block.position.x,
                block.position.y,
                block.size.x,
                block.size.y,
                block.mass,
                block.velocity.x,
                block.velocity.y,
                block.is_static
            ));
        }
        
        content
    }

    /// Importar desde formato .map
    pub fn import_from_map(&mut self, content: &str) {
        let lines: Vec<&str> = content.lines().collect();
        
        for line in lines {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() >= 8 && line.starts_with("BLOCK:") {
                let id = parts[1].trim().to_string();
                let x: f32 = parts[2].trim().parse().unwrap_or(0.0);
                let y: f32 = parts[3].trim().parse().unwrap_or(0.0);
                let width: f32 = parts[4].trim().parse().unwrap_or(100.0);
                let height: f32 = parts[5].trim().parse().unwrap_or(100.0);
                let mass: f32 = parts[6].trim().parse().unwrap_or(1.0);
                let vx: f32 = parts[7].trim().parse().unwrap_or(0.0);
                let vy: f32 = parts[8].trim().parse().unwrap_or(0.0);
                let is_static: bool = parts[9].trim() == "true";
                
                let block = PhysicsBlock {
                    id,
                    position: Vector2::new(x, y),
                    size: Vector2::new(width, height),
                    mass,
                    velocity: Vector2::new(vx, vy),
                    friction: 0.5,
                    restitution: 0.7,
                    is_static,
                    collider_type: ColliderType::Rectangle,
                };
                
                let id = block.id.clone();
                self.blocks.write().unwrap().insert(id, block);
            }
        }
    }
}

/// Resultado de una colisión
#[derive(Debug, Clone)]
pub struct CollisionResult {
    pub overlap_x: f32,
    pub overlap_y: f32,
    pub normal_x: f32,
    pub normal_y: f32,
}

