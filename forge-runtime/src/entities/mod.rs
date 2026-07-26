//! Entidades del juego 2D

use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Componente de posición 2D
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Position2D {
    pub position: forge_types::Vec2,
}

impl Position2D {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            position: forge_types::Vec2::new(x, y),
        }
    }
}

/// Componente de tamaño 2D
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Size2D {
    pub size: forge_types::Vec2,
}

impl Size2D {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            size: forge_types::Vec2::new(width, height),
        }
    }
}

/// Componente de color
#[derive(Clone, Debug)]
pub struct Color2D {
    pub color: egui::Color32,
}

impl Serialize for Color2D {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("Color2D", 3)?;
        let (r, g, b, _) = self.color.to_tuple();
        state.serialize_field("r", &r)?;
        state.serialize_field("g", &g)?;
        state.serialize_field("b", &b)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for Color2D {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        use serde::de::MapAccess;
        use serde::de::Visitor;
        
        struct ColorVisitor;
        
        impl<'de> Visitor<'de> for ColorVisitor {
            type Value = Color2D;
            
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a color object with r, g, b fields")
            }
            
            fn visit_map<V>(self, mut map: V) -> Result<Color2D, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut r: Option<u8> = None;
                let mut g: Option<u8> = None;
                let mut b: Option<u8> = None;
                
                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "r" => r = Some(map.next_value()?),
                        "g" => g = Some(map.next_value()?),
                        "b" => b = Some(map.next_value()?),
                        _ => { /* ignore unknown keys */ }
                    }
                }
                
                let r = r.unwrap_or(0);
                let g = g.unwrap_or(0);
                let b = b.unwrap_or(0);
                
                Ok(Color2D {
                    color: egui::Color32::from_rgb(r, g, b),
                })
            }
        }
        
        deserializer.deserialize_struct("Color2D", &["r", "g", "b"], ColorVisitor)
    }
}

impl Color2D {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self {
            color: egui::Color32::from_rgb((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8),
        }
    }
}

/// Componente de velocidad
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Velocity2D {
    pub velocity: forge_types::Vec2,
}

impl Velocity2D {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            velocity: forge_types::Vec2::new(x, y),
        }
    }
}

/// Entity structure
#[derive(Debug)]
pub struct Entity {
    pub id: u64,
    pub components: Vec<Box<dyn std::any::Any>>,
}

impl Entity {
    /// Crea una nueva entidad
    pub fn new() -> Self {
        Self {
            id: 0,
            components: Vec::new(),
        }
    }

    /// Añade un componente a la entidad
    pub fn add_component<C: std::any::Any>(&mut self, component: C) {
        self.components.push(Box::new(component));
    }

    /// Establece un componente
    pub fn set_component<C: std::any::Any>(&mut self, component: C) {
        self.components.push(Box::new(component));
    }

    /// Obtiene todos los componentes
    pub fn components(&self) -> &[Box<dyn std::any::Any>] {
        &self.components
    }

    /// Obtiene un componente por tipo
    pub fn get_component<C: std::any::Any>(&self) -> Option<&C> {
        self.components.iter().find_map(|c| {
            c.downcast_ref::<C>()
        })
    }
    
    /// Obtiene un componente mutable por tipo
    pub fn get_component_mut<C: std::any::Any>(&mut self) -> Option<&mut C> {
        self.components.iter_mut().find_map(|c| {
            c.downcast_mut::<C>()
        })
    }
}

impl Default for Entity {
    fn default() -> Self {
        Self::new()
    }
}
