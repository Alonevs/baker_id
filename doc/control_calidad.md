Para esta etapa de desarrollo puro, lo único que realmente necesitas programar y tener bajo control en tu control_calidad.md se reduce a los cimientos técnicos de Rust:

🛡️ CONTROL DE CALIDAD (Fase de Desarrollo Interno)
🧪 1. Pruebas Unitarias del Motor (cargo test)
[ ] Cálculos de Física y Colisiones (forge-physics):

Asegurar mediante tests en Rust que las detecciones AABB y circulares devuelven los booleanos y vectores de respuesta correctos.

[ ] Jerarquía y Transformaciones (forge-scene):

Comprobar que los cálculos de posición y herencia de nodos padres a hijos no acumulen errores de redondeo o desbordamientos.

[ ] Sistemas de Animación y Tiempos (forge-animation):

Verificar que la interpolación de keyframes (Linear, EaseIn, etc.) funcione matemáticamente bien dentro del rango de 0.0 a 1.0 del timeline.

📂 2. Pruebas de Integración y Persistencia Local
[ ] Serialización de Escenas y Proyectos:

Escribir pruebas unitarias que serialicen una estructura a JSON/TOML y la deserialicen de vuelta, comprobando que ningún puntero, ID o componente se pierda en el proceso.

[ ] Validador de Assets Internos:

Asegurar que el sistema rechaza correctamente los archivos malformados (como texturas que superen la paleta permitida o metadatos corruptos) arrojando errores controlados en consola en lugar de hacer panics abruptos.