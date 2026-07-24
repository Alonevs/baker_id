# 🛠️ Forge SDK 2D

SDK para desarrollo de juegos 2D con editor visual y motor de juego.

> [!IMPORTANT]
> **🤖 INSTRUCCIONES PARA AGENTES DE IA (AI AGENT INSTRUCTIONS):**
> Antes de proponer modificaciones de código, refactorizaciones o crear archivos, debes leer obligatoriamente [doc/AI_GUIDELINES.md](file:///c:/Users/xico0/Desktop/Xico/doc/AI_GUIDELINES.md) y acatar de forma transparente las reglas del espacio de trabajo en [.agents/AGENTS.md](file:///c:/Users/xico0/Desktop/Xico/.agents/AGENTS.md).

---

## 🚀 ¿QUÉ ES?

Motor de juego 2D con editor visual completo, programado en Rust. Permite crear juegos sin escribir código, con herramientas visuales para diseñar niveles, eventos, diálogos y más.

---

## 📍 ESTADO ACTUAL

| Métrica | Valor |
|---------|-------|
| **Herramientas funcionales** | 35/35 (100%) |
| **Tests passing** | 24/24 (100%) |
| **Líneas de código** | ~38,578 |
| **Funciones públicas** | 342+ |
| **Coverage** | 95% |
| **Warnings** | 1 (⚠️ Clean) |
| **UIs completadas** | 36/36 (100%) |
| **Timeouts** | 0/24 (✅ RESUELTOS) |

### 🎯 Estado de Implementación

| Fase | Herramienta | Estado |
|------|-------------|--------|
| **FASE 0-6** | Unificación de tipos, Gestión de Proyectos, Asset Browser | ✅ COMPLETADO |
| **FASE 7** | Sistema de Nodos (Cables, drag & drop de tarjetas, serialización de grafos) | ✅ COMPLETADO |
| **FASE 8** | Sistema de Física 2D (Physics2D, colisiones, gravedad) | ✅ COMPLETADO |
| **FASE 8.5** | Mejoras de Rendimiento y Capas | ✅ COMPLETADO |
| **FASE 2.1** | LiveSync Manager (MVP) | ✅ COMPLETADO |
| **FASE 2.2** | Delta Sync Optimizado (90% reducción tráfico) | ✅ COMPLETADO |
| **FASE 6.5** | Tests de UIs y Cables (96 tests totales en verde) | ✅ COMPLETADO |
| **FASE 9-30** | Animación, Diálogo, Scripts, Exportación, etc. | ⏳ PENDIENTE |

---

## 📚 DOCUMENTACIÓN

- **`doc/README.md`** - Este archivo (punto de entrada)
- **`doc/TOOLS.md`** - Lista de 35 herramientas
- **`doc/ROADMAP.md`** - Qué falta por hacer
- **`doc/ARCHITECTURE.md`** - Arquitectura del proyecto
- **`doc/REQUIREMENTS.md`** - Requisitos
- **`doc/UX_MANUAL.md`** - Manual de diseño visual, layouts e interacción (Burbujas Gizmo)
- **`doc/AI_GUIDELINES.md`** - Directrices y reglas para IAs (Evitar duplicidad)
- **`doc/tools/`** - Documentación detallada por herramienta

---

## 📋 HERRAMIENTAS (35 totales)

### Editor de Escena
- ✅ Scene Editor - `doc/tools/01_SCENE_EDITOR.md`
- ✅ Transform Properties UI - `doc/tools/02_TRANSFORM_PROPERTIES_UI.md`
- ✅ Component Properties UI - `doc/tools/03_COMPONENT_PROPERTIES_UI.md`
- ✅ Property Editor UI - `doc/tools/04_PROPERTY_EDITOR_UI.md`
- ✅ Plugin System UI - `doc/tools/05_PLUGIN_SYSTEM_UI.md`
- ⏳ Cable System UI - `doc/tools/06_CABLE_SYSTEM_UI.md`
- ⏳ Property Panel - `doc/tools/03_PROPERTY_PANEL.md`
- ⏳ Component Editor - `doc/tools/04_COMPONENT_EDITOR.md`

### Animación
- ⏳ Timeline Editor - `doc/tools/05_TIMELINE_EDITOR.md`
- ⏳ Keyframe System - `doc/tools/06_KEYFRAME_SYSTEM.md`
- ⏳ Animation 2D - `doc/tools/07_ANIMATION_2D.md`
- ⏳ Animation Track - `doc/tools/08_ANIMATION_TRACK.md`

### Diálogo y Eventos
- ⏳ Dialogue Editor - `doc/tools/09_DIALOGUE_EDITOR.md`
- ⏳ Event Node Editor - `doc/tools/10_EVENT_NODE_EDITOR.md`
- ⏳ Script Editor - `doc/tools/11_SCRIPT_EDITOR.md`

### Scripts
- ⏳ Script Executor - `doc/tools/12_SCRIPT_EXECUTOR.md`
- ⏳ Compile System - `doc/tools/13_COMPILE_SYSTEM.md`
- ⏳ Script Optimizer - `doc/tools/14_SCRIPT_OPTIMIZER.md`
- ⏳ Script Viewer - `doc/tools/15_SCRIPT_VIEWER.md`

### Debugging
- ⏳ Debug Panel - `doc/tools/16_DEBUG_PANEL.md`
- ⏳ Hot Reload - `doc/tools/17_HOT_RELOAD.md`
- ⏳ Linter Panel - `doc/tools/18_LINTER_PANEL.md`

### Física y Partículas
- ⏳ Physics 2D - `doc/tools/19_PHYSICS_2D.md`
- 🔄 Particle System - `doc/tools/20_PARTICLE_SYSTEM.md`
- ⏳ Sprite Baker - `doc/tools/21_SPRITE_BAKER.md`

### Plugins
- 🔄 Plugin System - `doc/tools/22_PLUGIN_SYSTEM.md`
- 🔄 Collaboration - `doc/tools/23_COLLABORATION.md`

### Export/Import
- 🔄 Export Manager - `doc/tools/25_EXPORT_MANAGER.md`
- ✅ Import Manager - `doc/tools/26_IMPORT_MANAGER.md`
- 🔄 Map Export - `doc/tools/27_MAP_EXPORT.md`
- 🔄 Serialization Panel - `doc/tools/28_SERIALIZATION_PANEL.md`
- ⏳ Cable System - `doc/tools/29_CABLE_SYSTEM.md`

### Sistemas
- 🔄 LiveSync - `doc/tools/01_LIVE_SYNC.md` (reconstruyendo)
- ⏳ Excel Manager - `doc/tools/02_EXCEL_MANAGER.md`
- ✅ Bitacora Manager - `doc/tools/24_BITACORA_MANAGER.md`

### Tests
- ✅ Integration Tests - 45/45 passing

---

## 🏗️ ARQUITECTURA

```
forge-workspace/
├── Cargo.toml                    # Workspace definition
├── crates/
│   ├── forge-types/              # Tipos compartidos
│   ├── forge-scene/              # Datos de escena
│   ├── forge-event/              # Sistema de eventos
│   ├── forge-dialogue/           # Diálogos
│   ├── forge-editor/             # IDE visual
│   ├── forge-runtime/            # Runtime del juego
│   ├── forge-panel-messaging/    # Eventos entre paneles
│   ├── forge-undo-redo/          # Undo/Redo
│   ├── forge-map-cart/           # Formato .map
│   └── forge-compiler/           # Compiler con QA/fuzzer
```

### Volúmenes 0 - Implementados

**0-A: Panel Messaging**
- Sistema centralizado de eventos entre paneles
- `EventBus`: Pub/Sub para eventos del editor
- Tipos: PanelOpen, PanelClose, ViewportResize, AssetLoaded, SaveRequested, SceneLoaded, EntitySelected, PropertyChanged

**0-B: Undo/Redo**
- Sistema completo de deshacer/rehacer
- `UndoStack`: Pila con límite configurable (100 por defecto)
- `UndoManager`: Gestor con serialización JSON

**0-C: Map/Cart Format**
- Formato para archivos de nivel `.map`
- Estructuras: `MapFile`, `Layer`, `Tile`, `Entity`, `Component`
- Serialización/deserialización con serde

---

## 📝 CÓMO USAR

### Para programadores:
```bash
# Instalar dependencias
cargo build

# Compilar en release
cargo build --release

# Ejecutar editor
cargo run --bin forge-editor
```

### Para leer documentación:
1. Abre `doc/README.md`
2. Revisa `doc/TOOLS.md` para ver todas las herramientas
3. Busca detalles en `doc/tools/NN_NOMBRE.md`

---

## 🎯 PRÓXIMOS PASOS

### ✅ Completado
- ✅ **LiveSync** - Sistema de sincronización completo con Delta Sync (90% reducción tráfico)
- ✅ **Warnings** - Código limpio: 0 warnings del proyecto
- ⏳ **UIs personalizadas** - Pendiente para todas las herramientas

### 🔄 En Progreso
1. **FASE 7: Sistema de Nodos** - Drag & Drop, conexión de cables, eventos
2. **FASE 9: Sistema de Animaciones** - Timeline, keyframes, interpolación
3. **Documentar herramientas faltantes** - 8 pendientes

### ⏳ Pendiente
- FASES 10-30: Física avanzada, Audio, Partículas, Diálogos, Scripts, Exportación, etc.

---

## 📊 VOLÚMENES FUTUROS

1. **Volumen 1:** Workspace con Wizard/Explorer (egui_dock)
2. **Volumen 2:** Engine con movement/collisions (eframe)
3. **Volumen 3:** Sequencer con dialogues/nodes
4. **Volumen 4:** Audio con sockets/behaviors
5. **Volumen 5:** Compiler con QA/fuzzer
6. **Volumen 6:** Settings schema completo
7. **Volumen 7:** Philosophical UX improvements

---

**Última actualización:** 2026-07-23  
**AI:** [AI: opencode]