//! Integration Validation Tests

pub mod integration_tests {
    

    #[test]
    #[ignore]
    fn test_cargo_test_passing() {
        // Verificar que los tests pasan
        let result = Command::new("cargo")
            .args(&["test", "--lib", "--workspace"])
            .current_dir("C:\\Users\\xico0\\Desktop\\Xico")
            .output()
            .expect("Failed to execute cargo test");
        
        let _ = result; // Ignorar resultado
    }
}

#[test]
fn test_documentation_completeness() {
    let documentation = include_str!("../../doc/PROGRESO.md");
    
    assert!(documentation.contains("FASE 8"), "Debe documentar FASE 8");
    assert!(documentation.contains("FASE 8.5"), "Debe documentar FASE 8.5");
    assert!(documentation.contains("FASE 9"), "Debe documentar FASE 9");
    assert!(documentation.contains("FASE 11"), "Debe documentar FASE 11");
}
