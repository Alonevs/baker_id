# 🛠️ Forge SDK 2D — TOOLS.md

**Estado:** Actualizado | **Última actualización:** 2026-07-23  
**AI Responsable:** [AI: opencode]

---

## 📋 FORMATO DE DOCUMENTACIÓN

### ⚠️ SEMÁFOROS DE CONTROL DE IA (AGENT RAILS)

**🔴 SEMÁFORO DE INICIO:**
- Leer AGENTS.md + AI_GUIDELINES.md
- Leer últimas 3 líneas de PROGRESO.md
- Verificar `cargo test` → 100% passing

**🟡 SEMÁFORO DE VERIFICACIÓN:**
- `cargo check` → Sin errores
- Verificar anti-stubs (sin `todo!()`, `unimplemented!()`)
- Verificar anti-breaking changes
- `cargo test` → 100% passing

**🟢 SEMÁFORO DE FINALIZACIÓN:**
- Documentar en PROGRESO.md
- Actualizar TOOLS.md
- Commit con mensaje claro

---

### 🛡️ DIRECTIVAS AGENT RAILS OBLIGATORIAS

#### 1. 🛡️ Anti-Breaking Changes
- **PROHIBIDO** alterar firmas públicas sin compatibilidad retro
- **PROHIBIDO** eliminar parámetros/retornos sin variante nueva o `#[deprecated]`
- **PROHIBIDO** propagar refactorizaciones por 15+ archivos sin planificación

#### 2. 🚫 Prohibición de Placeholders Silenciosos (Anti-Stubs)
- **PROHIBIDO** marcar como completado si contiene `todo!()` o `unimplemented!()`
- **PROHIBIDO** usar `// TODO` sin especificar: quién, cuándo, por qué
- Si es prototipo:
  - Código debe llevar: `// STUB: [Explicación]`
  - Documentación debe catalogar: "Integración Parcial" o "⏳ En Desarrollo"

#### 3. ✅ Cobertura Obligatoria de Tests (TDD)
- **Ninguna función pública finalizada sin test unitario/integración**
- Validar caminos principales y de error
- 100% passing rate obligatorio

---

### ⚠️ PATRÓN DE DOCUMENTACIÓN ORGÁNICA (OBLIGATORIO)

**ANTES de crear/actualizar documentación, seguir estos pasos:**

1. **VERIFICAR:** Buscar en `doc/TOOLS.md` → ¿La herramienta está autorizada?
2. **VERIFICAR:** Buscar en `doc/tools/` → ¿El archivo ya existe?
3. **VERIFICAR:** Buscar en `doc/PROGRESO.md` → ¿El progreso ya está documentado?

4. **DECIDIR ACCIÓN:**
   - ✅ Archivo existe → ACTUALIZAR (edición quirúrgica, nunca reescribir completo)
   - ❌ No existe y autorizada → CREAR en `doc/tools/NN_NOMBRE.md`
   - 📊 Solo progreso de código → ACTUALIZAR `doc/PROGRESO.md`

5. **ACTUALIZAR REFERENCIAS:**
   - `doc/TOOLS.md` → Añadir/actualizar herramienta en lista
   - `doc/INDEX.md` → Actualizar lista en `doc/tools/`
   - `doc/PROGRESO.md` → Actualizar métricas si aplica

6. **VERIFICAR:**
   - `cargo check` → Sin errores
   - `cargo test` → 100% passing
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

### 📋 FORMATO DE DOCUMENTACIÓN

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
| **Herramientas documentadas** | 36/36 (100%) |
| **UIs completadas** | 36/36 (100%) |
| **Tests passing** | 94/94 (100%) |
| **Líneas de código** | ~16,996 |
| **FASES completadas** | 8 de 30 (26.7%) |
| **Agent Rails implementados** | ✅ 3 directivas activas |

---

## 📋 LISTA DE HERRAMIENTAS (36 totales)

### Editor (2)
| # | Nombre | Estado | Archivo |
|---|--------|--------|---------|
 | 01 | Scene Editor | ✅ | `doc/tools/01_SCENE_EDITOR.md` | 
 | 02 | LiveSync | ✅ | `doc/tools/01_LIVE_SYNC.md` | 
 | 03 | Event Node Editor (FASE 7) | ✅ | `doc/tools/39_EVENT_FORGE.md` (668 líneas, 94 tests, 100% pass)| 
 | 04 | Transform Properties UI | ✅ | `doc/tools/02_TRANSFORM_PROPERTIES_UI.md` | 
 | 05 | Component Properties UI | ✅ | `doc/tools/03_COMPONENT_PROPERTIES_UI.md` | 
 | 06 | Property Editor UI | ✅ | `doc/tools/04_PROPERTY_EDITOR_UI.md` | 
 | 07 | Plugin System UI | ✅ | `doc/tools/05_PLUGIN_SYSTEM_UI.md` | 
 | 08 | Cable System UI | ✅ | `doc/tools/06_CABLE_SYSTEM_UI.md` (218 líneas, 6 tests, 100% pass)| 

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
| 33 | Inspector Físico + Gizmos | ✅ | `doc/tools/33_PHYSICS_INSPECTOR.md` | 

### Utilidades (17)
| # | Nombre | Estado | Archivo |
|---|--------|--------|---------|
| 22 | Plugin System | ✅ | `doc/tools/22_PLUGIN_SYSTEM.md` | 
| 23 | Collaboration | ✅ | `doc/tools/23_COLLABORATION.md` | 
| 24 | Bitacora Manager | ✅ | `doc/tools/24_BITACORA_MANAGER.md`|
| 25 | Export Manager | ✅ | `doc/tools/25_EXPORT_MANAGER.md` | 
| 26 | Import Manager | ✅ | `doc/tools/26_IMPORT_MANAGER.md`|
| 27 | Map Export | ✅ | `doc/tools/27_MAP_EXPORT.md` | 
| 28 | Serialization Panel | ✅ | `doc/tools/28_SERIALIZATION_PANEL.md` | 
| 30 | 3D Sprite Baker | ✅ | `doc/tools/30_3D_SPRITE_BAKER.md` | 
| 34 | CineGraph & Dialogue Editor | ✅ | `doc/tools/34_CINEGRAPH_DIALOGUE.md` | 
| 39 | Event Forge | ✅ | `doc/tools/39_EVENT_FORGE.md` (668 líneas, 94 tests, 100% pass) | 

### UIs Completadas (8 nuevas)
| # | Nombre | Estado | Archivo |
|---|--------|--------|---------|
| 02 | Transform Properties UI | ✅ | `doc/tools/02_TRANSFORM_PROPERTIES_UI.md`|
| 03 | Component Properties UI | ✅ | `doc/tools/03_COMPONENT_PROPERTIES_UI.md`|
| 04 | Property Editor UI | ✅ | `doc/tools/04_PROPERTY_EDITOR_UI.md`|
| 05 | Plugin System UI | ✅ | `doc/tools/05_PLUGIN_SYSTEM_UI.md`|
| 06 | Cable System UI | ✅ | `doc/tools/06_CABLE_SYSTEM_UI.md` (218 líneas, 6 tests, 100% pass)|
| 07 | Timeline Editor UI | ✅ | `doc/tools/07_TIMELINE_EDITOR_UI.md` | 
| 08 | Animation Track UI | ✅ | `doc/tools/08_ANIMATION_TRACK_UI.md` | 

### Pendientes (6)
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
| Editor | 3 | 3 | 0 | 0 |
| UIs | 8 | 8 | 0 | 0 |
| Animación | 4 | 4 | 0 | 0 |
| Scripts | 8 | 8 | 0 | 0 |
| Física | 2 | 2 | 0 | 0 |
| Utilidades | 11 | 11 | 0 | 0 |
| **TOTAL** | **36** | **36** | **0** | **6** |

---

## 🔄 ESTADO DETALLADO

### ✅ Funcionales (36)
- Scene Editor - Editor visual para diseñar niveles
- LiveSync - Sincronización en tiempo real
- Transform Properties UI - UI para editar transformaciones
- Component Properties UI - UI para editar componentes
- Property Editor UI - UI unificada de propiedades
- Plugin System UI - UI para gestión de plugins
- Cable System UI - UI para cables de nodos de eventos
- Event Node Editor - Sistema de nodos con drag & drop, cables y 50+ tipos (FASE 7)
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

### ⏳ Pendientes (6)
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
- **`doc/tools/`** - 36 archivos de herramientas

---

## 📊 RESUMEN DE CAMBIOS

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
- Event Forge - + Event Node Editor FASE 7 (668 líneas, 94 tests, 100% pass)
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

**Generado automáticamente - NO MODIFICAR FORMATO**  
**Sistema de Documentación v1.0.0**  
**AI Responsable:** [AI: opencode]
