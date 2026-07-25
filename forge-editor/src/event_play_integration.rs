//! # Event Forge - Play Session Integration
//! 
//! Integración del Event Forge con el Play Session para:
//! - Ejecutar eventos durante el play
//! - Control de entidades desde nodos de eventos
//! - Variables compartidas entre eventos y play session
//! - Callbacks para triggers de eventos

use crate::play_session::{PlaySession, Entity, Vec2};
use crate::event_node_manager::{EventNodeManager};
use crate::event_nodes::{EventType, NodeData};
use std::collections::{HashMap, HashSet};
use std::time::Instant;

/// Integración entre Event Forge y Play Session
pub struct EventPlayIntegration {
    /// Manager de nodos de eventos
    pub event_manager: EventNodeManager,
    /// Sesión de play
    pub play_session: Option<PlaySession>,
    /// Entidades activas en play
    pub active_entities: Vec<Entity>,
    /// Variables compartidas
    pub shared_variables: HashMap<String, String>,
    /// Estado de integración
    pub is_active: bool,
    /// Contador de eventos ejecutados
    pub events_executed: u32,
    /// Timer para eventos de OnTimer
    pub timers: HashMap<String, Instant>,
    /// Callbacks registrados
    pub callbacks: HashMap<String, Vec<String>>,
}

impl Default for EventPlayIntegration {
    fn default() -> Self {
        Self::new()
    }
}

impl EventPlayIntegration {
    /// Crea nueva integración
    pub fn new() -> Self {
        Self {
            event_manager: EventNodeManager::new(),
            play_session: None,
            active_entities: Vec::new(),
            shared_variables: HashMap::new(),
            is_active: false,
            events_executed: 0,
            timers: HashMap::new(),
            callbacks: HashMap::new(),
        }
    }

    /// Inicializa la integración con play session
    pub fn init_with_play_session(&mut self, play_session: PlaySession) {
        self.play_session = Some(play_session);
        self.is_active = true;
        self.event_manager.init_runtime();
        println!("[EVENT-PLAY INTEGRATION] Initialized with play session");
    }

    /// Inicia la integración
    pub fn start(&mut self) {
        if self.play_session.is_some() {
            self.is_active = true;
            self.event_manager.init_runtime();
            println!("[EVENT-PLAY INTEGRATION] Started");
        }
    }

    /// Detiene la integración
    pub fn stop(&mut self) {
        self.is_active = false;
        self.event_manager.runtime_context.is_running = false;
        println!("[EVENT-PLAY INTEGRATION] Stopped");
    }

    /// Actualiza la integración cada frame
    pub fn update(&mut self, delta: f32) {
        if !self.is_active || self.play_session.is_none() {
            return;
        }

        let play_session = self.play_session.as_mut().unwrap();
        
        // Ejecutar eventos de OnTimer si hay nodos temporizadores
        self.execute_timer_events(delta);
        
        // Ejecutar lógica de eventos
        self.execute_events(play_session);
        
        // Incrementar contador
        self.events_executed = self.events_executed.saturating_add(1);
    }

    /// Ejecuta eventos de OnTimer
    fn execute_timer_events(&mut self, delta: f32) {
        let delta_ms = delta * 1000.0;
        
        for node in &mut self.event_manager.nodes {
            if let NodeData::OnTimer { interval_ms, count } = &node.data {
                // Verificar si el temporizador debe dispararse
                if let Some(timer_start) = self.timers.get(&node.id) {
                    let elapsed_ms = (Instant::now() - *timer_start).as_secs_f64() * 1000.0;
                    
                    if elapsed_ms >= (*interval_ms as f64) && *count > 0 {
                        println!("[EVENT-PLAY] Timer fired: {:?} (count: {}/{})", 
                            node.event_type, 
                            node.execution_count, 
                            count);
                        
                        // Ejecutar nodo
                        self.execute_node(&node.id);
                        
                        // Decrementar contador
                        *count = count.saturating_sub(1);
                        
                        // Eliminar timer si se completó
                        if *count == 0 {
                            self.timers.remove(&node.id);
                        } else {
                            // Resetear timer
                            self.timers.insert(node.id.clone(), Instant::now());
                        }
                    }
                } else {
                    // Iniciar primer timer
                    self.timers.insert(node.id.clone(), Instant::now());
                }
            }
        }
    }

    /// Ejecuta todos los eventos activos
    pub fn execute_events(&mut self, play_session: &mut PlaySession) {
        if !self.event_manager.runtime_context.is_running {
            return;
        }

        // Ejecutar nodos de tipo Trigger
        for node in &self.event_manager.nodes {
            match node.event_type {
                // Triggers que se ejecutan automáticamente
                EventType::OnStart => {
                    if let NodeData::OnStart { auto_execute } = &node.data {
                        if *auto_execute {
                            println!("[EVENT-PLAY] Executing OnStart trigger: {:?}", node.id);
                            self.execute_node(&node.id);
                        }
                    }
                }
                // Acciones que requieren ejecución manual
                _ => {
                    // Se ejecutan vía callbacks o ejecución manual
                }
            }
        }
    }

    /// Ejecuta un nodo específico
    pub fn execute_node(&mut self, node_id: &str) {
        if let Some(node) = self.event_manager.get_node_mut(node_id) {
            // Incrementar contador de ejecución
            node.execution_count = node.execution_count.saturating_add(1);
            node.is_active = true;
            
            // Ejecutar nodo según su tipo
            self.execute_node_data(&node.event_type, &mut node.data);
            
            // Ejecutar nodos conectados en cascada
            self.execute_connected_nodes_with_context(&node.id);
        }
    }

    /// Ejecuta datos del nodo según su tipo
    fn execute_node_data(&self, event_type: &EventType, data: &NodeData) {
        match event_type {
            // Acciones de entidad
            EventType::ChangePosition | EventType::Rotate | EventType::Scale => {
                println!("[EVENT-PLAY] Executing action: {:?}", event_type);
            }
            
            // Acciones de animación
            EventType::PlayAnimation | EventType::StopAnimation | 
            EventType::PauseAnimation | EventType::ResumeAnimation => {
                println!("[EVENT-PLAY] Executing animation action: {:?}", event_type);
            }
            
            // Variables
            EventType::ChangeVariable => {
                println!("[EVENT-PLAY] Executing variable action: {:?}", event_type);
            }
            
            // Sonidos
            EventType::PlaySound => {
                println!("[EVENT-PLAY] Executing sound action: {:?}", event_type);
            }
            
            // UI
            EventType::ShowDialog => {
                println!("[EVENT-PLAY] Executing dialog action: {:?}", event_type);
            }
            
            _ => {
                println!("[EVENT-PLAY] Executing node: {:?}", event_type);
            }
        }
    }

    /// Ejecuta nodos conectados
    fn execute_connected_nodes_with_context(&mut self, from_id: &str) {
        let mut visited = HashSet::new();
        let mut queue = std::collections::VecDeque::new();
        
        queue.push_back(from_id.to_string());
        
        while let Some(current_id) = queue.pop_front() {
            if visited.contains(&current_id) {
                continue;
            }
            visited.insert(current_id.clone());
            
            if let Some(node) = self.event_manager.get_node_mut(&current_id) {
                // Actualizar contador
                node.execution_count = node.execution_count.saturating_add(1);
                node.is_active = true;
                
                // Evaluar condiciones de edges
                let mut edges_to_process = Vec::new();
                for edge in &self.event_manager.edges {
                    if edge.from == current_id {
                        let should_execute = if let Some(ref condition) = edge.condition {
                            context.evaluate_condition(condition)
                        } else {
                            true
                        };
                        
                        if should_execute && !visited.contains(&edge.to) {
                            edges_to_process.push(edge.to.clone());
                        }
                    }
                }
                
                // Agregar nodos al queue
                for edge_id in edges_to_process {
                    queue.push_back(edge_id);
                }
            }
        }
    }

    /// Registra un callback para un evento
    pub fn register_callback(&mut self, event_name: &str, callback_id: &str) {
        self.callbacks
            .entry(event_name.to_string())
            .or_insert_with(Vec::new)
            .push(callback_id.to_string());
        
        self.event_manager.runtime_context.register_callback(event_name, callback_id);
        println!("[EVENT-PLAY] Registered callback: {} -> {}", event_name, callback_id);
    }

    /// Obtiene callbacks registrados para un evento
    pub fn get_callbacks(&self, event_name: &str) -> Vec<String> {
        self.callbacks.get(event_name).map(|v| v.clone()).unwrap_or_default()
    }

    /// Establece una variable compartida
    pub fn set_variable(&mut self, name: &str, value: String) {
        self.shared_variables.insert(name.to_string(), value);
        self.event_manager.set_variable(name, value);
        println!("[EVENT-PLAY] Set variable: {} = {}", name, value);
    }

    /// Obtiene una variable compartida
    pub fn get_variable(&self, name: &str) -> Option<String> {
        self.shared_variables.get(name).cloned()
    }

    /// Establece posición de entidad desde evento
    pub fn set_entity_position(&mut self, entity_id: &str, position: Vec2) {
        if let Some(play_session) = &mut self.play_session {
            for entity in &mut play_session.entities {
                if entity.id == entity_id {
                    entity.position = (position.x, position.y);
                    println!("[EVENT-PLAY] Set position for {}: {:?}", entity_id, position);
                    return;
                }
            }
        }
    }

    /// Establece rotación de entidad desde evento
    pub fn set_entity_rotation(&mut self, entity_id: &str, rotation: f32) {
        if let Some(play_session) = &mut self.play_session {
            for entity in &mut play_session.entities {
                if entity.id == entity_id {
                    entity.rotation = rotation;
                    println!("[EVENT-PLAY] Set rotation for {}: {}", entity_id, rotation);
                    return;
                }
            }
        }
    }

    /// Establece escala de entidad desde evento
    pub fn set_entity_scale(&mut self, entity_id: &str, scale: (f32, f32)) {
        if let Some(play_session) = &mut self.play_session {
            for entity in &mut play_session.entities {
                if entity.id == entity_id {
                    entity.scale = scale;
                    println!("[EVENT-PLAY] Set scale for {}: {:?}", entity_id, scale);
                    return;
                }
            }
        }
    }

    /// Obtiene información de integración
    pub fn get_stats(&self) -> HashMap<String, String> {
        let mut stats = HashMap::new();
        stats.insert("is_active".to_string(), self.is_active.to_string());
        stats.insert("events_executed".to_string(), self.events_executed.to_string());
        stats.insert("nodes_count".to_string(), self.event_manager.nodes.len().to_string());
        stats.insert("variables_count".to_string(), self.shared_variables.len().to_string());
        stats.insert("callbacks_count".to_string(), self.callbacks.len().to_string());
        stats
    }

    /// Limpia timers
    pub fn cleanup_timers(&mut self) {
        self.timers.clear();
        println!("[EVENT-PLAY] Cleaned up timers");
    }
}
