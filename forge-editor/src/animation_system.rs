use crate::sprite_manager::{SpriteManager};

pub struct Animation {
    name: String,
    frames: Vec<Sprite>,
    frame_rate: u32,
    loop: bool,
}

impl Animation {
    pub fn new(name: &str, frames: Vec<Sprite>, frame_rate: u32, loop_anim: bool) -> Self {
        Animation {
            name: name.to_string(),
            frames,
            frame_rate,
            loop: loop_anim,
        }
    }

    pub fn get_current_frame(&self, time: f32) -> Option<&Sprite> {
        let frame_index = ((time as u32 * self.frame_rate) % self.frames.len() as u32) as usize;
        self.frames.get(frame_index)
    }
}

pub struct AnimationSystem {
    animations: HashMap<String, Animation>,
    sprite_manager: SpriteManager,
}

impl AnimationSystem {
    pub fn new(sprite_manager: SpriteManager) -> Self {
        AnimationSystem {
            animations: HashMap::new(),
            sprite_manager,
        }
    }

    pub fn load_animation(&mut self, anim_name: &str, sprite_name: &str, 
                         frame_rate: u32, loop_anim: bool) -> Result<(), String> {
        let sprite = self.sprite_manager.get_sprite("characters", sprite_name)
            .ok_or_else(|| format!("Sprite '{}' not found", sprite_name))?;
        
        let frames = vec![sprite];
        let animation = Animation::new(anim_name, frames, frame_rate, loop_anim);
        self.animations.insert(anim_name.to_string(), animation);
        Ok(())
    }

    pub fn get_animation(&self, name: &str) -> Option<&Animation> {
        self.animations.get(name)
    }

    pub fn update(&mut self, time: f32) {
        for anim in self.animations.values_mut() {
            if let Some(frame) = anim.get_current_frame(time) {
                // Actualizar sprite
            }
        }
    }
}
