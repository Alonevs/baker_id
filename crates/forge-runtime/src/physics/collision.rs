//! Collision detection and resolution

use crate::physics::{CollisionBrush, PhysicsProperties};

/// Collision detection result
#[derive(Clone)]
pub struct CollisionResult {
    pub collided: bool,
    pub penetration_depth: f32,
    pub normal: (f32, f32),
    pub contact_point: (f32, f32),
}

impl Default for CollisionResult {
    fn default() -> Self {
        Self {
            collided: false,
            penetration_depth: 0.0,
            normal: (0.0, 0.0),
            contact_point: (0.0, 0.0),
        }
    }
}

/// AABB collision detection
pub fn check_aabb_collision(
    rect1: (f32, f32, f32, f32), // x, y, w, h
    rect2: (f32, f32, f32, f32),
) -> CollisionResult {
    let (x1, y1, w1, h1) = rect1;
    let (x2, y2, w2, h2) = rect2;
    
    if x1 + w1 < x2 || x2 + w2 < x1 || y1 + h1 < y2 || y2 + h2 < y1 {
        return CollisionResult::default();
    }
    
    let overlap_x = (w1 + w2).min(f32::MAX) - (x1 + x2).max(f32::MIN) + x1;
    let overlap_y = (h1 + h2).min(f32::MAX) - (y1 + y2).max(f32::MIN) + y1;
    
    let depth = overlap_x.min(overlap_y);
    let normal = if overlap_x < overlap_y {
        (1.0, 0.0)
    } else {
        (0.0, 1.0)
    };
    
    CollisionResult {
        collided: true,
        penetration_depth: depth,
        normal,
        contact_point: (
            (x1 + x2 + (w1 + w2)) / 2.0,
            (y1 + y2 + (h1 + h2)) / 2.0,
        ),
    }
}

/// Polygon collision detection
pub fn check_polygon_collision(
    point: (f32, f32),
    polygon: &[(f32, f32)],
) -> CollisionResult {
    let mut collided = false;
    let mut depth = f32::MAX;
    let mut normal = (0.0, 0.0);
    let mut contact = point;
    
    for i in 0..polygon.len() {
        let p1 = polygon[i];
        let p2 = polygon[(i + 1) % polygon.len()];
        
        if is_point_inside_polygon(point, polygon) {
            collided = true;
            break;
        }
        
        if point_in_line_halfplane(point, p1, p2) {
            collided = true;
            let dist = point_to_line_distance(point, p1, p2);
            if dist < depth {
                depth = dist;
                normal = line_normal(p1, p2);
                contact = point;
            }
        }
    }
    
    CollisionResult {
        collided,
        penetration_depth: depth,
        normal,
        contact_point: contact,
    }
}

fn is_point_inside_polygon(point: (f32, f32), polygon: &[(f32, f32)]) -> bool {
    let mut inside = false;
    for i in 0..polygon.len() {
        let p1 = polygon[i];
        let p2 = polygon[(i + 1) % polygon.len()];
        
        if ((p1.1 <= point.1 && point.1 < p2.1) || (p2.1 <= point.1 && point.1 < p1.1))
            && point.0 < (p2.0 - p1.0) * (point.1 - p1.1) / (p2.1 - p1.1) + p1.0
        {
            inside = !inside;
        }
    }
    inside
}

fn point_in_line_halfplane(point: (f32, f32), p1: (f32, f32), p2: (f32, f32)) -> bool {
    let cross = (p2.0 - p1.0) * (point.1 - p1.1) - (p2.1 - p1.1) * (point.0 - p1.0);
    cross >= 0.0
}

fn point_to_line_distance(point: (f32, f32), p1: (f32, f32), p2: (f32, f32)) -> f32 {
    let dist_sq = (p2.0 - p1.0).powi(2) + (p2.1 - p1.1).powi(2);
    if dist_sq < 0.0001 { return (point.0 - p1.0).powi(2) + (point.1 - p1.1).powi(2).sqrt(); }
    
    let t = ((point.0 - p1.0) * (p2.0 - p1.0) + (point.1 - p1.1) * (p2.1 - p1.1)) / dist_sq;
    let closest_x = p1.0 + t * (p2.0 - p1.0);
    let closest_y = p1.1 + t * (p2.1 - p1.1);
    
    ((point.0 - closest_x).powi(2) + (point.1 - closest_y).powi(2)).sqrt()
}

fn line_normal(p1: (f32, f32), p2: (f32, f32)) -> (f32, f32) {
    let dx = p2.0 - p1.0;
    let dy = p2.1 - p1.1;
    let len = (dx * dx + dy * dy).sqrt();
    if len < 0.0001 { return (0.0, 0.0); }
    (-dy / len, dx / len)
}

/// Collision brush painter for map
pub struct CollisionBrushPainter {
    pub brushes: Vec<CollisionBrush>,
    pub current_brush: CollisionBrush,
    pub brush_size: f32,
}

impl CollisionBrushPainter {
    pub fn new() -> Self {
        Self {
            brushes: vec![CollisionBrush::Solid, CollisionBrush::OneWay, CollisionBrush::Ramp],
            current_brush: CollisionBrush::Solid,
            brush_size: 16.0,
        }
    }
    
    pub fn set_brush(&mut self, brush: CollisionBrush) {
        self.current_brush = brush;
    }
    
    pub fn paint_pixel(&self, _x: f32, _y: f32) -> CollisionBrush {
        self.current_brush
    }
    
    pub fn get_brush_type(&self, brush: CollisionBrush) -> &'static str {
        match brush {
            CollisionBrush::Solid => "Solid (Red)",
            CollisionBrush::OneWay => "One-Way (Orange)",
            CollisionBrush::Ramp => "Ramp (Blue)",
        }
    }
}

/// Physics resolution for entity movement
pub fn resolve_collision(
    _entity: &mut crate::entities::Entity,
    _physics: &mut PhysicsProperties,
    result: &CollisionResult,
) {
    if result.collided && result.penetration_depth > 0.0 {
        // Resolution logic
    }
}
