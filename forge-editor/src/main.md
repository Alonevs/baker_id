# forge-editor/src/main.rs

## 📋 DESCRIPCIÓN

Archivo principal que inicia la aplicación gráfica del **Forge Editor** usando `eframe` y `egui`.

## 🎯 OBJETIVO

Crear la interfaz visual completa del editor con:
- Panel superior: Menú de opciones
- Panel izquierdo: Propiedades de entidad y lista de entidades
- Panel central: Viewport principal con visualización
- Panel derecho: Editor de línea de tiempo
- Panel inferior: Consola de estado

## 📦 ESTRUCTURA DE DATOS

```rust
struct ForgeEditorApp {
    // Estado del motor de física
    physics: Physics2D,
    
    // Sistema de partículas
    particles: ParticleSystem,
    
    // Sistema de animación 2D
    animation: Animation2D,
    
    // Paneles de UI
    property_panel: PropertyPanel,
    transform_editor: TransformEditor,
    component_editor: ComponentEditor,
    timeline_editor: TimelineEditor,
    keyframe_editor: KeyframeEditor,
    animation_track: AnimationTrack,
    
    // Viewport principal
    viewport: egui::CentralPanel,
}
```

## 🔧 FUNCIONES PRINCIPALES

### 1. `new(_cc: &eframe::CreationContext<'_>) -> Self`

**Propósito:** Inicializar todos los componentes del editor.

**Componentes creados:**
- `Physics2D`: Motor de física 2D
- `ParticleSystem`: Sistema de partículas animadas
- `Animation2D`: Gestor de animaciones con keyframes
- `PropertyPanel`: Panel de propiedades de entidades
- `TransformEditor`: Editor de transformaciones (posición, rotación, escala)
- `ComponentEditor`: Editor de componentes
- `TimelineEditor`: Editor de línea de tiempo
- `KeyframeEditor`: Editor de keyframes
- `AnimationTrack`: Pista de animación

### 2. `update_physics(&mut self, dt: f32)`

**Propósito:** Actualizar el motor de física con delta time.

**Lógica:**
- Iterar sobre todos los cuerpos físicos
- Aplicar gravedad, colisiones y movimiento
- Actualizar posiciones y velocidades

### 3. `update_particles(&mut self, dt: f32)`

**Propósito:** Actualizar el sistema de partículas.

**Lógica:**
- Iterar sobre todas las partículas
- Actualizar posiciones, vidas y colores
- Eliminar partículas expiradas

### 4. `update_animation(&mut self, dt: f32)`

**Propósito:** Actualizar el sistema de animaciones.

**Lógica:**
- Reproducir animación si está activa
- Interpolar valores entre keyframes
- Actualizar tiempo de animación

## 🎨 UI LAYOUT

### Panel Superior (Top Panel)
**Función:** Menú de opciones del editor

**Submenús:**
- **File**: New, Open, Save, Exit
- **Edit**: Undo, Redo, Cut, Copy, Paste
- **Physics**: Add Body, Update Physics, Reset Physics
- **Particles**: Add Particle, Update Particles, Clear Particles
- **Animation**: Add Keyframe, Play Animation, Stop Animation
- **Help**: About, Documentation

### Panel Izquierdo (Left Panel)
**Función:** Propiedades de entidad y lista de entidades

**Secciones:**
1. **Transform Properties**
   - Position (X, Y, Z)
   - Rotation (X, Y, Z)
   - Scale (X, Y, Z)
   - Visible (bool)

2. **Component Properties**
   - Lista de componentes asignados

3. **Script Properties**
   - Lista de scripts asignados

4. **Entities**
   - Lista de entidades físicas
   - Selección al hacer click

### Panel Central (Central Panel)
**Función:** Viewport principal con visualización

**Contenido:**
1. **Physics Bodies**
   - Mostrar posición de cada cuerpo físico

2. **Particles**
   - Mostrar posición de cada partícula

3. **Animation**
   - Mostrar valor actual de animación

4. **Timeline Controls**
   - Botones: Play, Stop, Reset
   - Frame counter
   - Next Frame / Prev Frame

### Panel Derecho (Right Panel)
**Función:** Editor de línea de tiempo

**Secciones:**
1. **Timeline Info**
   - Current Frame
   - Frame Duration (ms)

2. **Tracks**
   - Lista de pistas de animación
   - Mostrar interpolación y keyframes

3. **Timeline Editor**
   - Add Keyframe form:
     - Frame number
     - Value
     - Interpolation type (Linear, EaseIn, EaseOut, EaseInOut)
   - Button: Add Keyframe

### Panel Inferior (Bottom Panel)
**Función:** Consola de estado

**Contenido:**
- Forge Editor v1.0
- Ready status
- Physics Engine: Active
- Particle System: Active
- Animation System: Active
- Timeline: Active

## 🔄 FLUJO DE EJECUCIÓN

```
1. main() inicia eframe::run_native()
   ↓
2. ForgeEditorApp::new(cc) inicializa todos los componentes
   ↓
3. Loop principal de eframe:
   - update(ctx, frame) se llama cada frame
   - Actualiza física, partículas, animación
   - Renderiza UI con egui
   ↓
4. App cierra cuando el usuario sale
```

## ⚠️ NOTAS IMPORTANTES

### APIs de egui
- `egui::TopBottomPanel::top()` - Panel superior
- `egui::SidePanel::left()` - Panel izquierdo
- `egui::SidePanel::right()` - Panel derecho
- `egui::BottomPanel::bottom()` - Panel inferior
- `egui::CentralPanel` - Panel central
- `ui.button()` - Botones interactivos
- `ui.label()` - Etiquetas de texto
- `ui.group()` - Grupos visuales
- `ui.separator()` - Líneas separadoras
- `ui.heading()` - Títulos
- `ui.selectable_label()` - Labels seleccionables

### Delta Time (dt)
- Representa el tiempo transcurrido entre frames (segundos)
- Usado para actualizaciones independientes del framerate
- Fórmula: `dt = frame_time - last_frame_time`

### State Management
- Todos los componentes se guardan en `ForgeEditorApp`
- Se actualizan en cada frame mediante `update()`
- UI se renderiza usando `ctx` (egui::Context)

## 📝 ARCHIVOS RELACIONADOS

- `lib.rs` - Módulo principal del editor
- `physics_2d.rs` - Motor de física
- `particle_system.rs` - Sistema de partículas
- `animation_2d.rs` - Sistema de animaciones
- `property_panel.rs` - Panel de propiedades
- `transform_editor.rs` - Editor de transformaciones
- `component_editor.rs` - Editor de componentes
- `timeline.rs` - Editor de línea de tiempo
- `keyframe.rs` - Editor de keyframes
- `animation_track.rs` - Pistas de animación

## ✅ ESTADO DE COMPILACIÓN

**✅ Compilación exitosa** - El proyecto compila sin errores de tipo

**⚠️ Warnings:**
- Variables `dt` no usadas en `update_physics()` y `update_particles()`
- Campos de `ForgeEditorApp` no leídos (placeholder UI)
- `text_edit_singleline` requiere referencia mutada a String

**📝 Notas:**
- `update_physics()` y `update_particles()` ahora usan métodos simplificados
- UI es placeholder - necesita implementación completa
- `physics.gravity.x` no se usa (solo `gravity.y` está definido)

## 🚀 COMANDOS PARA EJECUTAR

```bash
# Compilar
cargo build

# Ejecutar
cargo run

# Con dev features
cargo run --features native

# Con release optimizations
cargo run --release
```
