use uuid::Uuid;
use crate::animation::{InterpolationType, LoopMode};
use crate::keyframe::{Keyframe, Transform};

/// Interpolador avanzado para animaciones 2D
/// Proporciona interpolación con múltiples tipos de easing
pub struct AdvancedInterpolator {
    pub interpolation: InterpolationType,
    pub loop_mode: LoopMode,
    pub duration: f32,
}

impl Default for AdvancedInterpolator {
    fn default() -> Self {
        Self {
            interpolation: InterpolationType::Linear,
            loop_mode: LoopMode::Loop,
            duration: 1.0,
        }
    }
}

impl AdvancedInterpolator {
    pub fn new(interpolation: InterpolationType, loop_mode: LoopMode, duration: f32) -> Self {
        Self {
            interpolation,
            loop_mode,
            duration,
        }
    }

    pub fn get_normalized_time(&self, time: f32) -> f32 {
        let t = time % self.duration;
        match self.loop_mode {
            LoopMode::None => t,
            LoopMode::Loop => t,
            LoopMode::PingPong => {
                let half = self.duration / 2.0;
                if t <= half {
                    t
                } else {
                    self.duration - t
                }
            }
        }
    }

    pub fn interpolate_value(&self, a: f32, b: f32, time: f32) -> f32 {
        let t = self.get_normalized_time(time) / self.duration;
        self.interpolation.interpolate(a, b, t)
    }

    pub fn interpolate_position(&self, start_pos: [f32; 3], end_pos: [f32; 3], time: f32) -> [f32; 3] {
        let t = self.get_normalized_time(time) / self.duration;
        let interp_type = self.interpolation;

        [
            interp_type.interpolate(start_pos[0], end_pos[0], t),
            interp_type.interpolate(start_pos[1], end_pos[1], t),
            interp_type.interpolate(start_pos[2], end_pos[2], t),
        ]
    }

    pub fn interpolate_transform(&self, start: &Transform, end: &Transform, time: f32) -> Transform {
        let t = self.get_normalized_time(time) / self.duration;
        let interp_type = self.interpolation;

        Transform {
            position: self.interpolate_position(start.position, end.position, time),
            rotation: [
                interp_type.interpolate(start.rotation[0], end.rotation[0], t),
                interp_type.interpolate(start.rotation[1], end.rotation[1], t),
                interp_type.interpolate(start.rotation[2], end.rotation[2], t),
            ],
            scale: [
                interp_type.interpolate(start.scale[0], end.scale[0], t),
                interp_type.interpolate(start.scale[1], end.scale[1], t),
                interp_type.interpolate(start.scale[2], end.scale[2], t),
            ],
        }
    }

    pub fn interpolate_blend_weight(&self, current_weight: f32, target_weight: f32, time: f32) -> f32 {
        let t = self.get_normalized_time(time) / self.duration;
        self.interpolation.interpolate(current_weight, target_weight, t)
    }

    pub fn get_easing_function(&self) -> Box<dyn Fn(f32) -> f32 + Send + Sync> {
        match self.interpolation {
            InterpolationType::Linear => Box::new(|t| t),
            InterpolationType::EaseIn => Box::new(Self::ease_in),
            InterpolationType::EaseOut => Box::new(Self::ease_out),
            InterpolationType::EaseInOut => Box::new(Self::ease_in_out),
            InterpolationType::Step => Box::new(|t| if t >= 0.5 { 1.0 } else { 0.0 }),
        }
    }

    fn ease_in(t: f32) -> f32 {
        t * t
    }

    fn ease_out(t: f32) -> f32 {
        t * (2.0 - t)
    }

    fn ease_in_out(t: f32) -> f32 {
        if t < 0.5 {
            2.0 * t * t
        } else {
            -1.0 + (4.0 - 2.0 * t) * t
        }
    }
}

/// KeyframeEditor para crear y editar keyframes en tiempo real
pub struct KeyframeEditor {
    pub current_animation: Option<Uuid>,
    pub selected_keyframe: Option<usize>,
    pub current_time: f32,
    pub editing_target: Option<Uuid>,
}

impl Default for KeyframeEditor {
    fn default() -> Self {
        Self {
            current_animation: None,
            selected_keyframe: None,
            current_time: 0.0,
            editing_target: None,
        }
    }
}

impl KeyframeEditor {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_animation(&mut self, animation_id: Uuid) {
        self.current_animation = Some(animation_id);
        self.selected_keyframe = None;
        self.current_time = 0.0;
    }

    pub fn add_keyframe(&mut self, time: f32, target: Uuid, _transform: Transform, blend_weight: f32) {
        if let Some(_anim_id) = self.current_animation {
            // Aquí se llamaría a la función de la animación para añadir el keyframe
            // Por ahora, esto es una implementación básica
            println!("Adding keyframe at time {} for target {} with blend weight {}", time, target, blend_weight);
        }
    }

    pub fn remove_keyframe(&mut self, index: usize) {
        if let Some(_anim_id) = self.current_animation {
            println!("Removing keyframe {} from animation {}", index, _anim_id);
        }
    }

    pub fn move_keyframe(&mut self, from_index: usize, to_index: usize) {
        if let Some(_anim_id) = self.current_animation {
            println!("Moving keyframe {} to position {} in animation {}", from_index, to_index, _anim_id);
        }
    }

    pub fn update_keyframe(&mut self, index: usize, _transform: Transform) {
        if let Some(_anim_id) = self.current_animation {
            println!("Updating keyframe {} in animation {} with transform", index, _anim_id);
        }
    }

    pub fn set_keyframe_interpolation(&mut self, index: usize, interpolation: InterpolationType) {
        if let Some(_anim_id) = self.current_animation {
            println!("Setting interpolation {:?} for keyframe {} in animation {}", interpolation, index, _anim_id);
        }
    }

    pub fn get_selected_keyframe(&self) -> Option<&Keyframe> {
        // Esto requeriría acceso a la animación
        None
    }

    pub fn get_next_keyframe_time(&self) -> f32 {
        self.current_time + 0.1 // Default de 0.1 segundos
    }

    pub fn scrub_to_time(&mut self, time: f32) {
        self.current_time = time;
    }
}

/// TimelineManager para gestionar el timeline visual
pub struct TimelineManager {
    pub playhead: f32,
    pub zoom_level: f32, // En segundos por píxel
    pub visible_range: (f32, f32),
    pub selected_keys: Vec<usize>,
    pub dragging_key: Option<usize>,
}

impl Default for TimelineManager {
    fn default() -> Self {
        Self {
            playhead: 0.0,
            zoom_level: 1.0, // 1 segundo por píxel
            visible_range: (0.0, 10.0),
            selected_keys: Vec::new(),
            dragging_key: None,
        }
    }
}

impl TimelineManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_zoom(&mut self, zoom: f32) {
        self.zoom_level = zoom.clamp(0.1, 10.0);
    }

    pub fn get_visible_time_range(&self, width: u32) -> (f32, f32) {
        let range = width as f32 * self.zoom_level;
        let start = self.playhead;
        let end = start + range;
        (start, end)
    }

    pub fn get_keyframe_pixel_position(&self, keyframe_time: f32) -> f32 {
        let start = self.visible_range.0;
        (keyframe_time - start) / self.zoom_level
    }

    pub fn get_time_from_pixel(&self, pixel_x: f32) -> f32 {
        let start = self.visible_range.0;
        start + pixel_x * self.zoom_level
    }

    pub fn add_selected_key(&mut self, key_index: usize) {
        if !self.selected_keys.contains(&key_index) {
            self.selected_keys.push(key_index);
        }
    }

    pub fn remove_selected_key(&mut self, key_index: usize) {
        self.selected_keys.retain(|&i| i != key_index);
    }

    pub fn clear_selection(&mut self) {
        self.selected_keys.clear();
    }

    pub fn is_key_selected(&self, key_index: usize) -> bool {
        self.selected_keys.contains(&key_index)
    }

    pub fn get_selected_keys(&self) -> &[usize] {
        &self.selected_keys
    }

    pub fn start_dragging_key(&mut self, key_index: usize) {
        self.dragging_key = Some(key_index);
    }

    pub fn end_dragging_key(&mut self) {
        self.dragging_key = None;
    }

    pub fn update_playhead(&mut self, time: f32) {
        self.playhead = time;
    }

    pub fn get_playhead_time(&self) -> f32 {
        self.playhead
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::animation::Animation;
    use uuid::Uuid;

    #[test]
    fn test_interpolation_linear() {
        let interp = AdvancedInterpolator::new(InterpolationType::Linear, LoopMode::Loop, 1.0);
        let result = interp.interpolate_value(0.0, 10.0, 0.5);
        assert!((result - 5.0).abs() < 0.001);
    }

    #[test]
    fn test_interpolation_ease_in() {
        let interp = AdvancedInterpolator::new(InterpolationType::EaseIn, LoopMode::Loop, 1.0);
        let result = interp.interpolate_value(0.0, 10.0, 0.5);
        assert!(result < 5.0); // Ease in debe ser menor que lineal en medio
    }

    #[test]
    fn test_interpolation_ease_out() {
        let interp = AdvancedInterpolator::new(InterpolationType::EaseOut, LoopMode::Loop, 1.0);
        let result = interp.interpolate_value(0.0, 10.0, 0.5);
        assert!(result > 5.0); // Ease out debe ser mayor que lineal en medio
    }

    #[test]
    fn test_interpolation_ease_in_out() {
        let interp = AdvancedInterpolator::new(InterpolationType::EaseInOut, LoopMode::Loop, 1.0);
        let result = interp.interpolate_value(0.0, 10.0, 0.5);
        assert!((result - 5.0).abs() < 0.001); // Ease in out debe ser lineal en medio
    }

    #[test]
    fn test_interpolation_step() {
        let interp = AdvancedInterpolator::new(InterpolationType::Step, LoopMode::Loop, 1.0);
        let result1 = interp.interpolate_value(0.0, 10.0, 0.4);
        let result2 = interp.interpolate_value(0.0, 10.0, 0.6);
        assert_eq!(result1, 0.0); // Step debe mantener valor anterior
        assert_eq!(result2, 10.0); // Step debe cambiar después de 0.5
    }

    #[test]
    fn test_interpolation_position() {
        let interp = AdvancedInterpolator::new(InterpolationType::Linear, LoopMode::Loop, 1.0);
        let start = [0.0, 0.0, 0.0];
        let end = [10.0, 20.0, 30.0];
        let result = interp.interpolate_position(start, end, 0.5);
        assert!((result[0] - 5.0).abs() < 0.001);
        assert!((result[1] - 10.0).abs() < 0.001);
        assert!((result[2] - 15.0).abs() < 0.001);
    }

    #[test]
    fn test_interpolation_transform() {
        let interp = AdvancedInterpolator::new(InterpolationType::Linear, LoopMode::Loop, 1.0);
        let start = Transform {
            position: [0.0, 0.0, 0.0],
            rotation: [0.0, 0.0, 0.0],
            scale: [1.0, 1.0, 1.0],
        };
        let end = Transform {
            position: [10.0, 20.0, 30.0],
            rotation: [90.0, 180.0, 270.0],
            scale: [2.0, 3.0, 4.0],
        };
        let result = interp.interpolate_transform(&start, &end, 0.5);
        assert!((result.position[0] - 5.0).abs() < 0.001);
        assert!((result.rotation[0] - 45.0).abs() < 0.001);
        assert!((result.scale[0] - 1.5).abs() < 0.001);
    }

    #[test]
    fn test_advanced_interpolator_loop_modes() {
        let interp = AdvancedInterpolator::new(InterpolationType::Linear, LoopMode::PingPong, 1.0);
        
        // Tiempo 0.5 debería dar 0.5
        let result1 = interp.interpolate_value(0.0, 10.0, 0.5);
        assert!((result1 - 5.0).abs() < 0.001);
        
        // Tiempo 0.8 en ping pong debería dar lo mismo que 0.2
        let result2 = interp.interpolate_value(0.0, 10.0, 0.8);
        let result3 = interp.interpolate_value(0.0, 10.0, 0.2);
        assert!((result2 - result3).abs() < 0.001);
    }

    #[test]
    fn test_timeline_manager_zoom() {
        let mut timeline = TimelineManager::new();
        
        timeline.set_zoom(2.0);
        let range = timeline.get_visible_time_range(100);
        assert!((range.1 - range.0).abs() > 199.0); // 100 píxeles * 2 zoom
        
        timeline.set_zoom(0.5);
        let range = timeline.get_visible_time_range(100);
        assert!((range.1 - range.0).abs() < 51.0); // 100 píxeles * 0.5 zoom
    }

    #[test]
    fn test_timeline_manager_keyframe_positioning() {
        let mut timeline = TimelineManager::new();
        timeline.set_zoom(1.0);
        
        // Keyframe en tiempo 5.0 debería estar en pixel 5.0
        let pixel = timeline.get_keyframe_pixel_position(5.0);
        assert!((pixel - 5.0).abs() < 0.001);
        
        // Pixel 10.0 debería dar tiempo 10.0
        let time = timeline.get_time_from_pixel(10.0);
        assert!((time - 10.0).abs() < 0.001);
    }

    #[test]
    fn test_timeline_manager_selection() {
        let mut timeline = TimelineManager::new();
        
        timeline.add_selected_key(0);
        timeline.add_selected_key(1);
        timeline.add_selected_key(2);
        
        assert_eq!(timeline.get_selected_keys().len(), 3);
        assert!(timeline.is_key_selected(0));
        assert!(timeline.is_key_selected(1));
        assert!(timeline.is_key_selected(2));
        assert!(!timeline.is_key_selected(3));
    }

    #[test]
    fn test_animation_keyframe_operations() {
        let anim_id = Uuid::new_v4();
        let mut anim = Animation::new(anim_id, "test".to_string(), 1.0, 30.0);
        
        let transform = Transform::default();
        let keyframe = Keyframe::new(0.0, Uuid::new_v4(), transform, 0.0);
        anim.add_keyframe(keyframe);
        
        assert_eq!(anim.keyframes.len(), 1);
    }

    #[test]
    fn test_animation_keyframe_interpolation() {
        let anim_id = Uuid::new_v4();
        let mut anim = Animation::new(anim_id, "test".to_string(), 1.0, 30.0);
        
        let target = Uuid::new_v4();
        
        let transform1 = Transform {
            position: [0.0, 0.0, 0.0],
            rotation: [0.0, 0.0, 0.0],
            scale: [1.0, 1.0, 1.0],
        };
        
        let transform2 = Transform {
            position: [10.0, 20.0, 30.0],
            rotation: [90.0, 180.0, 270.0],
            scale: [2.0, 3.0, 4.0],
        };
        
        let keyframe1 = Keyframe::new_with_interpolation(0.0, target, transform1, 0.0, InterpolationType::Linear);
        let keyframe2 = Keyframe::new_with_interpolation(1.0, target, transform2, 0.0, InterpolationType::Linear);
        
        anim.add_keyframe(keyframe1);
        anim.add_keyframe(keyframe2);
        
        let result = anim.interpolate_transform(0.5, target);
        assert!((result.position[0] - 5.0).abs() < 0.001);
        assert!((result.position[1] - 10.0).abs() < 0.001);
    }

    #[test]
    fn test_animation_keyframe_generation() {
        let anim_id = Uuid::new_v4();
        let mut anim = Animation::new(anim_id, "test".to_string(), 1.0, 30.0);
        
        let targets = vec![Uuid::new_v4(), Uuid::new_v4()];
        anim.generate_keyframes(0.0, 1.0, 30.0, targets);
        
        // Debería haber 30 keyframes por target (30 fps * 1 segundo)
        assert_eq!(anim.keyframes.len(), 60); // 2 targets * 30 keyframes
    }
}
