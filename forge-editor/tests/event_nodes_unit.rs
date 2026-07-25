//! # Unit Tests - Event Nodes
//! 
//! Tests unitarios para validar EventNodes y RuntimeContext.

use crate::event_nodes::{EventNode, EventType, NodeData, RuntimeContext, Edge, EventGraph};

#[test]
fn test_runtime_context_creation() {
    let context = RuntimeContext::new();
    
    assert!(context.variables.is_empty());
    assert!(context.callbacks.is_empty());
    assert!(!context.is_running);
}

#[test]
fn test_runtime_context_set_get_var() {
    let mut context = RuntimeContext::new();
    
    context.set_var("score", "100");
    
    assert_eq!(context.get_var("score"), Some("100".to_string()));
    assert_eq!(context.get_var("nonexistent"), None);
}

#[test]
fn test_runtime_context_or_create_var() {
    let mut context = RuntimeContext::new();
    
    // Primero crea
    let value1 = context.get_or_create_var("count", "0".to_string());
    assert_eq!(value1, "0");
    
    // Luego obtiene
    let value2 = context.get_or_create_var("count", "999".to_string());
    assert_eq!(value2, "0"); // No debería cambiar
    
    // Modificar
    context.set_var("count", "50");
    assert_eq!(context.get_or_create_var("count", "0".to_string()), "50");
}

#[test]
fn test_runtime_context_remove_var() {
    let mut context = RuntimeContext::new();
    
    context.set_var("temp", "value");
    context.remove_var("temp");
    
    assert_eq!(context.get_var("temp"), None);
}

#[test]
fn test_runtime_context_register_get_callback() {
    let mut context = RuntimeContext::new();
    
    context.register_callback("on_click", "callback1");
    context.register_callback("on_click", "callback2");
    
    let callbacks = context.get_callbacks("on_click");
    assert_eq!(callbacks.len(), 2);
}

#[test]
fn test_runtime_context_callbacks_different_events() {
    let mut context = RuntimeContext::new();
    
    context.register_callback("on_click", "cb1");
    context.register_callback("on_start", "cb2");
    
    assert_eq!(context.get_callbacks("on_click").len(), 1);
    assert_eq!(context.get_callbacks("on_start").len(), 1);
    assert_eq!(context.get_callbacks("nonexistent").len(), 0);
}

#[test]
fn test_runtime_context_evaluate_condition_true() {
    let context = RuntimeContext::new();
    
    // Condición siempre verdadera
    assert!(context.evaluate_condition("true"));
    
    // Variable que es true
    let mut ctx = RuntimeContext::new();
    ctx.set_var("flag", "true");
    assert!(ctx.evaluate_condition("flag:flag"));
}

#[test]
fn test_runtime_context_evaluate_condition_false() {
    let context = RuntimeContext::new();
    
    // Variable que es false
    let mut ctx = RuntimeContext::new();
    ctx.set_var("flag", "false");
    assert!(!ctx.evaluate_condition("flag:flag"));
    
    // Variable que no existe
    assert!(!context.evaluate_condition("flag:nonexistent"));
}

#[test]
fn test_runtime_context_evaluate_condition_variable() {
    let mut context = RuntimeContext::new();
    context.set_var("score", "100");
    
    // Obtener valor de variable
    assert_eq!(context.get_var("score"), Some("100".to_string()));
}

#[test]
fn test_event_node_creation() {
    let node = EventNode {
        id: "node1".to_string(),
        event_type: EventType::OnStart,
        position: eframe::egui::Pos2::new(100.0, 200.0),
        execution_count: 0,
        is_active: false,
        data: NodeData::OnStart { auto_execute: true },
        group_id: None,
    };
    
    assert_eq!(node.id, "node1");
    assert_eq!(node.event_type, EventType::OnStart);
    assert_eq!(node.position, eframe::egui::Pos2::new(100.0, 200.0));
    assert_eq!(node.execution_count, 0);
    assert!(!node.is_active);
}

#[test]
fn test_event_node_execution_count() {
    let mut node = EventNode {
        id: "node1".to_string(),
        event_type: EventType::OnStart,
        position: eframe::egui::Pos2::new(0.0, 0.0),
        execution_count: 5,
        is_active: false,
        data: NodeData::Empty,
        group_id: None,
    };
    
    node.execution_count = node.execution_count.saturating_add(1);
    assert_eq!(node.execution_count, 6);
}

#[test]
fn test_event_graph_creation() {
    let graph = EventGraph::default();
    
    assert_eq!(graph.nodes.len(), 0);
    assert_eq!(graph.edges.len(), 0);
}

#[test]
fn test_event_edge_creation() {
    let edge = Edge {
        from: "node1".to_string(),
        to: "node2".to_string(),
        condition: None,
    };
    
    assert_eq!(edge.from, "node1");
    assert_eq!(edge.to, "node2");
    assert_eq!(edge.condition, None);
}

#[test]
fn test_event_edge_with_condition() {
    let edge = Edge {
        from: "node1".to_string(),
        to: "node2".to_string(),
        condition: Some("score > 100".to_string()),
    };
    
    assert_eq!(edge.condition, Some("score > 100".to_string()));
}

#[test]
fn test_node_data_various_types() {
    // Test OnStart
    let data1 = NodeData::OnStart { auto_execute: true };
    assert!(matches!(data1, NodeData::OnStart { .. }));
    
    // Test OnTimer
    let data2 = NodeData::OnTimer { interval_ms: 1000, count: 5 };
    assert!(matches!(data2, NodeData::OnTimer { .. }));
    
    // Test ChangePosition
    let data3 = NodeData::ChangePosition { x: 100.0, y: 200.0 };
    assert!(matches!(data3, NodeData::ChangePosition { .. }));
    
    // Test PlayAnimation
    let data4 = NodeData::PlayAnimation { 
        target: "player".to_string(), 
        animation: "run".to_string(), 
        duration_ms: 500 
    };
    assert!(matches!(data4, NodeData::PlayAnimation { .. }));
}

#[test]
fn test_event_node_group() {
    let node = EventNode {
        id: "node1".to_string(),
        event_type: EventType::OnStart,
        position: eframe::egui::Pos2::new(0.0, 0.0),
        execution_count: 0,
        is_active: false,
        data: NodeData::Empty,
        group_id: Some("group1".to_string()),
    };
    
    assert_eq!(node.group_id, Some("group1".to_string()));
}

#[test]
fn test_runtime_context_is_running() {
    let mut context = RuntimeContext::new();
    
    assert!(!context.is_running);
    
    context.is_running = true;
    assert!(context.is_running);
    
    context.is_running = false;
    assert!(!context.is_running);
}
