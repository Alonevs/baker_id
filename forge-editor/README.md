# Forge Editor

Un editor de juegos 2D desarrollado en Rust con eframe y egui.

## 📊 Estado Actual

- **Rustc:** 1.97.1
- **UI Framework:** eframe + egui
- **Fecha:** 20/07/2026
- **Estado:** ✅ Fase 4 completada - Asset Browser con árbol jerárquico, filtros e importación

## 🎯 Módulos Principales

### UI Panels
- **Property Panel** - Panel de propiedades de entidades
- **Timeline Editor** - Editor de línea de tiempo y animaciones
- **Transform Editor** - Editor de transformaciones (posición, rotación, escala)
- **Component Editor** - Editor de componentes de entidades
- **Viewport** - Viewport 2D con cámara y zoom
- **Asset Browser** - Explorador de archivos con árbol jerárquico, filtros e importación

### Sistemas Core
- **Physics 2D** - Sistema de física 2D
- **Particle System** - Sistema de partículas
- **Animation 2D** - Sistema de animación
- **Cable System** - Sistema de cables
- **Dialogue Editor** - Editor de diálogos

### Herramientas
- **Export Manager** - Exportar/importar proyectos `.map`
- **Event Nodes** - Nodos de eventos
- **Bitacora** - Sistema de logging

## 📚 Documentación

### APIs Disponibles

#### Property Panel
```rust
use forge_editor::property_panel::{PropertyPanel, EntityId, TransformProperties};

let mut panel = PropertyPanel::new();
panel.set_selected_entity(EntityId(1));
panel.set_transform_props(TransformProperties {
    position: (100.0, 200.0, 0.0),
    rotation: (0.0, 0.0, 0.0),
    scale: (1.0, 1.0, 1.0),
    visible: true,
});
```

#### Timeline Editor
```rust
use forge_editor::timeline::TimelineEditor;

let mut timeline = TimelineEditor::new(panel);
timeline.add_keyframe(0, 0.0);
timeline.add_keyframe(10, 1.0);
let value = timeline.interpolate(5.0);
```

#### Transform Editor
```rust
use forge_editor::transform_editor::TransformEditor;

let mut transform = TransformEditor::new(panel);
let props = transform.get_transform_properties();
```

#### Viewport
```rust
use forge_editor::viewport::Viewport;

let mut viewport = Viewport::new();
viewport.zoom_in();
viewport.set_position(100.0, 200.0);
```

## 🧪 Tests

```bash
cargo test
```

**Resultados:**
- ✅ 24 tests passing
- ✅ 0 tests failing
- ✅ Compilación exitosa

## 📦 Ejemplos

```bash
cargo run --example property_panel
cargo run --example timeline
cargo run --example transform_editor
cargo run --example component_editor
cargo run --example viewport
cargo run --example export_import
```

## 📁 Estructura del Proyecto

```
forge-editor/
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── property_panel.rs
│   ├── timeline.rs
│   ├── transform_editor.rs
│   ├── component_editor.rs
│   ├── ui/
│   │   └── viewport.rs
│   ├── physics_2d.rs
│   ├── particle_system.rs
│   ├── animation_2d.rs
│   ├── cable_system.rs
│   ├── dialogue_editor.rs
│   ├── export_manager.rs
│   ├── event_nodes.rs
│   ├── bitacora_manager.rs
│   └── tests/
│       ├── property_panel_test.rs
│       ├── timeline_test.rs
│       ├── transform_editor_test.rs
│       └── component_editor_test.rs
├── examples/
│   ├── property_panel.rs
│   ├── timeline.rs
│   ├── transform_editor.rs
│   ├── component_editor.rs
│   ├── viewport.rs
│   └── export_import.rs
├── README.md
├── CHANGELOG.md
├── ROADMAP.md
└── Cargo.toml
```

## 🚀 Development

### Compilación
```bash
cargo build
```

### Ejecutar
```bash
cargo run --bin forge-editor
```

### Tests
```bash
cargo test
```

### Lint
```bash
cargo clippy
```

## 📈 Roadmap

Ver [ROADMAP.md](ROADMAP.md) para el plan de desarrollo completo.

## 📝 License

Copyright © 2026. All rights reserved.

(End of file - total 103 lines)