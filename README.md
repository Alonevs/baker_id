# Forge SDK 2D

SDK para desarrollo de juegos 2D con editor visual y motor de juego.

## Estructura del Proyecto

```
forge-workspace/
├── Cargo.toml                    # Workspace definition
├── crates/
│   ├── forge-types/              # Datos compartidos (Project, Scene, Events)
│   ├── forge-panel-messaging/    # Sistema de eventos entre paneles
│   ├── forge-undo-redo/          # Sistema Undo/Redo
│   ├── forge-map-cart/           # Formato de archivos .map
│   └── forge-editor/             # IDE visual (eframe + egui)
```

## Volúmenes 0 - Implementados

### 0-A: Panel Messaging
Sistema centralizado de eventos entre paneles del editor.

**Archivo:** `crates/forge-panel-messaging/src/lib.rs`

**Características:**
- `EventBus`: Pub/Sub para eventos del editor
- `PanelMessenger`: Comunicación entre paneles
- Tipos de eventos: PanelOpen, PanelClose, ViewportResize, AssetLoaded, SaveRequested, SceneLoaded, EntitySelected, PropertyChanged

### 0-B: Undo/Redo
Sistema completo de deshacer/rehacer.

**Archivo:** `crates/forge-undo-redo/src/lib.rs`

**Características:**
- `UndoStack`: Pila de operaciones con límite configurable
- `UndoManager`: Gestor con serialización JSON
- Máximo 100 operaciones por defecto

### 0-C: Map/Cart Format
Formato para archivos de nivel `.map`.

**Archivo:** `crates/forge-map-cart/src/lib.rs`

**Características:**
- Estructuras: `MapFile`, `Layer`, `Tile`, `Entity`, `Component`
- Serialización/deserialización con serde

### forge-types
Datos compartidos entre todos los crates.

**Archivo:** `crates/forge-types/src/lib.rs`

**Módulos:**
- `project.rs`: Datos para `proyecto.toml`
- `scene.rs`: Entidades, posiciones, capas
- `events.rs`: Nodos y cables de Event Forge

### forge-editor
IDE visual basado en eframe + egui + egui_dock.

**Archivo:** `crates/forge-editor/src/main.rs`

**Características:**
- Ventanas dockeables (egui_dock)
- Panel izquierdo: Explorador
- Panel derecho: Inspector
- Viewport central: Mapa

## Compilación

```bash
# Instalar dependencias
cargo build

# Compilar en release
cargo build --release

# Ejecutar editor
cargo run --bin forge-editor
```

## Requisitos

- Rust 1.70+
- Windows, Linux o macOS

## Siguiente: Volúmenes 1-7

Después de completar los Volúmenes 0, implementar:

1. **Volúmen 1**: Workspace con Wizard/Explorer (egui_dock)
2. **Volúmen 2**: Engine con movement/collisions (eframe)
3. **Volúmen 3**: Sequencer con dialogues/nodes
4. **Volúmen 4**: Audio con sockets/behaviors
5. **Volúmen 5**: Compiler con QA/fuzzer
6. **Volúmen 6**: Settings schema completo
7. **Volúmen 7**: Philosophical UX improvements
