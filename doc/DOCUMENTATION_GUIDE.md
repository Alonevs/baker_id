# 📚 GUÍA DE USO - SISTEMA DE DOCUMENTACIÓN FORGE SDK

**Versión:** 1.0.0  
**Última actualización:** 2026-07-23  
**AI Responsable:** [AI: opencode]

---

## 🎯 OBJETIVO

Esta guía explica cómo usar el sistema de documentación del Forge SDK para:
1. Crear nueva documentación
2. Actualizar documentación existente
3. Validar que sigue el formato correcto
4. Entender qué falta por implementar

---

## 📁 ESTRUCTURA DEL SISTEMA

```
doc/
├── README.md                    ← Punto de entrada
├── TOOLS.md                     ← Lista de 29 herramientas
├── ROADMAP.md                   ← Qué falta por hacer
├── ARCHITECTURE.md              ← Arquitectura del proyecto
├── VISION.md                    ← Visión única (single source of truth)
├── REQUIREMENTS.md              ← Requisitos
│
└── tools/
    ├── 00_TEMPLATE.md           ← Plantilla MAESTRA
    ├── 01_LIVE_SYNC.md          ← Sistema de sincronización
    ├── 02_PROJECT_MANAGER.md    ← Gestor de proyectos
    ├── 03_ASSET_BROWSER.md      ← Explorador de assets
    ├── 04_VIEWPORT.md           ← Viewport 2D
    ├── 05_ANIMATION_2D.md       ← Animación 2D
    ├── 06_ANIMATION_TRACK.md    ← Animation Track
    ├── 03_TIMELINE_EDITOR.md    ← Timeline Editor
    ├── 04_KEYFRAME_SYSTEM.md    ← Keyframe System
    ├── 10_SCRIPT_EDITOR.md      ← Script Editor
    ├── 11_SCRIPT_EXECUTOR.md    ← Script Executor
    ├── 12_COMPILE_SYSTEM.md     ← Compile System
    ├── 13_SCRIPT_OPTIMIZER.md   ← Script Optimizer
    ├── 14_SCRIPT_VIEWER.md      ← Script Viewer
    ├── 15_DEBUG_PANEL.md        ← Debug Panel
    ├── 16_HOT_RELOAD.md         ← Hot Reload
    ├── 17_LINTER_PANEL.md       ← Linter Panel
    ├── 20_PARTICLE_SYSTEM.md    ← Particle System
    ├── 22_PLUGIN_SYSTEM.md      ← Plugin System
    ├── 23_COLLABORATION.md      ← Collaboration
    ├── 24_BITACORA_MANAGER.md   ← Bitacora Manager
    ├── 25_EXPORT_MANAGER.md     ← Export Manager
    ├── 26_IMPORT_MANAGER.md     ← Import Manager
    ├── 27_MAP_EXPORT.md         ← Map Export
    ├── 28_SERIALIZATION_PANEL.md← Serialization Panel
    ├── 30_3D_SPRITE_BAKER.md    ← 3D Sprite Baker
    ├── 33_PHYSICS_INSPECTOR.md  ← Physics Inspector
    ├── 34_CINEGRAPH_DIALOGUE.md ← CineGraph & Dialogue
    ├── 35_SOUND_SOCKETS.md      ← Sound Sockets
    ├── 36_PLAY_MODE.md          ← Play Mode
    ├── 37_ASSET_CONFIG.md       ← Asset Config
    ├── 38_PREFABS.md            ← Prefabs
    ├── 39_EVENT_FORGE.md        ← Event Forge
    └── TESTS.md                 ← Tests de integración
```

---

## 📋 FORMATO OBLIGATORIO (10 SECCIONES)

Cada documento debe seguir EXACTAMENTE este orden:

### Sección 1: ESPECIFICACIONES (Obligatorio)
- 1.1 Qué debe hacer
- 1.2 Problemas que resuelve
- 1.3 Usuarios objetivo
- 1.4 Requisitos de entrada
- 1.5 Requisitos de salida

### Sección 2: ARQUITECTURA (Obligatorio)
- 2.1 Diagrama de flujo
- 2.2 Componentes principales
- 2.3 Flujo de datos
- 2.4 Dependencias
- 2.5 API pública

### Sección 3: IMPLEMENTACIÓN ACTUAL (Obligatorio)
- 3.1 Código implementado
- 3.2 Archivos creados
- 3.3 Funcionalidades implementadas
- 3.4 Funcionalidades pendientes

### Sección 4: TESTS (Obligatorio - 100% passing)
- 4.1 Test Unitario
- 4.2 Test de Integración
- 4.3 Test de Validación
- 4.4 Estado de tests

### Sección 5: USO (Obligatorio)
- 5.1 Ejemplo básico
- 5.2 Ejemplo avanzado

### Sección 6: MÉTRICAS (Obligatorio)
- Líneas de código
- Funciones públicas
- Tests passing
- Coverage
- Build time
- Memory usage

### Sección 7: PROBLEMAS CONOCIDOS (Obligatorio)
- ID, Problema, Impacto, Prioridad, Estado

### Sección 8: ROADMAP (Obligatorio)
- Fase 1: MVP
- Fase 2: Mejoras
- Fase 3: Avanzado

### Sección 9: NOTAS Y DECISIONES (Obligatorio)
- Decisiones de diseño
- Limitaciones conocidas

### Sección 10: RELACIONES (Obligatorio)
- Herramientas relacionadas
- Referencias externas

---

## 🚀 CREAR NUEVA DOCUMENTACIÓN

### Paso 1: Copiar plantilla
```bash
# Windows PowerShell
Copy-Item "doc\tools\01_TEMPLATE.md" "doc\tools\NN_NOMBRE.md"

# O usar PowerShell
cp doc\tools\01_TEMPLATE.md doc\tools\NN_NOMBRE.md
```

### Paso 2: Completar secciones obligatorias
- [ ] Sección 1: Especificaciones
- [ ] Sección 2: Arquitectura
- [ ] Sección 3: Implementación actual
- [ ] Sección 4: Tests (DEBE SER 100%)
- [ ] Sección 5: Uso
- [ ] Sección 6: Métricas
- [ ] Sección 7: Problemas conocidos
- [ ] Sección 8: Roadmap
- [ ] Sección 9: Notas y decisiones
- [ ] Sección 10: Relaciones

### Paso 3: Verificar formato
```bash
# Verificar que sigue el formato exacto
grep -n "^# " doc\tools\NN_NOMBRE.md
```

### Paso 4: Commit
```bash
git add .
git commit -m "Add complete documentation for NN_NOMBRE"
```

---

## 🔧 ACTUALIZAR DOCUMENTACIÓN EXISTENTE

### Cambiar Estado
- ✅ Funcional → Si la herramienta funciona
- 🔄 En desarrollo → Si está en progreso
- ❌ Pendiente → Si no existe

### Actualizar Métricas
```markdown
| Métrica | Valor Actual | Objetivo | Estado |
|---------|--------------|----------|--------|
| Líneas de código | ~1800 | < 2000 | ✅ |
| Tests passing | 13/13 | 100% | ✅ |
```

### Actualizar Roadmap
```markdown
### 8.1 Fase 1: MVP (Ya implementado ✅)
- [x] Feature A
- [x] Feature B
```

### Actualizar Tests
```markdown
| Test Suite | Passing | Total | Rate |
|------------|---------|-------|------|
| Unit Tests | 8/8 | 100% |
| Integration | 5/5 | 100% |
| **TOTAL** | **13/13** | **100%** |
```

---

## ✅ CHECKLIST DE VALIDACIÓN

### Secciones obligatorias
- [ ] Sección 1: Especificaciones
- [ ] Sección 2: Arquitectura
- [ ] Sección 3: Implementación actual
- [ ] Sección 4: Tests (100% passing)
- [ ] Sección 5: Uso
- [ ] Sección 6: Métricas
- [ ] Sección 7: Problemas conocidos
- [ ] Sección 8: Roadmap
- [ ] Sección 9: Notas y decisiones
- [ ] Sección 10: Relaciones

### Formato
- [ ] Fecha YYYY-MM-DD actual
- [ ] AI responsable marcada
- [ ] Estado correcto (✅/🔄/❌)
- [ ] Prioridad marcada
- [ ] Versión X.X.X

### Contenido
- [ ] Diagrama de flujo (sección 2.1)
- [ ] API pública documentada (sección 2.5)
- [ ] Ejemplos de uso (sección 5)
- [ ] Tests unitarios (sección 4.1)
- [ ] Tests de integración (sección 4.2)
- [ ] Roadmap completo (sección 8)

---

## 🎯 TRACKING PARA AI

### Para cada herramienta, la AI debe saber:

1. **Qué se espera** (Sección 1)
2. **Cómo funciona** (Sección 2)
3. **Qué está hecho** (Sección 3)
4. **Qué falta** (Sección 3.4 + Sección 8.2/8.3/8.4)
5. **Cómo validar** (Sección 4)
6. **Cómo usar** (Sección 5)

### Flujo de trabajo para AI:

```
1. Leer Sección 1 → Entender qué construir
2. Leer Sección 2 → Entender arquitectura
3. Leer Sección 3 → Ver qué ya existe
4. Leer Sección 3.4 → Saber qué falta
5. Implementar siguiendo Sección 2
6. Crear tests para Sección 4
7. Validar con Sección 6 (métricas)
8. Documentar problemas en Sección 7
9. Actualizar roadmap en Sección 8
10. Actualizar fecha y AI responsable
```

---

## 📊 ESTADO DEL SISTEMA

| Métrica | Valor |
|---------|-------|
| Plantillas creadas | 1 |
| Guías creadas | 1 |
| Herramientas documentadas | 29/29 (100%) |
| Sistema completo | ✅ Funcional |
| Formato estandarizado | ✅ Sí |

---

## 🔗 REFERENCIAS

- **`doc/README.md`** - Punto de entrada
- **`doc/TOOLS.md`** - Lista de herramientas
- **`doc/VISION.md`** - Visión única
- **`doc/ARCHITECTURE.md`** - Arquitectura
- **`doc/ROADMAP.md`** - Qué falta
- **`doc/REQUIREMENTS.md`** - Requisitos

---

**Sistema de Documentación v1.0.0 - [AI: opencode]**
