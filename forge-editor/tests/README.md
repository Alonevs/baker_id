# 🧪 Testing System - Forge Editor

## 📋 **INSTRUCCIONES**

### **Ejecutar todos los tests**
```bash
cargo test
```

### **Ejecutar tests específicos**
```bash
# Tests unitarios
cargo test --test play_session

# Tests de integración
cargo test --test event_play

# Tests E2E
cargo test --test basic_play

# Tests con output
cargo test -- --nocapture
```

### **Ejecutar tests específicos**
```bash
cargo test test_entity_creation
cargo test test_basic_play_flow
cargo test test_variable_persistence_e2e
```

### **Ignorar tests (si es necesario)**
```bash
cargo test -- --skip slow_tests
```

## 📊 **ESTRUCTURA DE TESTS**

```
tests/
├── unit/
│   ├── play_session.rs      # 15 tests
│   ├── event_nodes.rs       # 14 tests
│   └── input_capture.rs     # 18 tests
├── integration/
│   ├── event_play.rs        # 18 tests
│   └── play_input.rs        # 14 tests
└── e2e/
    ├── basic_play.rs        # 13 tests
    └── event_execution.rs   # 13 tests
```

## 📈 **MÉTRICAS**

### **Tests Unitarios** (47 tests)
- ✅ **play_session.rs**: Tests de entidades, vectores, física
- ✅ **event_nodes.rs**: Tests de nodos de eventos, contexto de runtime
- ✅ **input_capture.rs**: Tests de captura de teclado y mouse

### **Tests de Integración** (32 tests)
- ✅ **event_play.rs**: Tests entre Event Forge y Play Session
- ✅ **play_input.rs**: Tests entre InputCapture y PlaySession

### **Tests E2E** (26 tests)
- ✅ **basic_play.rs**: Flujo completo de play
- ✅ **event_execution.rs**: Ejecución de eventos

**Total: 105 tests**

## 🎯 **TIPOS DE TESTS**

### **Unit Tests**
- Funciones individuales
- Structs y métodos
- Lógica básica
- Validación de tipos

### **Integration Tests**
- Comunicación entre módulos
- Event Forge ↔ Play Session
- InputCapture ↔ PlaySession

### **E2E Tests**
- Flujos completos
- Play loop
- Ejecución de eventos
- Variables compartidas

## 🛠️ **HERRAMIENTAS**

### **cargo test**
```bash
cargo test                    # Ejecutar todos los tests
cargo test -- --nocapture     # Ver output de tests
cargo test -- --test-threads=1 # Ejecutar secuencialmente
```

### **cargo tarpaulin** (opcional)
```bash
cargo tarpaulin --report-html  # Generar reporte de cobertura
```

## 📝 **CHECKLIST DE TESTING**

- [x] Tests unitarios para todos los módulos principales
- [x] Tests de integración entre módulos clave
- [x] Tests E2E para flujos principales
- [x] Tests de entidades y física
- [x] Tests de input capture
- [x] Tests de eventos y play session
- [x] Tests de variables compartidas
- [x] Tests de callbacks
- [x] Tests de timers
- [x] Tests de limpieza y reset

## 🚀 **BEST PRACTICES**

1. **Nombres descriptivos**: `test_entity_creation`, `test_basic_play_flow`
2. **Caso de uso claro**: Cada test debe probar un comportamiento específico
3. **Asserts claros**: Verificar lo que realmente importa
4. **Tests aislados**: No depender del estado de otros tests
5. **Input reproducible**: Usar datos conocidos y predecibles

## 🐛 **DEBUGGING TESTS**

### **Ver detalles de un test fallido**
```bash
cargo test -- --nocapture test_name
```

### **Ejecutar solo tests fallidos**
```bash
cargo test -- --include-ignored
```

### **Mostrar todos los tests**
```bash
cargo test -- --list
```

## 📊 **COVERAGE REPORT**

Para generar un reporte de cobertura:
```bash
cargo tarpaulin --out Html --output-directory ./coverage
cargo tarpaulin --out Text
```

## 🔄 **CI/CD INTEGRATION**

### **GitHub Actions**
```yaml
- name: Run tests
  run: cargo test
```

### **GitLab CI**
```yaml
test:
  script:
    - cargo test
```

## 📚 **DOCUMENTACIÓN**

Los tests incluyen documentación en línea:
- `//!` para módulos
- `///` para funciones y structs
- Ejemplos en tests complejos

## 🎯 **GOALS**

- **Coverage objetivo**: >80%
- **Zero failures**: Siempre
- **Fast feedback**: <30 segundos
- **Reliable**: Tests deterministas
- **Maintainable**: Código limpio y documentado
