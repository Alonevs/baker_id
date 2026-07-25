# 📚 DOCUMENTACIÓN - ESTRUCTURA FINAL

**Versión:** 1.2.0  
**Fecha:** 2026-07-25  
**AI:** [AI: opencode]

---

## 🗂️ ESTRUCTURA DE CARPETAS

```
doc/
├── VISION.md                              ← Punto de verdad (único)
├── ARCHITECTURE.md                        ← Arquitectura técnica
├── TOOLS.md                               ← Lista de herramientas
├── ROADMAP.md                             ← Qué falta por hacer
├── REQUIREMENTS.md                        ← Requisitos
├── README.md                              ← Entrada principal
├── PROGRESO.md                            ← Registro de Progreso y Handoff
├── AI_GUIDELINES.md                       ← Directrices para IA
├── UX_MANUAL.md                           ← Manual de diseño UX/UI
│
├── tools/                                 ← Documentación de herramientas
│   ├── 01_SCENE_EDITOR.md                 ← Editor de escena
│   ├── 01_LIVE_SYNC.md                    ← Sincronización
│   ├── Uis.md                             ← UIs (Transform, Component, Property, Plugin)
│   ├── 03_TIMELINE_EDITOR.md              ← Editor de timeline
│   ├── 04_KEYFRAME_SYSTEM.md              ← Sistema de keyframes
│   ├── 05_ANIMATION_2D.md                 ← Reproducción de clips
│   ├── 05_ANIMATION_TRACK.md              ← Track data serialization
│   ├── 06_ANIMATION_TRACK.md              ← Track UI
│   ├── 06_CABLE_SYSTEM_UI.md              ← UI de cables
│   ├── 10_SCRIPT_EDITOR.md                ← Editor de scripts
│   ├── 11_SCRIPT_EXECUTOR.md              ← Hot-reload de scripts
│   ├── 12_COMPILE_SYSTEM.md               ← AST parsing
│   ├── 13_SCRIPT_OPTIMIZER.md             ← Dead code elimination
│   ├── 14_SCRIPT_VIEWER.md                ← Diff de versiones
│   ├── 15_DEBUG_PANEL.md                  ← Console + variables
│   ├── 16_HOT_RELOAD.md                   ← Hot-reload assets
│   ├── 17_LINTER_PANEL.md                 ← Reglas configurables
│   ├── 20_PARTICLE_SYSTEM.md              ← Efectos visuales
│   ├── 22_PLUGIN_SYSTEM.md                ← Plugin loading
│   ├── 23_COLLABORATION.md                ← Multiplayer editing
│   ├── 24_BITACORA_MANAGER.md             ← Logging
│   ├── 25_EXPORT_MANAGER.md               ← Export a GML/JSON
│   ├── 26_IMPORT_MANAGER.md               ← Import sprites/audio
│   ├── 27_MAP_EXPORT.md                   ← Tilemap export
│   ├── 28_SERIALIZATION_PANEL.md          ← JSON export/import
│   ├── 30_3D_SPRITE_BAKER.md              ← Atlas generation
│   ├── 31_SPRITE_SLICER.md                ← Editor de atlas
│   ├── 32_TILEMAP_PAINTER.md              ← Pincel de mapas
│   ├── 33_PHYSICS_INSPECTOR.md            ← Físicas + Gizmos
│   ├── 34_CINEGRAPH_DIALOGUE.md           ← Diálogos
│   ├── 35_SOUND_SOCKETS.md                ← Audio 3D
│   ├── 36_PLAY_MODE.md                    ← Play + Live Reload
│   ├── 37_HOT_RELOAD.md                  ← Hot Reload Panel
│   ├── 38_ASSET_CONFIG.md                ← Import settings
│   ├── 38_PREFABS.md                      ← Plantillas
│   └── 39_EVENT_FORGE.md                  ← Grafo de eventos
│
├── .agents/                               ← Reglas para IAs
│   ├── AGENTS.md                          ← REGLAS DE ORO OBLIGATORIAS
│   └── skills/
│       └── task.md                        ← Tareas de agentes
│
└── old/                                   ← Backup antiguo
    ├── DOCUMENTACION/                     ← Backup docs
    ├── DOCUMENTATION_SYSTEM/              ← Backup sistema
    └── *.txt                              ← Archivos .txt originales
```

## 🚦 SEMÁFOROS DE CONTROL DE IA (AGENT RAILS)

### 🔴 SEMÁFORO DE INICIO (Al crear nueva sesión)
- ✅ Leer AGENTS.md (Reglas de Oro)
- ✅ Leer AI_GUIDELINES.md (Directrices detalladas)
- ✅ Leer PROGRESO.md (últimas 3 líneas del log)
- ✅ Verificar `cargo test` → 100% passing

### 🟡 SEMÁFORO DE VERIFICACIÓN (Después de cada cambio)
- ✅ `cargo check` → Sin errores
- ✅ `cargo test` → 100% passing
- ✅ Verificar anti-stubs (sin `todo!()`, `unimplemented!()`)
- ✅ Verificar anti-breaking changes (firmas públicas)

### 🟢 SEMÁFORO DE FINALIZACIÓN (Al completar tarea)
- ✅ Documentar cambios en PROGRESO.md
- ✅ Actualizar métricas en TOOLS.md
- ✅ Commit con mensaje claro
- ✅ Registrar en Historial de Sesiones

---

## 🛡️ DIRECTIVAS AGENT RAILS

### 1. 🛡️ Anti-Breaking Changes
- **PROHIBIDO** alterar firmas públicas sin compatibilidad retro
- **PROHIBIDO** eliminar parámetros/retornos sin variante nueva o `#[deprecated]`
- **PROHIBIDO** propagar refactorizaciones por 15+ archivos sin planificación

### 2. 🚫 Prohibición de Placeholders Silenciosos (Anti-Stubs)
- **PROHIBIDO** marcar como completado si contiene `todo!()` o `unimplemented!()`
- **PROHIBIDO** usar `// TODO` sin especificar: quién, cuándo, por qué
- Si es prototipo:
  - Código debe llevar: `// STUB: [Explicación]`
  - Documentación debe catalogar: "Integración Parcial" o "⏳ En Desarrollo"

### 3. ✅ Cobertura Obligatoria de Tests (TDD)
- **Ninguna función pública finalizada sin test unitario/integración**
- Validar caminos principales y de error
- 100% passing rate obligatorio

---

## 📋 GUÍA RÁPIDA

---

## 📋 GUÍA RÁPIDA

### Para crear nueva documentación:
1. **PASO 1:** Verificar en `doc/TOOLS.md` (¿está autorizada?)
2. **PASO 2:** Copiar plantilla de `old/PLANTILLA.md`
3. **PASO 3:** Guardar en `doc/tools/NN_NOMBRE.md`
4. **PASO 4:** Completar secciones 1-10
5. **PASO 5:** Actualizar `doc/TOOLS.md` y `doc/INDEX.md`
6. **PASO 6:** Commit

### Para usar como guía de implementación:
1. Leer `doc/VISION.md` (qué construir)
2. Leer `doc/ARCHITECTURE.md` (arquitectura)
3. Leer `doc/tools/NN_NOMBRE.md` (qué está hecho + qué falta)
4. Implementar y testear

### Para documentar cambios existentes:
1. **VERIFICAR:** Buscar en `doc/TOOLS.md` y `doc/tools/`
2. **ACTUALIZAR:** Editar solo lo que cambió (nunca reescribir completo)
3. **REGISTRAR:** Añadir funciones nuevas en Sección 3
4. **VERIFICAR:** `cargo check` y `cargo test`

---

## 📊 ESTADO ACTUAL

| Categoría | Total | Hecho | En progreso | Pendiente |
|-----------|-------|-------|-------------|-----------|
| **Herramientas documentadas** | 39 | 39 | 0 | 0 |
| **UIs completadas** | 38 | 38 | 0 | 0 |
| **Tests passing** | 135/135 (100%) | 135 | 0 | 0 |

---

## 🎯 PUNTOS DE ENTRADA

- **Inicio:** `doc/README.md`
- **Visión:** `doc/VISION.md`
- **Herramientas:** `doc/TOOLS.md`
- **Manual de UX:** [UX_MANUAL.md](file:///c:/Users/xico0/Desktop/Xico/doc/UX_MANUAL.md)
- **Directrices de IA:** [AI_GUIDELINES.md](file:///c:/Users/xico0/Desktop/Xico/doc/AI_GUIDELINES.md)
- **Reglas de Oro:** [`.agents/AGENTS.md`](file:///c:/Users/xico0/Desktop/Xico/.agents/AGENTS.md)
- **Patrón de Documentación:** [`.agents/AGENTS.md`](file:///c:/Users/xico0/Desktop/Xico/.agents/AGENTS.md) (sección 2)
- **Plantilla:** `old/PLANTILLA.md`

---

## 📊 RESUMEN DE FASES

### FASE 10: Timeline System - COMPLETO (51/51 tests)
- FASE 10.1: Timeline UI Editor (13/13)
- FASE 10.2: Timeline System Runtime (12/12)
- FASE 10.3: Timeline System Integration (15/15)
- FASE 10.4: Timeline Editor UI (10/10)
- FASE 10.5: Animation Clips & Library (9/9)

### FASE 11: Audio System - COMPLETO (18/18 tests)

### FASE 12: Play Mode - COMPLETO (29/29 tests)
- PlaySession con simulate_physics y WASD
- InputCapture con mouse state
- SnapshotManager con restore_snapshot

### FASE 13: Hot Reload Panel - COMPLETO (5/5 tests)
- HotReloadManager para gestionar cambios
- HotReloadPanel con UI completa
- Integration en ForgeEditorApp

### FASE 38: File Watcher + Hot Reload - COMPLETO (20/20 tests)
- FileWatcher con detección de cambios automática
- HotReloadManager con pending_changes y process
- ScriptExecutor para ejecución de scripts
- HotReloadPanel con UI completa
- Integration en ForgeEditorApp

---

## 🔄 PATRÓN DE DOCUMENTACIÓN ORGÁNICA

**Flujo de 5 pasos:**
1. **VERIFICAR** → Buscar en TOOLS.md, tools/, PROGRESO.md
2. **DECIDIR** → Crear (si no existe) o Actualizar (si existe)
3. **SEGUIR REGLAS** → Nunca duplicar, siempre actualizar TOOLS.md
4. **ACTUALIZAR** → Referencias en TOOLS.md, INDEX.md, PROGRESO.md
5. **VERIFICAR** → cargo check, cargo test, consistencia

**Niveles de documentación:**
- Nivel 1: `doc/TOOLS.md` (autorización)
- Nivel 2: `doc/tools/NN_NOMBRE.md` (documentación técnica)
- Nivel 3: `doc/PROGRESO.md` (progreso de desarrollo)

---

**Sistema de Documentación v1.1.0 - [AI: opencode]**








**L�neas de c�digo** | ~577,132 bytes |
