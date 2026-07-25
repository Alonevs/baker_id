//! # Test Module - Simple Test
//! 
//! Test simple para verificar que el sistema de tests funciona.

#[test]
fn test_simple_math() {
    let result = 2 + 2;
    assert_eq!(result, 4);
}

#[test]
fn test_string_comparison() {
    let s1 = String::from("hello");
    let s2 = String::from("hello");
    
    assert_eq!(s1, s2);
}

#[test]
fn test_vector_operations() {
    // Test básico de vectores
    let x = 10.0;
    let y = 20.0;
    
    let result = x + y;
    assert_eq!(result, 30.0);
}
