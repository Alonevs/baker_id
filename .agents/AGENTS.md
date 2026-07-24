# 🤖 REGLAS DE WORKSPACE PARA AGENTES E IA

Este archivo define las directivas automáticas que todo agente de Inteligencia Artificial que se ejecute en este espacio de trabajo debe leer y acatar de forma transparente.

---

## 🚨 REGLAS DE ORO OBLIGATORIAS

### 🛡️ SEMÁFOROS DE CONTROL DE IA (AGENT RAILS)

**🔴 SEMÁFORO DE INICIO (Al crear nueva sesión):**
- Leer AGENTS.md (Reglas de Oro)
- Leer AI_GUIDELINES.md (Directrices detalladas)
- Leer PROGRESO.md (últimas 3 líneas del log)
- Verificar estado actual con `cargo test`

**🟡 SEMÁFORO DE VERIFICACIÓN (Después de cada cambio):**
- `cargo check` → Sin errores
- `cargo test` → 100% passing
- Verificar anti-stubs en código modificado
- Verificar compatibilidad de firmas públicas

**🟢 SEMÁFORO DE FINALIZACIÓN (Al completar tarea):**
- Documentar cambios en PROGRESO.md
- Actualizar métricas en TOOLS.md
- Commit con mensaje claro
- Registrar en Historial de Sesiones

---

## 🚨 REGLAS DE ORO OBLIGATORIAS

1. **Consulta de Pautas de IA:**
    - Antes de proponer modificaciones de código, refactorizaciones o actualizaciones, debes leer obligatoriamente el archivo [doc/AI_GUIDELINES.md](file:///c:/Users/xico0/Desktop/Xico/doc/AI_GUIDELINES.md).
   
2. **Prevención de Duplicidad de Archivos:**
   - Queda estrictamente prohibido crear nuevos archivos en `doc/tools/` que no estén explícitamente autorizados en [doc/TOOLS.md](file:///c:/Users/xico0/Desktop/Xico/doc/TOOLS.md).
   - Queda estrictamente prohibido crear archivos `.md` independientes para tests unitarios, componentes de UI secundarios o módulos auxiliares. Todo test nuevo debe documentarse en [doc/tools/TESTS.md](file:///c:/Users/xico0/Desktop/Xico/doc/tools/TESTS.md). Toda UI nueva debe documentarse en [doc/tools/Uis.md](file:///c:/Users/xico0/Desktop/Xico/doc/tools/Uis.md). Las utilidades menores deben catalogarse en la Sección 3 del documento de la herramienta principal.
   - Queda prohibido crear archivos de tareas/listas sueltos (como `todo.md` o similares) en la raíz del workspace. Todo plan de tareas o pendientes debe documentarse exclusivamente en [doc/PROGRESO.md](file:///c:/Users/xico0/Desktop/Xico/doc/PROGRESO.md) en la sección de próximos objetivos.
   - Los cambios de lógica en funciones del sistema, integraciones entre herramientas, sincronizaciones y flujos de consulta cruzada deben registrarse actualizando los archivos `.md` existentes de las respectivas herramientas afectadas, nunca creando nuevos documentos de integración.
   - Queda prohibido que los archivos `.md` superen de forma innecesaria las 1,500-3,000 líneas (para evitar cortar definiciones de funciones o arquitecturas). Si crece por encima de este límite, se debe dividir en partes lógicas (ej: `_PARTE2.md` o `_HISTORIAL.md`). Cada vez que se cree o edite un archivo `.md` de tamaño considerable, la IA debe generar o actualizar obligatoriamente una cabecera con un índice o tabla de contenidos al inicio del archivo si no cuenta con ella, facilitando lecturas parciales quirúrgicas de la IA sin consumir contexto innecesario. Si un cambio aplica a una sección preexistente, se debe modificar quirúrgicamente en la parte original, no añadiendo notas redundantes en las expansiones.
   - Cualquier cambio en la documentación de herramientas debe realizarse mediante ediciones quirúrgicas parciales (`replace_file_content`), nunca reescribiendo documentos completos.

3. **Registro de Funciones Granular:**
   - Cada nueva función o estructura pública de Rust añadida en los crates debe ser catalogada en la Sección 3 (Catálogo de Funciones Detalladas) de la documentación de la herramienta afectada.

4. **Tolerancia a Cambios de Rutas (Refactorizaciones):**
   - Las rutas descritas en los manuales son instantáneas temporales. Si el usuario mueve archivos de carpetas, debes usar búsquedas recursivas (`grep_search` / `list_dir`) en el workspace para localizar los componentes en su nueva ubicación, en lugar de duplicar código o asumir que no existen.

5. **Consistencia del Estilo de Código Existente:**
    - Debes respetar y mantener de forma prioritaria la estructura, el formato, el estilo y los patrones del código preexistente en cada archivo.
    - Queda prohibido reformatear archivos completos o reescribir lógica funcional con otros estilos (por ejemplo, cambiar flujos de control, estructuras de préstamos o control de errores) a menos que se solicite de forma explícita. El código nuevo debe integrarse y mimetizarse de manera invisible con el código del entorno.

6. **🛡️ Garantía Anti-Breaking Changes (Compatibilidad Retro):**
    - **PROHIBIDO** alterar firmas de funciones públicas compartidas sin asegurar compatibilidad hacia atrás.
    - **PROHIBIDO** eliminar parámetros, retornos o cambiar tipos sin crear variante nueva o marcar como `#[deprecated]`.
    - **PROHIBIDO** propagar refactorizaciones por 15+ archivos sin comunicación y planificación previa.
    - Si un cambio es necesario:
      1. Marcar como `#[deprecated]` con mensaje de migración
      2. Crear variante nueva con prefijo `_v2` o similar
      3. Actualizar documentación en TOOLS.md y AI_GUIDELINES.md
      4. Asegurar que tests existentes sigan passing

7. **🚫 Prohibición de Placeholders Silenciosos (Anti-Stubs):**
    - **PROHIBIDO** marcar una herramienta como completada si contiene `todo!()`, `unimplemented!()` o marcadores de posición en el código de producción.
    - **PROHIBIDO** crear archivos `.md` documentando una función como "✅ Completado" si el código tiene lógica stubbed.
    - Si algo es un prototipo:
      1. Código debe llevar comentario: `// STUB: [Explicación]`
      2. Documentación debe catalogarlo bajo "Integración Parcial" o "⏳ En Desarrollo"
      3. No propagar este estado a otras herramientas dependientes
    - **PROHIBIDO** usar `// TODO` sin especificar: quién, cuándo y por qué

8. **Actualización Automática de Progreso (Handoff de Agentes):**
   - Al finalizar cualquier tarea, sesión o cambio en el código, debes actualizar obligatoriamente el archivo [doc/PROGRESO.md](file:///c:/Users/xico0/Desktop/Xico/doc/PROGRESO.md).
   - Registra de forma detallada los cambios realizados, actualiza las métricas de tests totales activos en verde (ej: 96 tests totales) y redefine con precisión el estado y tareas pendientes en la sección `## 🎯 PRÓXIMOS OBJETIVOS` para que el siguiente agente retome el trabajo sin fricciones y sin necesidad de preguntar.

---

## 📝 PATRÓN DE DOCUMENTACIÓN ORGÁNICA (5 PASOS)

### PASO 1: VERIFICAR SI EXISTE (CHECK FIRST)
Antes de cualquier acción de documentación, busca en este orden:
1. **`doc/TOOLS.md`** → ¿La herramienta está autorizada en la lista?
2. **`doc/tools/NN_NOMBRE.md`** → ¿El archivo ya existe?
3. **`doc/PROGRESO.md`** → ¿El progreso ya está documentado?

### PASO 2: DECIDIR ACCIÓN

| Situación | Acción |
|-----------|--------|
| ✅ Archivo existe | ACTUALIZAR (edición quirúrgica, nunca reescribir completo) |
| ❌ No existe y autorizada | CREAR en `doc/tools/NN_NOMBRE.md` siguiendo plantilla |
| 📊 Solo progreso de código | ACTUALIZAR `doc/PROGRESO.md` con métricas y estado |

### PASO 3: SEGUIR REGLAS OBLIGATORIAS
- **NUNCA** crear duplicados (si existe, actualizar)
- **SIEMPRE** actualizar `doc/TOOLS.md` al crear nueva herramienta
- **SIEMPRE** registrar funciones en Sección 3 del .md correspondiente
- **NUNCA** reescribir archivos completos (usar `edit`/`replace_file_content`)
- **SIEMPRE** verificar consistencia con `doc/VISION.md`

### PASO 4: ACTUALIZAR REFERENCIAS
Al crear/actualizar documentación, actualiza:
- **`doc/TOOLS.md`** → Añadir/actualizar herramienta en lista
- **`doc/INDEX.md`** → Actualizar lista en `doc/tools/`
- **`doc/PROGRESO.md`** → Actualizar métricas si aplica

### PASO 5: VERIFICAR
Antes de finalizar:
- `cargo check` → Sin errores
- `cargo test` → 100% passing
- Consistencia con VISION.md y TOOLS.md
- No hay archivos duplicados en `doc/tools/`

---

## 🗂️ NIVELES DE DOCUMENTACIÓN

| Nivel | Ubicación | Uso | Regla |
|-------|-----------|-----|-------|
| **Nivel 1** | `doc/TOOLS.md` | Autorización de herramientas | SI NO ESTÁ AQUÍ → NO CREAR |
| **Nivel 2** | `doc/tools/NN_NOMBRE.md` | Documentación técnica completa (10 secciones) | SI EXISTE → ACTUALIZAR |
| **Nivel 3** | `doc/PROGRESO.md` | Progreso de desarrollo, métricas, tests | Solo cambios de progreso |

**Flujo correcto:** Nivel 1 (autorizar) → Nivel 2 (documentar) → Nivel 3 (progreso)

