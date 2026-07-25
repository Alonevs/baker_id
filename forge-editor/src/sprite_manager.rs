use std::collections::HashMap;

pub struct SpriteManager {
    atlases: HashMap<String, TextureAtlas>,
    current_atlas: Option<String>,
}

impl SpriteManager {
    pub fn new() -> Self {
        SpriteManager {
            atlases: HashMap::new(),
            current_atlas: None,
        }
    }

    pub fn load_atlas(&mut self, name: &str, config: &AtlasConfig) -> Result<(), String> {
        let texture = self.load_texture(&config.path)?;
        let atlas = TextureAtlas::new(texture, config);
        self.atlases.insert(name.to_string(), atlas);
        Ok(())
    }

    pub fn get_atlas(&self, name: &str) -> Option<&TextureAtlas> {
        self.atlases.get(name)
    }

    pub fn get_sprite(&self, atlas_name: &str, sprite_name: &str) -> Option<Sprite> {
        self.atlases.get(atlas_name)
            .and_then(|atlas| atlas.get_sprite(sprite_name))
    }

    pub fn load_all_atlases(&mut self, config_path: &str) -> Result<(), String> {
        let config = serde_json::from_str::<AtlasConfig>(std::fs::read_to_string(config_path).unwrap())
            .unwrap();
        
        for (name, atlas_config) in config.atlases {
            self.load_atlas(&name, &atlas_config)?;
        }
        
        Ok(())
    }
}

struct TextureAtlas {
    texture: Texture,
    sprites: HashMap<String, Sprite>,
}

impl TextureAtlas {
    fn new(texture: Texture, config: &AtlasConfig) -> Self {
        let mut sprites = HashMap::new();
        for sprite_config in &config.sprites {
            let region = Rect::new(sprite_config.region.x, sprite_config.region.y, 
                                   sprite_config.region.w, sprite_config.region.h);
            let sprite = Sprite::new(sprite_config.name.clone(), texture, region);
            sprites.insert(sprite_config.name.clone(), sprite);
        }
        TextureAtlas { texture, sprites }
    }

    fn get_sprite(&self, name: &str) -> Option<Sprite> {
        self.sprites.get(name).cloned()
    }
}

pub struct Sprite {
    name: String,
    texture: Texture,
    region: Rect,
}

impl Sprite {
    fn new(name: String, texture: Texture, region: Rect) -> Self {
        Sprite { name, texture, region }
    }

    pub fn draw(&self, renderer: &mut Renderer, x: f32, y: f32, angle: f32) {
        renderer.draw_texture(&self.texture, x, y, self.region, angle);
    }
}
