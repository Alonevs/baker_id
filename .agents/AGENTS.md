# 🤖 REGLAS DE WORKSPACE PARA AGENTES E IA

Este archivo define las directivas automáticas que todo agente de Inteligencia Artificial que se ejecute en este espacio de trabajo debe leer y acatar de forma transparente.

---

## 🚨 REGLAS DE ORO OBLIGATORIAS

1. **Consulta de Pautas de IA:**
   - Antes de proponer modificaciones de código, refactorizaciones o actualizaciones, debes leer obligatoriamente el archivo [doc/AI_GUIDELINES.md](file:///c:/Users/xico0/Desktop/Xico/doc/AI_GUIDELINES.md).
   
2. **Prevención de Duplicidad de Archivos:**
   - Queda estrictamente prohibido crear nuevos archivos en `doc/tools/` que no estén explícitamente autorizados en [doc/TOOLS.md](file:///c:/Users/xico0/Desktop/Xico/doc/TOOLS.md).
   - Cualquier cambio en la documentación de herramientas debe realizarse mediante ediciones quirúrgicas parciales (`replace_file_content`), nunca reescribiendo documentos completos.

3. **Registro de Funciones Granular:**
   - Cada nueva función o estructura pública de Rust añadida en los crates debe ser catalogada en la Sección 3 (Catálogo de Funciones Detalladas) de la documentación de la herramienta afectada.

4. **Tolerancia a Cambios de Rutas (Refactorizaciones):**
   - Las rutas descritas en los manuales son instantáneas temporales. Si el usuario mueve archivos de carpetas, debes usar búsquedas recursivas (`grep_search` / `list_dir`) en el workspace para localizar los componentes en su nueva ubicación, en lugar de duplicar código o asumir que no existen.
