# 📋 PLANTILLA MAESTRA - DOCUMENTACIÓN FORGE SDK

**Versión:** 1.0.0  
**Última actualización:** 2026-07-23  
**AI Responsable:** [AI: opencode]

---

## 🎯 OBJETIVO

Esta plantilla es el **formato obligatorio** para documentar cada herramienta del Forge SDK. Sigue estas 10 secciones EXACTAMENTE.

---

# [NOMBRE HERRAMIENTA]

**Estado:** 🔄 Integración Parcial / 🔄 En desarrollo / ❌ Pendiente  
**Prioridad:** 🔴 Alta / 🟡 Media / 🟢 Baja  
**Versión:** X.X.X  
**Fecha creación:** YYYY-MM-DD  
**AI responsable:** [AI: nombre]  
**Última actualización:** YYYY-MM-DD  

---

## 🎯 1. ESPECIFICACIONES

### 1.1 Qué debe hacer esta herramienta
[Descripción detallada de la funcionalidad esperada]

### 1.2 Problemas que resuelve
- [Problema 1]
- [Problema 2]
- [Problema 3]

### 1.3 Usuarios objetivo
- [Quién usa esta herramienta]
- [Quién se beneficia]

### 1.4 Requisitos de entrada
- [Requisito 1]
- [Requisito 2]
- [Requisito 3]

### 1.5 Requisitos de salida
- [Salida 1]
- [Salida 2]
- [Salida 3]

---

## 🏗️ 2. ARQUITECTURA

### 2.1 Diagrama de flujo
```
┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│   Input     │───▶│  Process    │───▶│   Output    │
└─────────────┘    └─────────────┘    └─────────────┘
```

### 2.2 Componentes principales
| Componente | Responsabilidad | Archivo |
|------------|-----------------|---------|
| Componente 1 | Descripción | archivo.rs |
| Componente 2 | Descripción | archivo.rs |

### 2.3 Flujo de datos
1. Input entra por X
2. Se procesa en Y
3. Output sale por Z

### 2.4 Dependencias
- **Depende de:** [Lista de dependencias]
- **Usado por:** [Lista de dependencias]

### 2.5 Interfaz pública (API)
```rust
pub struct Nombre {
    // Campos públicos
}

impl Nombre {
    pub fn new() -> Self { ... }
    pub fn function(&self, param: T) -> Result<U, E> { ... }
}
```

---

## 💻 3. IMPLEMENTACIÓN ACTUAL

### 3.1 Código implementado
```rust
pub struct Nombre {
    pub campo1: T,
    pub campo2: U,
}

impl Nombre {
    pub fn new() -> Self {
        Self { ... }
    }
}
```

### 3.2 Archivos creados
| Archivo | Líneas | Estado |
|---------|--------|--------|
 | archivo1.rs | XXX | ⏳ Pendiente de Integración | 
| archivo2.rs | XXX | 🔄 En progreso |

### 3.3 Funcionalidades implementadas
- [x] Feature A - Descripción
- [x] Feature B - Descripción
- [x] Feature C - Descripción

### 3.4 Funcionalidades pendientes
- [ ] Feature X - Descripción
- [ ] Feature Y - Descripción
- [ ] Feature Z - Descripción

---

## 🧪 4. TESTS

### 4.1 Test Unitario
```rust
#[test]
fn test_feature_a() {
    // Test de feature A
    assert!(true);
}
```

### 4.2 Test de Integración
```rust
#[test]
fn test_integration_with_module() {
    // Test de integración con otros módulos
}
```

### 4.3 Test de Validación
```rust
#[test]
fn test_validation() {
    // Test de validación de input/output
}
```

### 4.4 Estado de tests
| Test Suite | Passing | Total | Rate |
|------------|---------|-------|------|
| Unit Tests | XX/XX | 100% |
| Integration | XX/XX | 100% |
| Validation | XX/XX | 100% |
| **TOTAL** | **XX/XX** | **100%** |

---

## 🚀 5. USO

### 5.1 Ejemplo de uso básico
```rust
let tool = Nombre::new();
let result = tool.function(parametro);
```

### 5.2 Ejemplo de uso avanzado
```rust
let tool = Nombre::new();
// Configuración avanzada
// Uso en producción
```

---

## 📊 6. MÉTRICAS

| Métrica | Valor | Objetivo |
|---------|-------|----------|
| Líneas de código | XXX | < 1000 |
| Funciones públicas | XX | < 50 |
| Tests passing | XX/XX | 100% |
| Coverage | XX% | > 90% |
| Build time | XXs | < 5s |
| Memory usage | XXMB | < 50MB |

---

## 🐛 7. PROBLEMAS CONOCIDOS

| ID | Problema | Impacto | Prioridad | Estado |
|----|----------|---------|-----------|--------|
| BUG-001 | Descripción | Alto | 🔴 | 🔄 Fix en progreso |
| BUG-002 | Descripción | Medio | 🟡 | ⏳ Pendiente |

---

## 🔮 8. ROADMAP

### 8.1 Fase 1: MVP (Ya implementado)
- [x] Feature A
- [x] Feature B
- [x] Feature C

### 8.2 Fase 2: Mejoras (En progreso)
- [ ] Feature X
- [ ] Feature Y
- [ ] Feature Z

### 8.3 Fase 3: Avanzado (Planificado)
- [ ] Feature Alpha
- [ ] Feature Beta
- [ ] Feature Gamma

---

## 📝 9. NOTAS Y DECISIONES

### 9.1 Decisiones de diseño
- [Decisión 1] - Por qué se tomó
- [Decisión 2] - Por qué se tomó

### 9.2 Limitaciones conocidas
- [Limitación 1] - Por qué existe
- [Limitación 2] - Por qué existe

---

## 🔗 10. RELACIONES

### 10.1 Herramientas relacionadas
- **[Herramienta relacionada 1]** - Descripción de relación
- **[Herramienta relacionada 2]** - Descripción de relación

### 10.2 Referencias externas
- [Documentación externa 1] - URL
- [Documentación externa 2] - URL

---

**Generado automáticamente - NO MODIFICAR FORMATO**  
**Sistema de Documentación v1.0.0**  
**AI Responsable:** [AI: opencode]