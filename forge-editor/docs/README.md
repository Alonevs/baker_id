# Forge Editor - Documentación

## Overview
Forge Editor es un editor de desarrollo de juegos 2D/3D con soporte para física, partículas, animaciones, exportación de mapas y línea de tiempo.

---

## Módulos

### 1. `physics_2d`
Motor de física 2D con detección de colisiones, gravedad y fricción.

**Estructuras principales:**
- `Physics2D`: Configuración del motor de física (gravedad, fricción, límites)
- `Body`: Entidad física con posición, rotación, escala y velocidad
- `Position`: Coordenadas 3D
- `Rotation`: Rotación en radianes
- `Velocity`: Vector de velocidad
- `Collision`: Información de colisiones

**Funciones principales:**
- `create_body()`: Crea un nuevo cuerpo físico
- `update()`: Actualiza la física en un delta de tiempo
- `check_collision()`: Detecta colisiones entre cuerpos
- `apply_force()`: Aplica fuerza a un cuerpo

### 2. `particle_system`
Sistema de partículas animadas con control de FPS y sprites.

**Estructuras principales:**
- `ParticleSystem`: Sistema de partículas con lista de partículas y frames de sprite
- `Particle`: Partícula individual con posición, rotación, escala, velocidad y vida
- `SpriteFrame`: Frame de sprite con textura y duración

**Funciones principales:**
- `add_particle()`: Añade una nueva partícula
- `update()`: Actualiza todas las partículas
- `remove_particle()`: Elimina una partícula
- `set_fps()`: Configura el FPS del sistema
- `add_sprite_frame()`: Añade un frame de sprite

### 3. `animation_2d`
Sistema de animación 2D con keyframes e interpolación.

**Tipos de interpolación:**
- `Linear`: Interpolación lineal
- `EaseIn`: Interpolación con easing hacia adelante
- `EaseOut`: Interpolación con easing hacia atrás
- `EaseInOut`: Interpolación con easing en ambos extremos

**Estructuras principales:**
- `Animation2D`: Animación con lista de keyframes
- `Keyframe`: Keyframe con frame, valor y tipo de interpolación

**Funciones principales:**
- `add_keyframe()`: Añade un keyframe
- `interpolate()`: Interpola entre keyframes
- `play()`: Reproduce la animación
- `get_current_value()`: Obtiene el valor actual

### 4. `map_export`
Exportador de archivos `.map` con soporte para física, partículas, animaciones, diálogos y eventos.

**Funciones principales:**
- `export_map()`: Exporta un archivo `.map` completo
- `import_map()`: Importa un archivo `.map`
- `export_dialogue()`: Exporta datos de diálogo
- `export_event()`: Exporta datos de evento

### 5. `viewport_2d`
Componente de viewport 2D con canvas y paneles de UI.

**Funciones principales:**
- `create_canvas()`: Crea el canvas del viewport
- `render()`: Renderiza el viewport
- `add_panel()`: Añade un panel de UI

### 6. `viewport_integration`
Integración de viewport con render loop.

**Funciones principales:**
- `init()`: Inicializa el viewport
- `render_loop()`: Render loop principal
- `update()`: Actualiza el viewport

### 7. `property_panel`
Panel de propiedades para editar transformaciones, componentes y scripts.

**Estructuras principales:**
- `PropertyPanel`: Panel de propiedades principal
- `PropertyTab`: Tipo de pestaña (Transform, Component, Script)
- `TransformProperties`: Propiedades de transformación
- `ComponentProperties`: Propiedades de componentes
- `ScriptProperties`: Propiedades de scripts

**Funciones principales:**
- `set_selected_entity()`: Establece la entidad seleccionada
- `get_transform_properties()`: Obtiene propiedades de transformación
- `set_position()`: Establece posición
- `set_rotation()`: Establece rotación
- `set_scale()`: Establece escala
- `set_visible()`: Establece visibilidad
- `add_component()`: Añade un componente
- `remove_component()`: Elimina un componente

### 8. `transform_editor`
Editor de transformaciones para entidades.

**Funciones principales:**
- `create_widgets()`: Crea los widgets del editor
- `get_transform_properties()`: Obtiene propiedades de transformación
- `get_component_properties()`: Obtiene propiedades de componentes
- `get_script_properties()`: Obtiene propiedades de scripts

### 9. `component_editor`
Editor de componentes para entidades.

**Funciones principales:**
- `create_widgets()`: Crea los widgets del editor
- `get_component_properties()`: Obtiene propiedades de componentes
- `get_script_properties()`: Obtiene propiedades de scripts

### 10. `timeline`
Línea de tiempo para animaciones.

**Estructuras principales:**
- `TimelineEditor`: Editor de línea de tiempo
- `Track`: Pista de animación con keyframes
- `Keyframe`: Keyframe con frame y valor
- `InterpolationType`: Tipo de interpolación

**Funciones principales:**
- `create_widgets()`: Crea los widgets del editor
- `add_keyframe()`: Añade un keyframe
- `remove_keyframe()`: Elimina un keyframe
- `get_current_frame()`: Obtiene el frame actual
- `set_current_frame()`: Establece el frame actual

### 11. `keyframe`
Editor de keyframes para animaciones.

**Funciones principales:**
- `create_widgets()`: Crea los widgets del editor
- `get_transform_properties()`: Obtiene propiedades de transformación
- `get_component_properties()`: Obtiene propiedades de componentes

### 12. `animation_track`
Pista de animación para línea de tiempo.

**Estructuras principales:**
- `AnimationTrack`: Pista de animación
- `TrackData`: Datos de pista con frame, valor y propiedades

**Funciones principales:**
- `create_widgets()`: Crea los widgets del editor
- `add_track_data()`: Añade datos a la pista
- `remove_track_data()`: Elimina datos de la pista
- `clear_track_data()`: Limpia todos los datos

---

## Dependencias
- `serde`: Serialización/deserialización
- `serde_json`: JSON
- `rfd`: Diálogos de archivo
- `once_cell`: Lazy static

---

## Ejemplo de uso

```rust
use forge_editor::physics_2d::Physics2D;
use forge_editor::particle_system::ParticleSystem;
use forge_editor::animation_2d::Animation2D;
use forge_editor::property_panel::PropertyPanel;
use forge_editor::transform_editor::TransformEditor;
use forge_editor::timeline::TimelineEditor;

fn main() {
    // Crear motor de física
    let physics = Physics2D::new();
    
    // Crear sistema de partículas
    let mut particles = ParticleSystem::new();
    
    // Crear animación
    let mut animation = Animation2D::new();
    
    // Crear panel de propiedades
    let property_panel = PropertyPanel::new();
    
    // Crear editor de transformaciones
    let transform_editor = TransformEditor::new(property_panel);
    
    // Crear editor de línea de tiempo
    let timeline_editor = TimelineEditor::new(property_panel);
    
    println!("Forge Editor iniciado");
}
```

## Integración UI

La aplicación utiliza `eframe` y `egui` para la interfaz gráfica:

```rust
use eframe::egui;
use forge_editor::ForgeEditorApp;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 720.0])
            .with_min_inner_size([800.0, 600.0]),
        ..Default::default()
    };
    
    eframe::run_native(
        "Forge Editor",
        options,
        Box::new(|cc| Ok(Box::new(ForgeEditorApp::new(cc)))),
    )
}
```

### Layout de la UI

- **Panel Superior (Top)**: Menú de aplicación (File, Edit, Physics, Particles, Animation, Help)
- **Panel Izquierdo (Left)**: Propiedades de entidad y lista de entidades
- **Panel Central (Center)**: Viewport principal con visualización de física, partículas y animaciones
- **Panel Derecho (Right)**: Editor de línea de tiempo con pistas y keyframes
- **Panel Inferior (Bottom)**: Consola de salida
