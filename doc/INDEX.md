# ðŸ“š DOCUMENTACIÃ“N - ESTRUCTURA FINAL

**VersiÃ³n:** 1.1.0  
**Fecha:** 2026-07-23  
**AI:** [AI: opencode]

---

## ðŸ—‚ï¸ ESTRUCTURA DE CARPETAS

```
doc/
â”œâ”€â”€ VISION.md                              â† Punto de verdad (Ãºnico)
â”œâ”€â”€ ARCHITECTURE.md                        â† Arquitectura tÃ©cnica
â”œâ”€â”€ TOOLS.md                               â† Lista de herramientas
â”œâ”€â”€ ROADMAP.md                             â† QuÃ© falta por hacer
â”œâ”€â”€ REQUIREMENTS.md                        â† Requisitos
â”œâ”€â”€ README.md                              â† Entrada principal
â”œâ”€â”€ PROGRESO.md                            â† Registro de Progreso y Handoff
â”œâ”€â”€ AI_GUIDELINES.md                       â† Directrices para IA
â”œâ”€â”€ UX_MANUAL.md                           â† Manual de diseÃ±o UX/UI
â”‚
â”œâ”€â”€ tools/                                 â† DocumentaciÃ³n de herramientas
â”‚   â”œâ”€â”€ 01_SCENE_EDITOR.md                 â† Editor de escena
â”‚   â”œâ”€â”€ 01_LIVE_SYNC.md                    â† SincronizaciÃ³n
â”‚   â”œâ”€â”€ Uis.md                             â† UIs (Transform, Component, Property, Plugin)
â”‚   â”œâ”€â”€ 03_TIMELINE_EDITOR.md              â† Editor de timeline
â”‚   â”œâ”€â”€ 04_KEYFRAME_SYSTEM.md              â† Sistema de keyframes
â”‚   â”œâ”€â”€ 05_ANIMATION_2D.md                 â† ReproducciÃ³n de clips
â”‚   â”œâ”€â”€ 05_ANIMATION_TRACK.md              â† Track data serialization
â”‚   â”œâ”€â”€ 06_ANIMATION_TRACK.md              â† Track UI
â”‚   â”œâ”€â”€ 06_CABLE_SYSTEM_UI.md              â† UI de cables
â”‚   â”œâ”€â”€ 10_SCRIPT_EDITOR.md                â† Editor de scripts
â”‚   â”œâ”€â”€ 11_SCRIPT_EXECUTOR.md              â† Hot-reload de scripts
â”‚   â”œâ”€â”€ 12_COMPILE_SYSTEM.md               â† AST parsing
â”‚   â”œâ”€â”€ 13_SCRIPT_OPTIMIZER.md             â† Dead code elimination
â”‚   â”œâ”€â”€ 14_SCRIPT_VIEWER.md                â† Diff de versiones
â”‚   â”œâ”€â”€ 15_DEBUG_PANEL.md                  â† Console + variables
â”‚   â”œâ”€â”€ 16_HOT_RELOAD.md                   â† Hot-reload assets
â”‚   â”œâ”€â”€ 17_LINTER_PANEL.md                 â† Reglas configurables
â”‚   â”œâ”€â”€ 20_PARTICLE_SYSTEM.md              â† Efectos visuales
â”‚   â”œâ”€â”€ 22_PLUGIN_SYSTEM.md                â† Plugin loading
â”‚   â”œâ”€â”€ 23_COLLABORATION.md                â† Multiplayer editing
â”‚   â”œâ”€â”€ 24_BITACORA_MANAGER.md             â† Logging
â”‚   â”œâ”€â”€ 25_EXPORT_MANAGER.md               â† Export a GML/JSON
â”‚   â”œâ”€â”€ 26_IMPORT_MANAGER.md               â† Import sprites/audio
â”‚   â”œâ”€â”€ 27_MAP_EXPORT.md                   â† Tilemap export
â”‚   â”œâ”€â”€ 28_SERIALIZATION_PANEL.md          â† JSON export/import
â”‚   â”œâ”€â”€ 30_3D_SPRITE_BAKER.md              â† Atlas generation
â”‚   â”œâ”€â”€ 31_SPRITE_SLICER.md                â† Editor de atlas
â”‚   â”œâ”€â”€ 32_TILEMAP_PAINTER.md              â† Pincel de mapas
â”‚   â”œâ”€â”€ 33_PHYSICS_INSPECTOR.md            â† FÃ­sicas + Gizmos
â”‚   â”œâ”€â”€ 34_CINEGRAPH_DIALOGUE.md           â† DiÃ¡logos
â”‚   â”œâ”€â”€ 35_SOUND_SOCKETS.md                â† Audio 3D
â”‚   â”œâ”€â”€ 36_PLAY_MODE.md                    â† Play + Live Reload
â”‚   â”œâ”€â”€ 37_ASSET_CONFIG.md                 â† Import settings
â”‚   â”œâ”€â”€ 38_PREFABS.md                      â† Plantillas
â”‚   â””â”€â”€ 39_EVENT_FORGE.md                  â† Grafo de eventos
â”‚
â”œâ”€â”€ .agents/                               â† Reglas para IAs
â”‚   â”œâ”€â”€ AGENTS.md                          â† REGLAS DE ORO OBLIGATORIAS
â”‚   â””â”€â”€ skills/
â”‚       â””â”€â”€ task.md                        â† Tareas de agentes
â”‚
â””â”€â”€ old/                                   â† Backup antiguo
    â”œâ”€â”€ DOCUMENTACION/                     â† Backup docs
    â”œâ”€â”€ DOCUMENTATION_SYSTEM/              â† Backup sistema
    â””â”€â”€ *.txt                              â† Archivos .txt originales
```

## ðŸš¦ SEMÃFOROS DE CONTROL DE IA (AGENT RAILS)

### ðŸ”´ SEMÃFORO DE INICIO (Al crear nueva sesiÃ³n)
- âœ… Leer AGENTS.md (Reglas de Oro)
- âœ… Leer AI_GUIDELINES.md (Directrices detalladas)
- âœ… Leer PROGRESO.md (Ãºltimas 3 lÃ­neas del log)
- âœ… Verificar `cargo test` â†’ 100% passing

### ðŸŸ¡ SEMÃFORO DE VERIFICACIÃ“N (DespuÃ©s de cada cambio)
- âœ… `cargo check` â†’ Sin errores
- âœ… `cargo test` â†’ 100% passing
- âœ… Verificar anti-stubs (sin `todo!()`, `unimplemented!()`)
- âœ… Verificar anti-breaking changes (firmas pÃºblicas)

### ðŸŸ¢ SEMÃFORO DE FINALIZACIÃ“N (Al completar tarea)
- âœ… Documentar cambios en PROGRESO.md
- âœ… Actualizar mÃ©tricas en TOOLS.md
- âœ… Commit con mensaje claro
- âœ… Registrar en Historial de Sesiones

---

## ðŸ›¡ï¸ DIRECTIVAS AGENT RAILS

### 1. ðŸ›¡ï¸ Anti-Breaking Changes
- **PROHIBIDO** alterar firmas pÃºblicas sin compatibilidad retro
- **PROHIBIDO** eliminar parÃ¡metros/retornos sin variante nueva o `#[deprecated]`
- **PROHIBIDO** propagar refactorizaciones por 15+ archivos sin planificaciÃ³n

### 2. ðŸš« ProhibiciÃ³n de Placeholders Silenciosos (Anti-Stubs)
- **PROHIBIDO** marcar como completado si contiene `todo!()` o `unimplemented!()`
- **PROHIBIDO** usar `// TODO` sin especificar: quiÃ©n, cuÃ¡ndo, por quÃ©
- Si es prototipo:
  - CÃ³digo debe llevar: `// STUB: [ExplicaciÃ³n]`
  - DocumentaciÃ³n debe catalogar: "IntegraciÃ³n Parcial" o "â³ En Desarrollo"

### 3. âœ… Cobertura Obligatoria de Tests (TDD)
- **Ninguna funciÃ³n pÃºblica finalizada sin test unitario/integraciÃ³n**
- Validar caminos principales y de error
- 100% passing rate obligatorio

---

## ðŸ“‹ GUÃA RÃPIDA

---

## ðŸ“‹ GUÃA RÃPIDA

### Para crear nueva documentaciÃ³n:
1. **PASO 1:** Verificar en `doc/TOOLS.md` (Â¿estÃ¡ autorizada?)
2. **PASO 2:** Copiar plantilla de `old/PLANTILLA.md`
3. **PASO 3:** Guardar en `doc/tools/NN_NOMBRE.md`
4. **PASO 4:** Completar secciones 1-10
5. **PASO 5:** Actualizar `doc/TOOLS.md` y `doc/INDEX.md`
6. **PASO 6:** Commit

### Para usar como guÃ­a de implementaciÃ³n:
1. Leer `doc/VISION.md` (quÃ© construir)
2. Leer `doc/ARCHITECTURE.md` (arquitectura)
3. Leer `doc/tools/NN_NOMBRE.md` (quÃ© estÃ¡ hecho + quÃ© falta)
4. Implementar y testear

### Para documentar cambios existentes:
1. **VERIFICAR:** Buscar en `doc/TOOLS.md` y `doc/tools/`
2. **ACTUALIZAR:** Editar solo lo que cambiÃ³ (nunca reescribir completo)
3. **REGISTRAR:** AÃ±adir funciones nuevas en SecciÃ³n 3
4. **VERIFICAR:** `cargo check` y `cargo test`

---

## ðŸ“Š ESTADO ACTUAL

| CategorÃ­a | Total | Hecho | En progreso | Pendiente |
|-----------|-------|-------|-------------|-----------|
| **Herramientas documentadas** | 38 | 38 | 0 | 0 |
| **UIs completadas** | 37 | 37 | 0 | 0 |
| **Tests passing | 42/42 (100%) |

---

## ðŸŽ¯ PUNTOS DE ENTRADA

- **Inicio:** `doc/README.md`
- **VisiÃ³n:** `doc/VISION.md`
- **Herramientas:** `doc/TOOLS.md`
- **Manual de UX:** [UX_MANUAL.md](file:///c:/Users/xico0/Desktop/Xico/doc/UX_MANUAL.md)
- **Directrices de IA:** [AI_GUIDELINES.md](file:///c:/Users/xico0/Desktop/Xico/doc/AI_GUIDELINES.md)
- **Reglas de Oro:** [`.agents/AGENTS.md`](file:///c:/Users/xico0/Desktop/Xico/.agents/AGENTS.md)
- **PatrÃ³n de DocumentaciÃ³n:** [`.agents/AGENTS.md`](file:///c:/Users/xico0/Desktop/Xico/.agents/AGENTS.md) (secciÃ³n 2)
- **Plantilla:** `old/PLANTILLA.md`

---

## ðŸ”„ PATRÃ“N DE DOCUMENTACIÃ“N ORGÃNICA

**Flujo de 5 pasos:**
1. **VERIFICAR** â†’ Buscar en TOOLS.md, tools/, PROGRESO.md
2. **DECIDIR** â†’ Crear (si no existe) o Actualizar (si existe)
3. **SEGUIR REGLAS** â†’ Nunca duplicar, siempre actualizar TOOLS.md
4. **ACTUALIZAR** â†’ Referencias en TOOLS.md, INDEX.md, PROGRESO.md
5. **VERIFICAR** â†’ cargo check, cargo test, consistencia

**Niveles de documentaciÃ³n:**
- Nivel 1: `doc/TOOLS.md` (autorizaciÃ³n)
- Nivel 2: `doc/tools/NN_NOMBRE.md` (documentaciÃ³n tÃ©cnica)
- Nivel 3: `doc/PROGRESO.md` (progreso de desarrollo)

---

**Sistema de DocumentaciÃ³n v1.1.0 - [AI: opencode]**








**Líneas de código** | ~577,132 bytes |
