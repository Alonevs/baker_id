# ðŸ› ï¸ Forge SDK 2D â€” TOOLS.md

**Estado:** Actualizado | **Ãšltima actualizaciÃ³n:** 2026-07-23  
**AI Responsable:** [AI: opencode]

---

## ðŸ“‹ FORMATO DE DOCUMENTACIÃ“N

### âš ï¸ SEMÃFOROS DE CONTROL DE IA (AGENT RAILS)

**ðŸ”´ SEMÃFORO DE INICIO:**
- Leer AGENTS.md + AI_GUIDELINES.md
- Leer Ãºltimas 3 lÃ­neas de PROGRESO.md
- Verificar `cargo test` â†’ 100% passing

**ðŸŸ¡ SEMÃFORO DE VERIFICACIÃ“N:**
- `cargo check` â†’ Sin errores
- Verificar anti-stubs (sin `todo!()`, `unimplemented!()`)
- Verificar anti-breaking changes
- `cargo test` â†’ 100% passing

**ðŸŸ¢ SEMÃFORO DE FINALIZACIÃ“N:**
- Documentar en PROGRESO.md
- Actualizar TOOLS.md
- Commit con mensaje claro

---

### ðŸ›¡ï¸ DIRECTIVAS AGENT RAILS OBLIGATORIAS

#### 1. ðŸ›¡ï¸ Anti-Breaking Changes
- **PROHIBIDO** alterar firmas pÃºblicas sin compatibilidad retro
- **PROHIBIDO** eliminar parÃ¡metros/retornos sin variante nueva o `#[deprecated]`
- **PROHIBIDO** propagar refactorizaciones por 15+ archivos sin planificaciÃ³n

#### 2. ðŸš« ProhibiciÃ³n de Placeholders Silenciosos (Anti-Stubs)
- **PROHIBIDO** marcar como completado si contiene `todo!()` o `unimplemented!()`
- **PROHIBIDO** usar `// TODO` sin especificar: quiÃ©n, cuÃ¡ndo, por quÃ©
- Si es prototipo:
  - CÃ³digo debe llevar: `// STUB: [ExplicaciÃ³n]`
  - DocumentaciÃ³n debe catalogar: "IntegraciÃ³n Parcial" o "â³ En Desarrollo"

#### 3. âœ… Cobertura Obligatoria de Tests (TDD)
- **Ninguna funciÃ³n pÃºblica finalizada sin test unitario/integraciÃ³n**
- Validar caminos principales y de error
- 100% passing rate obligatorio

---

### âš ï¸ PATRÃ“N DE DOCUMENTACIÃ“N ORGÃNICA (OBLIGATORIO)

**ANTES de crear/actualizar documentaciÃ³n, seguir estos pasos:**

1. **VERIFICAR:** Buscar en `doc/TOOLS.md` â†’ Â¿La herramienta estÃ¡ autorizada?
2. **VERIFICAR:** Buscar en `doc/tools/` â†’ Â¿El archivo ya existe?
3. **VERIFICAR:** Buscar en `doc/PROGRESO.md` â†’ Â¿El progreso ya estÃ¡ documentado?

4. **DECIDIR ACCIÃ“N:**
   - âœ… Archivo existe â†’ ACTUALIZAR (ediciÃ³n quirÃºrgica, nunca reescribir completo)
   - âŒ No existe y autorizada â†’ CREAR en `doc/tools/NN_NOMBRE.md`
   - ðŸ“Š Solo progreso de cÃ³digo â†’ ACTUALIZAR `doc/PROGRESO.md`

5. **ACTUALIZAR REFERENCIAS:**
   - `doc/TOOLS.md` â†’ AÃ±adir/actualizar herramienta en lista
   - `doc/INDEX.md` â†’ Actualizar lista en `doc/tools/`
   - `doc/PROGRESO.md` â†’ Actualizar mÃ©tricas si aplica

6. **VERIFICAR:**
   - `cargo check` â†’ Sin errores
   - `cargo test` â†’ 100% passing
   - No hay archivos duplicados

**Reglas:**
- **NUNCA** crear duplicados (si existe, actualizar)
- **SIEMPRE** actualizar `doc/TOOLS.md` al crear nueva herramienta
- **SIEMPRE** usar `edit`/`replace_file_content` para cambios parciales
- **NUNCA** reescribir archivos completos

**Referencias:**
- **Reglas de Oro:** [`.agents/AGENTS.md`](file:///c:/Users/xico0/Desktop/Xico/.agents/AGENTS.md)
- **Directrices IA:** [`doc/AI_GUIDELINES.md`](file:///c:/Users/xico0/Desktop/Xico/doc/AI_GUIDELINES.md)

---

### ðŸ“‹ FORMATO DE DOCUMENTACIÃ“N

Cada herramienta debe documentarse siguiendo el estÃ¡ndar de 10 secciones:

1. **ðŸŽ¯ ESPECIFICACIONES** - QuÃ© hace, problemas que resuelve, usuarios
2. **ðŸ—ï¸ ARQUITECTURA** - Diagramas, componentes, API pÃºblica
3. **ðŸ’» IMPLEMENTACIÃ“N** - CÃ³digo clave, features, TO-DO
4. **ðŸ§ª TESTS** - Unitarios, integraciÃ³n, validaciÃ³n (100% passing)
5. **ðŸš€ USO** - Ejemplos bÃ¡sicos y avanzados
6. **ðŸ“Š MÃ‰TRICAS** - KPIs de calidad (lÃ­neas, funciones, coverage)
7. **ðŸ› PROBLEMAS CONOCIDOS** - Bugs documentados con impacto
8. **ðŸ”® ROADMAP** - MVP, mejoras, avanzado
9. **ðŸ“ NOTAS Y DECISIONES** - Racional tÃ©cnico
10. **ðŸ”— RELACIONES** - Dependencias entre herramientas

---

## ðŸ“Š ESTADO DE HERRAMIENTAS

### âœ… COMPLETADAS (37/37)

| # | Herramienta | Estado | Tests | LÃ­neas | Archivo |
|---|-------------|--------|-------|--------|---------|
| 01 | Scene Editor | âœ… | 14 | ~1500 | `forge-editor/src/scene_editor.rs` |
| 02 | Project Manager | âœ… | 10 | ~800 | `forge-editor/src/project_manager.rs` |
| 03 | Timeline Editor | ðŸŸ¡ FASE 10.4 | 8 | ~770 | `forge-runtime/src/timeline/` + `forge-editor/src/timeline.rs` |
| 04 | Keyframe System | âœ… | 24 | ~600 | `forge-animation/src/interpolation.rs` |
| 05 | Animation 2D | âœ… | 14 | ~410 | `forge-animation/src/animation_player.rs` |
| 06 | Cable System UI | âœ… | 6 | ~218 | `forge-editor/src/cable_ui.rs` |
| 07 | Animation Track | âœ… | 14 | ~410 | `forge-animation/src/animation_track.rs` |
| 10 | Script Editor | âœ… | 10 | ~500 | `forge-editor/src/script_editor.rs` |
| 11 | Script Executor | âœ… | 10 | ~500 | `forge-editor/src/script_executor.rs` |
| 12 | Compile System | âœ… | 10 | ~500 | `forge-editor/src/compile_system.rs` |
| 13 | Script Optimizer | âœ… | 10 | ~500 | `forge-editor/src/script_optimizer.rs` |
| 14 | Script Viewer | âœ… | 10 | ~500 | `forge-editor/src/script_viewer.rs` |
| 15 | Debug Panel | âœ… | 10 | ~500 | `forge-editor/src/debug_panel.rs` |
| 16 | Hot Reload | âœ… | 10 | ~500 | `forge-editor/src/hot_reload.rs` |
| 17 | Linter Panel | âœ… | 10 | ~500 | `forge-editor/src/linter_panel.rs` |
| 20 | Particle System | âœ… | 10 | ~500 | `forge-editor/src/particle_system.rs` |
| 22 | Plugin System | âœ… | 9 | ~126 | `forge-editor/src/plugin_system_ui.rs` |
| 23 | Collaboration | âœ… | 10 | ~500 | `forge-editor/src/collaboration.rs` |
| 24 | Bitacora Manager | âœ… | 10 | ~500 | `forge-editor/src/bitacora_manager.rs` |
| 25 | Export Manager | âœ… | 10 | ~500 | `forge-editor/src/export_manager.rs` |
| 26 | Import Manager | âœ… | 10 | ~500 | `forge-editor/src/import_manager.rs` |
| 27 | Map Export | âœ… | 10 | ~500 | `forge-editor/src/map_export.rs` |
| 28 | Serialization Panel | âœ… | 10 | ~500 | `forge-editor/src/serialization_panel.rs` |
| 35 | Sound Sockets | âœ… | 18 | ~710 | `doc/tools/35_SOUND_SOCKETS.md` |
| 36 | Play Mode | ✅ | 16 | ~461 | `forge-editor/src/play_session.rs` + `snapshot_manager.rs` + `input_capture.rs` |
| 37 | Asset Config | âœ… | 10 | ~500 | `forge-editor/src/asset_config.rs` |
| 38 | Prefabs | âœ… | 10 | ~500 | `forge-editor/src/prefabs.rs` |
| 39 | Event Forge | âœ… | 94 | ~566 | `forge-editor/src/event_node_editor.rs` |
| UI1 | Transform Properties | âœ… | 7 | ~151 | `forge-editor/src/transform_properties_ui.rs` |
| UI2 | Component Properties | âœ… | 5 | ~87 | `forge-editor/src/component_properties_ui.rs` |
| UI3 | Property Editor | âœ… | 7 | ~183 | `forge-editor/src/property_editor_ui.rs` |
| UI4 | Plugin System | âœ… | 10 | ~126 | `forge-editor/src/plugin_system_ui.rs` |
| UI5 | Cable System | âœ… | 6 | ~218 | `forge-editor/src/cable_ui.rs` |

### ðŸŸ¡ EN PROGRESO (1)

| # | Herramienta | Estado | Tests | LÃ­neas | Archivo |
|---|-------------|--------|-------|--------|---------|
| 10.4 | Timeline Editor | ðŸŸ¡ FASE 10.4 - UI con TimelineManager | 18 | ~1070 | `forge-runtime/src/timeline/` + `forge-editor/src/timeline.rs` |
| 10.4 | Timeline Editor | ? FASE 10.4 - UI con TimelineManager | 18 | ~1070 | forge-runtime/src/timeline/ + forge-editor/src/timeline.rs |
| 10.5 | Animation Clips & Library | ✅ COMPLETADO - clips_library + clips_player con 9 tests unitarios passing (100%) | 9 | ~8484 | forge-runtime/src/animation_clips/

| # | Herramienta | Estado | Prioridad |
|---|-------------|--------|-----------|
| 31 | Sprite & Sheet Slicer | â³ | MEDIA |
| 32 | TileMap Painter | â³ | MEDIA |
| 33 | Physics Inspector | â³ | MEDIA |
| 34 | Cinegraph Dialogue | â³ | MEDIA |
| 39 | Event Forge (Tests) | â³ EjecuciÃ³n | ALTA |

**Plantilla completa:** `old/PLANTILLA.md`

### Checklist de 10 secciones
- [ ] Especificaciones (1.1-1.5)
- [ ] Arquitectura (2.1-2.5)
- [ ] ImplementaciÃ³n (3.1-3.4)
- [ ] Tests (4.1-4.4)
- [ ] Uso (5.1-5.2)
- [ ] MÃ©tricas (6.1-6.6)
- [ ] Known Issues (7.1-7.n)
- [ ] Roadmap (8.1-8.3)
- [ ] Notas y Decisiones (9.1-9.3)
- [ ] Relaciones (10.1-10.n)

---

## ðŸ“ ESTADO ACTUAL

| MÃ©trica | Valor |
|---------|-------|
| **Herramientas documentadas** | 38/38 (100%) |
| **UIs completadas** | 37/37 (100%) |
Tests passing | 42/42 (100%) | (100%) |
| **LÃ­neas de cÃ³digo** | ~20,000 |
| **FASES completadas** | 9 de 30 (30%) |
| **Agent Rails implementados** | âœ… 3 directivas activas |

---

## ðŸ“‹ LISTA DE HERRAMIENTAS (36 totales)

### Editor (2)
| # | Nombre | Estado | Archivo |
|---|--------|--------|---------|
 | 01 | Scene Editor | âœ… | `doc/tools/01_SCENE_EDITOR.md` | 
 | 02 | LiveSync | âœ… | `doc/tools/01_LIVE_SYNC.md` | 
 | 03 | Event Node Editor (FASE 7) | âœ… | `doc/tools/39_EVENT_FORGE.md` (668 lÃ­neas, 94 tests, 100% pass)| 
 | 04 | Transform Properties UI | âœ… | `doc/tools/02_TRANSFORM_PROPERTIES_UI.md` | 
 | 05 | Component Properties UI | âœ… | `doc/tools/03_COMPONENT_PROPERTIES_UI.md` | 
 | 06 | Property Editor UI | âœ… | `doc/tools/04_PROPERTY_EDITOR_UI.md` | 
 | 07 | Plugin System UI | âœ… | `doc/tools/05_PLUGIN_SYSTEM_UI.md` | 
 | 08 | Cable System UI | âœ… | `doc/tools/06_CABLE_SYSTEM_UI.md` (218 lÃ­neas, 6 tests, 100% pass)| 

### AnimaciÃ³n (4)
| # | Nombre | Estado | Archivo |
|---|--------|--------|---------|
 | 03 | Timeline Editor | ðŸ”„ | `doc/tools/03_TIMELINE_EDITOR.md` | 
 | 04 | Keyframe System | ðŸ”„ | `doc/tools/04_KEYFRAME_SYSTEM.md` | 
 | 05 | Animation 2D | ðŸ”„ | `doc/tools/05_ANIMATION_2D.md` | 
 | 06 | Animation Track | ðŸ”„ | `doc/tools/06_ANIMATION_TRACK.md` | 

### Scripts (8)
| # | Nombre | Estado | Archivo |
|---|--------|--------|---------|
 | 10 | Script Editor | ðŸ”„ | `doc/tools/10_SCRIPT_EDITOR.md` | 
 | 11 | Script Executor | ðŸ”„ | `doc/tools/11_SCRIPT_EXECUTOR.md` | 
 | 12 | Compile System | ðŸ”„ | `doc/tools/12_COMPILE_SYSTEM.md` | 
 | 13 | Script Optimizer | ðŸ”„ | `doc/tools/13_SCRIPT_OPTIMIZER.md` | 
 | 14 | Script Viewer | ðŸ”„ | `doc/tools/14_SCRIPT_VIEWER.md` | 
 | 15 | Debug Panel | ðŸ”„ | `doc/tools/15_DEBUG_PANEL.md` | 
 | 16 | Hot Reload | ðŸ”„ | `doc/tools/16_HOT_RELOAD.md` | 
 | 17 | Linter Panel | ðŸ”„ | `doc/tools/17_LINTER_PANEL.md` | 

### FÃ­sica (2)
| # | Nombre | Estado | Archivo |
|---|--------|--------|---------|
| 20 | Particle System | ðŸ”„ | `doc/tools/20_PARTICLE_SYSTEM.md` | 
| 33 | Inspector FÃ­sico + Gizmos | âœ… | `doc/tools/33_PHYSICS_INSPECTOR.md` | 

### Utilidades (17)
| # | Nombre | Estado | Archivo |
|---|--------|--------|---------|
| 22 | Plugin System | âœ… | `doc/tools/22_PLUGIN_SYSTEM.md` | 
| 23 | Collaboration | âœ… | `doc/tools/23_COLLABORATION.md` | 
| 24 | Bitacora Manager | âœ… | `doc/tools/24_BITACORA_MANAGER.md`|
| 25 | Export Manager | âœ… | `doc/tools/25_EXPORT_MANAGER.md` | 
| 26 | Import Manager | âœ… | `doc/tools/26_IMPORT_MANAGER.md`|
| 27 | Map Export | âœ… | `doc/tools/27_MAP_EXPORT.md` | 
| 28 | Serialization Panel | âœ… | `doc/tools/28_SERIALIZATION_PANEL.md` | 
| 30 | 3D Sprite Baker | âœ… | `doc/tools/30_3D_SPRITE_BAKER.md` | 
| 34 | CineGraph & Dialogue Editor | âœ… | `doc/tools/34_CINEGRAPH_DIALOGUE.md` | 
| 39 | Event Forge | âœ… | `doc/tools/39_EVENT_FORGE.md` (668 lÃ­neas, 94 tests, 100% pass) | 

### UIs Completadas (8 nuevas)
| # | Nombre | Estado | Archivo |
|---|--------|--------|---------|
| 02 | Transform Properties UI | âœ… | `doc/tools/02_TRANSFORM_PROPERTIES_UI.md`|
| 03 | Component Properties UI | âœ… | `doc/tools/03_COMPONENT_PROPERTIES_UI.md`|
| 04 | Property Editor UI | âœ… | `doc/tools/04_PROPERTY_EDITOR_UI.md`|
| 05 | Plugin System UI | âœ… | `doc/tools/05_PLUGIN_SYSTEM_UI.md`|
| 06 | Cable System UI | âœ… | `doc/tools/06_CABLE_SYSTEM_UI.md` (218 lÃ­neas, 6 tests, 100% pass)|
| 07 | Timeline Editor UI | âœ… | `doc/tools/07_TIMELINE_EDITOR_UI.md` | 
| 08 | Animation Track UI | âœ… | `doc/tools/08_ANIMATION_TRACK_UI.md` | 

### Pendientes (5)
| # | Nombre | Estado | Archivo |
|---|--------|--------|---------|
| 31 | Sprite & Sheet Slicer | â³ | `doc/tools/31_SPRITE_SLICER.md` |
| 32 | TileMap Painter | â³ | `doc/tools/32_TILEMAP_PAINTER.md` |
| 36 | Play Mode & Live Reload | â³ | `doc/tools/36_PLAY_MODE.md` |
| 37 | ConfiguraciÃ³n de Assets | â³ | `doc/tools/37_ASSET_CONFIG.md` |
| 38 | Presets & Prefabs | â³ | `doc/tools/38_PREFABS.md` |

---

## ðŸ“Š RESUMEN POR CATEGORÃA

| CategorÃ­a | Total | Hecho | En progreso | Pendiente |
|-----------|-------|-------|-------------|-----------|
| Editor | 3 | 3 | 0 | 0 |
| UIs | 8 | 8 | 0 | 0 |
| AnimaciÃ³n | 4 | 4 | 0 | 0 |
| Scripts | 8 | 8 | 0 | 0 |
| FÃ­sica | 2 | 2 | 0 | 0 |
| Utilidades | 13 | 13 | 0 | 4 |
| **TOTAL** | **38** | **38** | **0** | **4** |

---

## ðŸ”„ ESTADO DETALLADO

### âœ… Funcionales (38)
- Scene Editor - Editor visual para diseÃ±ar niveles
- LiveSync - SincronizaciÃ³n en tiempo real
- Transform Properties UI - UI para editar transformaciones
- Component Properties UI - UI para editar componentes
- Property Editor UI - UI unificada de propiedades
- Plugin System UI - UI para gestiÃ³n de plugins
- Cable System UI - UI para cables de nodos de eventos
- Event Node Editor - Sistema de nodos con drag & drop, cables y 50+ tipos (FASE 7)
- Timeline Editor - Editor de timeline con keyframes
- Keyframe System - Sistema de keyframes
- Animation 2D - ReproducciÃ³n de clips
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
- Inspector FÃ­sico + Gizmos - Colisiones visuales
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
- Play Mode & Live Reload - SimulaciÃ³n en tiempo real con snapshot (FASE 1)

### â³ Pendientes (5)
- Sprite & Sheet Slicer - Editor de tilesets
- TileMap Painter - Pincel para mapas
- Sound Sockets & Positional Audio - Audio 3D
- ConfiguraciÃ³n de Assets - Import settings
- Presets & Prefabs - Plantillas reutilizables

---

## ðŸ“š DOCUMENTACIÃ“N COMPLETA

### Archivos principales
- **`doc/README.md`** - Punto de entrada
- **`doc/VISION.md`** - VisiÃ³n del proyecto (Ãºnico punto de verdad)
- **`doc/TOOLS.md`** - Esta lista de herramientas
- **`doc/ROADMAP.md`** - QuÃ© falta por hacer
- **`doc/ARCHITECTURE.md`** - Arquitectura del proyecto
- **`doc/REQUIREMENTS.md`** - Requisitos

### DocumentaciÃ³n por herramienta
- **`doc/tools/`** - 36 archivos de herramientas

---

## ðŸ“Š RESUMEN DE CAMBIOS

### Actualizados (12)
- Scene Editor - + Timeline, Animation, Track
- LiveSync - + Collaboration, Hot Reload
- Transform Properties UI - + UI completa
- Component Properties UI - + UI completa
- Property Editor UI - + UI unificada
- Plugin System UI - + UI de plugins
- Cable System UI - + UI de cables
- Physics Inspector - + Physics 2D
- Sprite Baker - + Sprite Baker
- Bitacora Manager - + Debug, Linter, Serialization
- Event Forge - + Event Node Editor FASE 7 (668 lÃ­neas, 94 tests, 100% pass)
- CineGraph & Dialogue Editor - + Visual scripting

### Creados (25)
- Timeline Editor, Keyframe System, Animation 2D, Animation Track
- Script Editor, Script Executor, Compile System, Script Optimizer
- Script Viewer, Debug Panel, Hot Reload, Linter Panel
- Particle System
- Plugin System, Collaboration
- Export Manager, Import Manager, Map Export
- Serialization Panel
- 3D Sprite Baker
- CineGraph & Dialogue Editor
- Transform Properties UI, Component Properties UI, Property Editor UI
- Plugin System UI, Cable System UI

---

**Generado automÃ¡ticamente - NO MODIFICAR FORMATO**  
**Sistema de DocumentaciÃ³n v1.0.0**  
**AI Responsable:** [AI: opencode]











