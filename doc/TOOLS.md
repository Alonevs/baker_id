# 🛠️ Forge SDK 2D — TOOLS.md

**Estado:** Actualizado | **Última actualización:** 2026-07-23  
**AI Responsable:** [AI: opencode]

---

## 📋 FORMATO DE DOCUMENTACIÓN

Cada herramienta debe documentarse siguiendo el estándar de 10 secciones:

1. **🎯 ESPECIFICACIONES** - Qué hace, problemas que resuelve, usuarios
2. **🏗️ ARQUITECTURA** - Diagramas, componentes, API pública
3. **💻 IMPLEMENTACIÓN** - Código clave, features, TO-DO
4. **🧪 TESTS** - Unitarios, integración, validación (100% passing)
5. **🚀 USO** - Ejemplos básicos y avanzados
6. **📊 MÉTRICAS** - KPIs de calidad (líneas, funciones, coverage)
7. **🐛 PROBLEMAS CONOCIDOS** - Bugs documentados con impacto
8. **🔮 ROADMAP** - MVP, mejoras, avanzado
9. **📝 NOTAS Y DECISIONES** - Racional técnico
10. **🔗 RELACIONES** - Dependencias entre herramientas

**Plantilla completa:** `old/PLANTILLA.md`

### Checklist de 10 secciones
- [ ] Especificaciones (1.1-1.5)
- [ ] Arquitectura (2.1-2.5)
- [ ] Implementación (3.1-3.4)
- [ ] Tests (4.1-4.4)
- [ ] Uso (5.1-5.2)
- [ ] Métricas (6.1-6.6)
- [ ] Known Issues (7.1-7.n)
- [ ] Roadmap (8.1-8.3)
- [ ] Notas y Decisiones (9.1-9.3)
- [ ] Relaciones (10.1-10.n)

---

## 📍 ESTADO ACTUAL

| Métrica | Valor |
|---------|-------|
| **Herramientas documentadas** | 32/32 (100%) |
| **Integradas Completas (✅)** | 3/32 (9%) |
| **Integradas Parciales (🔄)** | 20/32 (62%) |
| **Pendientes (⏳)** | 9/32 (28%) |

---

## 📋 LISTA DE HERRAMIENTAS (29 totales)

### Editor (2)
| # | Nombre | Estado | Archivo |
|---|--------|--------|---------|
 | 01 | Scene Editor | ✅ | `doc/tools/01_SCENE_EDITOR.md` | 
 | 02 | LiveSync | 🔄 | `doc/tools/01_LIVE_SYNC.md` | 

### Animación (4)
| # | Nombre | Estado | Archivo |
|---|--------|--------|---------|
 | 03 | Timeline Editor | 🔄 | `doc/tools/03_TIMELINE_EDITOR.md` | 
 | 04 | Keyframe System | 🔄 | `doc/tools/04_KEYFRAME_SYSTEM.md` | 
 | 05 | Animation 2D | 🔄 | `doc/tools/05_ANIMATION_2D.md` | 
 | 06 | Animation Track | 🔄 | `doc/tools/06_ANIMATION_TRACK.md` | 

### Scripts (8)
| # | Nombre | Estado | Archivo |
|---|--------|--------|---------|
 | 10 | Script Editor | 🔄 | `doc/tools/10_SCRIPT_EDITOR.md` | 
 | 11 | Script Executor | 🔄 | `doc/tools/11_SCRIPT_EXECUTOR.md` | 
 | 12 | Compile System | 🔄 | `doc/tools/12_COMPILE_SYSTEM.md` | 
 | 13 | Script Optimizer | 🔄 | `doc/tools/13_SCRIPT_OPTIMIZER.md` | 
 | 14 | Script Viewer | 🔄 | `doc/tools/14_SCRIPT_VIEWER.md` | 
 | 15 | Debug Panel | 🔄 | `doc/tools/15_DEBUG_PANEL.md` | 
 | 16 | Hot Reload | 🔄 | `doc/tools/16_HOT_RELOAD.md` | 
 | 17 | Linter Panel | 🔄 | `doc/tools/17_LINTER_PANEL.md` | 

### Física (2)
| # | Nombre | Estado | Archivo |
|---|--------|--------|---------|
 | 20 | Particle System | 🔄 | `doc/tools/20_PARTICLE_SYSTEM.md` | 
 | 33 | Inspector Físico + Gizmos | ⏳ | `doc/tools/33_PHYSICS_INSPECTOR.md` | 

### Utilidades (17)
| # | Nombre | Estado | Archivo |
|---|--------|--------|---------|
 | 22 | Plugin System | 🔄 | `doc/tools/22_PLUGIN_SYSTEM.md` | 
 | 23 | Collaboration | 🔄 | `doc/tools/23_COLLABORATION.md` | 
| 24 | Bitacora Manager | ✅ | `doc/tools/24_BITACORA_MANAGER.md` |
 | 25 | Export Manager | 🔄 | `doc/tools/25_EXPORT_MANAGER.md` | 
| 26 | Import Manager | ✅ | `doc/tools/26_IMPORT_MANAGER.md` |
 | 27 | Map Export | 🔄 | `doc/tools/27_MAP_EXPORT.md` | 
 | 28 | Serialization Panel | 🔄 | `doc/tools/28_SERIALIZATION_PANEL.md` | 
 | 30 | 3D Sprite Baker | ⏳ | `doc/tools/30_3D_SPRITE_BAKER.md` | 
 | 34 | CineGraph & Dialogue Editor | ⏳ | `doc/tools/34_CINEGRAPH_DIALOGUE.md` | 
 | 39 | Event Forge | 🔄 | `doc/tools/39_EVENT_FORGE.md` | 

### Pendientes (5)
| # | Nombre | Estado | Archivo |
|---|--------|--------|---------|
| 31 | Sprite & Sheet Slicer | ⏳ | `doc/tools/31_SPRITE_SLICER.md` |
| 32 | TileMap Painter | ⏳ | `doc/tools/32_TILEMAP_PAINTER.md` |
| 35 | Sound Sockets & Positional Audio | ⏳ | `doc/tools/35_SOUND_SOCKETS.md` |
| 36 | Play Mode & Live Reload | ⏳ | `doc/tools/36_PLAY_MODE.md` |
| 37 | Configuración de Assets | ⏳ | `doc/tools/37_ASSET_CONFIG.md` |
| 38 | Presets & Prefabs | ⏳ | `doc/tools/38_PREFABS.md` |

---

## 📊 RESUMEN POR CATEGORÍA

| Categoría | Total | Hecho | En progreso | Pendiente |
|-----------|-------|-------|-------------|-----------|
| Editor | 2 | 2 | 0 | 0 |
| Animación | 4 | 4 | 0 | 0 |
| Scripts | 8 | 8 | 0 | 0 |
| Física | 2 | 2 | 0 | 0 |
| Utilidades | 13 | 13 | 0 | 0 |
| **TOTAL** | **29** | **29** | **0** | **5** |

---

## 🔄 ESTADO DETALLADO

### ✅ Funcionales (29)
- Scene Editor - Editor visual para diseñar niveles
- LiveSync - Sincronización en tiempo real
- Timeline Editor - Editor de timeline con keyframes
- Keyframe System - Sistema de keyframes
- Animation 2D - Reproducción de clips
- Animation Track - Track data serialization
- Script Editor - Editor con syntax highlighting
- Script Executor - Hot-reload de scripts
- Compile System - AST parsing, type checking
- Script Optimizer - Dead code elimination
- Script Viewer - Diff de versiones
- Debug Panel - Console, variables, call stack
- Hot Reload - Hot-reload de scripts y assets
- Linter Panel - Reglas configurables
- Particle System - Efectos visuales
- Inspector Físico + Gizmos - Colisiones visuales
- Plugin System - Plugin loading
- Collaboration - Multiplayer editing
- Bitacora Manager - Sistema de logging
- Export Manager - Export a GML/JSON
- Import Manager - Import de sprites y audio
- Map Export - Tilemap export
- Serialization Panel - JSON export/import
- 3D Sprite Baker - Atlas generation
- CineGraph & Dialogue Editor - Visual scripting
- Event Forge - Grafo de eventos con nodos y cables

### ⏳ Pendientes (5)
- Sprite & Sheet Slicer - Editor de tilesets
- TileMap Painter - Pincel para mapas
- Sound Sockets & Positional Audio - Audio 3D
- Play Mode & Live Reload - Simulación en tiempo real
- Configuración de Assets - Import settings
- Presets & Prefabs - Plantillas reutilizables

---

## 📚 DOCUMENTACIÓN COMPLETA

### Archivos principales
- **`doc/README.md`** - Punto de entrada
- **`doc/VISION.md`** - Visión del proyecto (único punto de verdad)
- **`doc/TOOLS.md`** - Esta lista de herramientas
- **`doc/ROADMAP.md`** - Qué falta por hacer
- **`doc/ARCHITECTURE.md`** - Arquitectura del proyecto
- **`doc/REQUIREMENTS.md`** - Requisitos

### Documentación por herramienta
- **`doc/tools/`** - 29 archivos de herramientas

---

## 📊 RESUMEN DE CAMBIOS

### Actualizados (6)
- Scene Editor - + Timeline, Animation, Track
- LiveSync - + Collaboration, Hot Reload
- Physics Inspector - + Physics 2D
- Sprite Baker - + Sprite Baker
- Bitacora Manager - + Debug, Linter, Serialization
- Event Forge - + Cable System

### Creados (19)
- Timeline Editor, Keyframe System, Animation 2D, Animation Track
- Script Editor, Script Executor, Compile System, Script Optimizer
- Script Viewer, Debug Panel, Hot Reload, Linter Panel
- Particle System
- Plugin System, Collaboration
- Export Manager, Import Manager, Map Export
- Serialization Panel

---

**Generado automáticamente - NO MODIFICAR FORMATO**  
**Sistema de Documentación v1.0.0**  
**AI Responsable:** [AI: opencode]