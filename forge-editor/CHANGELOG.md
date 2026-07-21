# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [2.0.0] - 2026-07-19

### Added
- **Property Panel API** (`property_panel.rs`)
  - `PropertyPanel::new()` - Crear nuevo panel
  - `set_selected_entity()` - Seleccionar entidad
  - `get_transform_properties()` - Obtener propiedades de transformación
  - `get_component_properties()` - Obtener propiedades de componentes
  - `get_script_properties()` - Obtener propiedades de scripts
  - `set_transform_props()` - Establecer propiedades de transformación
  - `set_component_props()` - Establecer propiedades de componentes
  - `set_script_props()` - Establecer propiedades de scripts
  - `set_position()`, `set_rotation()`, `set_scale()` - Control de transformaciones
  - `set_visible()` - Control de visibilidad
  - `add_component()`, `remove_component()` - Gestión de componentes
  - `add_script()`, `remove_script()` - Gestión de scripts

- **Timeline Editor API** (`timeline.rs`)
  - `TimelineEditor::new()` - Crear nuevo editor
  - `create_widgets()` - Generar UI
  - `get_property_panel()` - Acceder a panel de propiedades
  - `get_timeline_track()` - Obtener pista de timeline
  - `get_current_frame()` / `set_current_frame()` - Control de frame actual
  - `add_keyframe()` / `remove_keyframe()` - Gestión de keyframes
  - `interpolate()` - Interpolación de valores
  - `update()` - Actualizar editor

- **Transform Editor API** (`transform_editor.rs`)
  - `TransformEditor::new()` - Crear nuevo editor
  - `create_widgets()` - Generar UI
  - `get_transform_properties()` - Obtener propiedades
  - `get_component_properties()` - Obtener propiedades
  - `get_script_properties()` - Obtener propiedades
  - `set_selected_entity()` - Seleccionar entidad
  - `get_selected_entity()` - Obtener entidad seleccionada
  - `update()` - Actualizar editor

- **Component Editor API** (`component_editor.rs`)
  - `ComponentEditor::new()` - Crear nuevo editor
  - `create_widgets()` - Generar UI
  - `get_component_properties()` - Obtener propiedades
  - `get_transform_properties()` - Obtener propiedades
  - `get_script_properties()` - Obtener propiedades
  - `set_selected_entity()` - Seleccionar entidad
  - `update()` - Actualizar editor

- **Viewport API** (`ui/viewport.rs`)
  - `Viewport::new()` / `Viewport::default()` - Crear nuevo viewport
  - `reset_zoom()` - Restablecer zoom
  - `zoom_in()` / `zoom_out()` - Control de zoom
  - `set_zoom()` - Establecer zoom manual
  - `set_position()` - Mover cámara
  - `set_size()` - Establecer tamaño
  - `add_selection()` / `remove_selection()` - Gestión de selección
  - `is_moving()` - Verificar si cámara está moviéndose
  - `update()` - Actualizar viewport

### Tests
- **property_panel_test.rs** - 5 tests unitarios
  - `test_property_panel_new`
  - `test_property_panel_set_selected_entity`
  - `test_property_panel_set_transform_props`
  - `test_property_panel_set_component_props`
  - `test_property_panel_set_script_props`

- **timeline_test.rs** - 5 tests unitarios
  - `test_timeline_new`
  - `test_timeline_create_widgets`
  - `test_timeline_get_property_panel`
  - `test_timeline_get_timeline_track`
  - `test_timeline_get_current_frame`

- **transform_editor_test.rs** - 7 tests unitarios
  - `test_transform_editor_new`
  - `test_transform_editor_create_widgets`
  - `test_transform_editor_get_widgets`
  - `test_transform_editor_get_property_panel`
  - `test_transform_editor_get_transform_properties`
  - `test_transform_editor_get_component_properties`
  - `test_transform_editor_get_script_properties`

- **component_editor_test.rs** - 7 tests unitarios
  - `test_component_editor_new`
  - `test_component_editor_create_widgets`
  - `test_component_editor_get_widgets`
  - `test_component_editor_get_property_panel`
  - `test_component_editor_get_component_properties`
  - `test_component_editor_get_transform_properties`
  - `test_component_editor_get_script_properties`

### Documentation
- 4 ejemplos de uso en `examples/`
  - `property_panel.rs` - Ejemplo completo de Property Panel
  - `timeline.rs` - Ejemplo completo de Timeline Editor
  - `transform_editor.rs` - Ejemplo completo de Transform Editor
  - `component_editor.rs` - Ejemplo completo de Component Editor
  - `viewport.rs` - Ejemplo completo de Viewport

### Build
- ✅ 24 tests passing
- ✅ 0 tests failing
- ✅ Compilación exitosa sin errores

---

## [1.0.0] - 2026-07-19

### Added
- Initial project setup
- Basic UI structure with eframe + egui
- Core modules: physics_2d, particle_system, animation_2d
- Export manager for `.map` file format
- Dialog system with dialogue_editor

---

## [0.1.0] - 2026-07-19

### Added
- Project scaffolding
- Cargo.toml configuration
- Basic directory structure

(End of file - total 89 lines)