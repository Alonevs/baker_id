# Resource Manager API Documentation

## Overview

El `ResourceManager` es un sistema completo para cargar, gestionar y cachear recursos del juego (sprites, audio, scripts, niveles). Usa un algoritmo LRU (Least Recently Used) para optimizar el uso de memoria.

## Quick Start

```rust
use forge_runtime::resource::{ResourceManager, ResourceType};

// Crear manager con configuración por defecto
let mut manager = ResourceManager::default();

// Cargar recurso desde JSON string
let json = r#"{
    "name": "my_sprite",
    "width": 32,
    "height": 32,
    "texture_path": "sprites/my_sprite.png",
    "frames": []
}"#;

manager.load_from_json("my_sprite", json, ResourceType::Sprite)?;

// Obtener recurso
if let Some(sprite) = manager.get("my_sprite") {
    println!("Loaded sprite: {}", sprite.name());
}

// Limpiar cache
manager.cleanup();
```

## Tipos de Recursos

### 1. Sprite Resource (`ResourceType::Sprite`)

```json
{
    "name": "hero_sprite",
    "width": 32,
    "height": 32,
    "texture_path": "sprites/hero.png",
    "frames": [
        {
            "name": "frame_0",
            "x": 0,
            "y": 0,
            "width": 32,
            "height": 32,
            "duration": 0.1
        }
    ],
    "atlas": null,
    "animations": {
        "walk": ["frame_0", "frame_1", "frame_2"],
        "idle": ["frame_0"]
    }
}
```

**Campos:**
- `name`: Nombre único del recurso
- `width`, `height`: Dimensiones del sprite
- `texture_path`: Ruta al archivo de textura
- `frames`: Array de frames para animación
- `atlas`: Path al atlas de texturas (opcional)
- `animations`: Definición de animaciones

### 2. Audio Resource (`ResourceType::Audio`)

```json
{
    "name": "background_music",
    "duration": 300.0,
    "path": "audio/background.mp3",
    "channels": 2,
    "sample_rate": 44100,
    "format": "stereo",
    "loop": true,
    "volume": 1.0,
    "fade_in": 2.0,
    "fade_out": 2.0
}
```

**Campos:**
- `name`: Nombre único del recurso
- `duration`: Duración en segundos
- `path`: Ruta al archivo de audio
- `channels`: Número de canales (1=mono, 2=stereo)
- `sample_rate`: Muestreo (44100, 48000, etc.)
- `format`: "mono", "stereo", "surround"
- `loop`: Repetir en bucle
- `volume`: Volumen (0.0 - 1.0)
- `fade_in`, `fade_out`: Efectos de fade (segundos)

### 3. Script Resource (`ResourceType::Script`)

```json
{
    "name": "player_controller",
    "language": "lua",
    "code": "print('hello')",
    "is_compiled": false,
    "dependencies": ["input_system", "vec2_math"],
    "metadata": {
        "version": "1.0.0",
        "author": "developer"
    }
}
```

**Campos:**
- `name`: Nombre único del script
- `language`: "lua", "rust", "python", o custom
- `code`: Código del script
- `is_compiled`: Si está compilado
- `dependencies`: Scripts dependientes
- `metadata`: Metadatos (opcional)

### 4. Level Resource (`ResourceType::Level`)

```json
{
    "name": "level_1",
    "width": 20,
    "height": 15,
    "tiles": [
        {"x": 0, "y": 0, "type": "grass"},
        {"x": 1, "y": 0, "type": "wall"}
    ],
    "entities": [
        {"id": "player", "x": 1.0, "y": 1.0, "sprite": "hero_sprite", "health": 100}
    ],
    "transitions": {
        "type": "fade",
        "duration": 1.0,
        "color": "#000000"
    },
    "camera": {
        "type": "isometric",
        "position": {"x": 10.0, "y": 7.5},
        "zoom": 1.0
    }
}
```

**Campos:**
- `name`: Nombre único del nivel
- `width`, `height`: Dimensiones del nivel
- `tiles`: Array de tiles
- `entities`: Array de entidades
- `transitions`: Transición entre niveles
- `camera`: Configuración de cámara

## API Reference

### ResourceManager

```rust
pub struct ResourceManager {
    cache: ResourceCache,
    config: CacheConfig,
}

// Crear nuevo ResourceManager
pub fn new(config: CacheConfig) -> Self

// Crear con configuración por defecto
pub fn default() -> Self

// Cargar recurso desde archivo
pub fn load(&mut self, name: &str, file_path: &Path, resource_type: ResourceType) -> Result<(), String>

// Cargar recurso desde JSON string
pub fn load_from_json(&mut self, name: &str, json: &str, resource_type: ResourceType) -> Result<(), String>

// Obtener recurso por nombre
pub fn get(&self, name: &str) -> Option<&dyn Resource>

// Obtener referencia mutable
pub fn get_mut(&mut self, name: &str) -> Option<&mut dyn Resource>

// Obtener referencia mutable al Box
pub fn get_mut_box(&mut self, name: &str) -> Option<&mut Box<dyn Resource>>

// Marcar recurso como usado (move al final de la lista LRU)
pub fn mark_used(&mut self, name: &str)

// Limpiar cache si supera el límite
pub fn cleanup(&mut self)

// Limpiar recursos de un tipo específico
pub fn cleanup_type(&mut self, resource_type: ResourceType)

// Limpiar todo
pub fn clear(&mut self)

// Número de recursos
pub fn len(&self) -> usize

// Verifica si está vacío
pub fn is_empty(&self) -> bool
```

### ResourceCache

```rust
pub struct ResourceCache {
    cache: HashMap<String, Box<dyn Resource>>,
    access_order: Vec<String>,
    config: CacheConfig,
}

// Inserta un recurso directamente
pub fn insert(&mut self, name: String, resource: Box<dyn Resource>)

// Carga un recurso desde archivo
pub fn load(&mut self, name: &str, file_path: &Path, resource_type: ResourceType) -> Result<(), String>

// Limpia recursos de un tipo específico
pub fn cleanup_type(&mut self, resource_type: ResourceType)

// Retorna número de recursos
pub fn len(&self) -> usize

// Verifica si está vacío
pub fn is_empty(&self) -> bool

// Limpia todo
pub fn clear(&self)
```

### CacheConfig

```rust
pub struct CacheConfig {
    pub max_size: usize,           // Tamaño máximo total
    pub max_per_type: HashMap<ResourceType, usize>,  // Máximo por tipo
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            max_size: 1000,
            max_per_type: HashMap::from([
                (ResourceType::Sprite, 500),
                (ResourceType::Audio, 100),
                (ResourceType::Script, 50),
                (ResourceType::Level, 100),
            ]),
        }
    }
}

pub fn new(max_size: usize, max_per_type: HashMap<ResourceType, usize>) -> Self
```

### ResourceType

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ResourceType {
    Sprite,
    Audio,
    Script,
    Level,
    Custom(String),
}

impl ResourceType {
    pub fn to_string(&self) -> &str
}
```

## Características

### 1. Cache LRU (Least Recently Used)

El `ResourceManager` usa un algoritmo LRU para gestionar la caché:
- Los recursos más recientemente usados se mantienen
- Cuando se supera el límite, se eliminan los menos usados
- `mark_used()` mueve un recurso al final de la lista de acceso

### 2. Lazy Loading

Los recursos se cargan solo cuando son necesarios:
```rust
if let Some(sprite) = manager.get("my_sprite") {
    // Solo se carga si existe en la caché
    sprite.load()?;
}
```

### 3. Tipado Seguro

Cada recurso tiene su propio tipo:
```rust
if let Some(sprite) = manager.get("my_sprite") {
    if let Some(s) = sprite.downcast_ref::<SpriteResource>() {
        // Usar SpriteResource específico
    }
}
```

### 4. Limpieza Automática

El sistema limpia automáticamente:
- Recursos que no han sido usados
- Recursos de tipos específicos (ej. limpiar todos los sprites)
- Cache completo con `clear()`

## Ejemplos Avanzados

### Cargar nivel completo

```rust
let mut manager = ResourceManager::default();

// Cargar assets del nivel
manager.load_from_json("hero", sprite_json, ResourceType::Sprite)?;
manager.load_from_json("enemy", enemy_json, ResourceType::Sprite)?;
manager.load_from_json("bgm", audio_json, ResourceType::Audio)?;

// Cargar nivel
manager.load_from_json("level_1", level_json, ResourceType::Level)?;

// Obtener nivel
if let Some(level) = manager.get("level_1") {
    if let Some(l) = level.downcast_ref::<LevelResource>() {
        println!("Level: {}x{}", l.width, l.height);
    }
}
```

### Gestionar memoria

```rust
// Marcar recursos como usados
manager.mark_used("hero");
manager.mark_used("bgm");

// Limpiar tipos específicos cuando no se necesitan
if level_finished {
    manager.cleanup_type(ResourceType::Sprite);
    manager.cleanup_type(ResourceType::Audio);
}

// Limpiar todo antes de cargar nuevo nivel
manager.clear();
```

### Custom Resources

```rust
// Crear recurso personalizado
let custom = CustomResource::new("my_custom", "my_type".to_string(), &data);

// Cargar desde JSON
manager.load_from_json("custom", json, ResourceType::Custom("my_type".to_string()))?;

// Obtener
if let Some(resource) = manager.get("custom") {
    if let Some(c) = resource.downcast_ref::<CustomResource>() {
        println!("Custom resource: {}", c.name());
    }
}
```

## Testing

Ejecutar tests:
```bash
cargo test -p forge-runtime
```

Tests disponibles:
- `test_resource_manager_new`
- `test_resource_manager_load_sprite`
- `test_resource_manager_load_audio`
- `test_resource_manager_load_script`
- `test_resource_manager_load_level`
- `test_resource_manager_cache_lru`
- `test_resource_manager_cleanup_type`
- `test_resource_manager_clear`
- `test_custom_resource`
- `test_resource_manager_default`

## Sample Files

Los siguientes ejemplos están en `src/resource/samples/`:
- `hero_sprite.json` - Sprite con animación
- `background_music.json` - Audio completo
- `player_controller.json` - Script Lua
- `level_1.json` - Nivel completo

## Notas de Implementación

1. **Lifetimes**: Los métodos `get()` y `get_mut()` retornan referencias con lifetime implícito
2. **Type Erasure**: `Box<dyn Resource>` permite almacenar diferentes tipos en el mismo contenedor
3. **Memory Safety**: El sistema usa RAII para gestionar la memoria de forma segura
4. **Serialization**: Todos los recursos implementan `Serialize` y `Deserialize` para fácil persistencia
