//! # Event Node Manager Stub
//! 
//! Módulo temporal para manager de nodos de evento.

use eframe::egui;
use crate::event_nodes::{EventNode, Edge as EventEdge, NodeGroup};
use std::collections::{HashMap, VecDeque, HashSet};
use std::sync::{Arc, Mutex};
use crate::event_nodes::RuntimeContext;

/// Event Node Manager stub
#[derive(Default)]
pub struct EventNodeManager {
    pub nodes: Vec<EventNode>,
    pub edges: Vec<EventEdge>,
    pub selected_node: Option<String>,
    pub groups: Vec<NodeGroup>,
    /// Contexto de runtime para ejecución
    pub runtime_context: RuntimeContext,
}

impl EventNodeManager {
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Inicializa el contexto de runtime
    pub fn init_runtime(&mut self) {
        self.runtime_context = RuntimeContext::new();
    }
    
    /// Ejecuta un nodo con contexto de runtime (soporte para variables y callbacks)
    pub fn execute_node_with_context(&mut self, id: String, context: &mut RuntimeContext) {
        if let Some(node) = self.get_node_mut(&id) {
            // Incrementar contador de ejecución
            node.execution_count = node.execution_count.saturating_add(1);
            node.is_active = true;
            
            // Ejecutar nodos conectados en cascada con contexto
            self.execute_connected_nodes_with_context(&id, context);
        }
    }
    
    /// Ejecuta nodos conectados con contexto de runtime
    fn execute_connected_nodes_with_context(&mut self, from_id: &str, context: &mut RuntimeContext) {
        let mut visited = std::collections::HashSet::new();
        let mut queue = std::collections::VecDeque::new();
        
        queue.push_back(from_id.to_string());
        
        while let Some(current_id) = queue.pop_front() {
            if visited.contains(&current_id) {
                continue;
            }
            visited.insert(current_id.clone());
            
            if let Some(node) = self.get_node_mut(&current_id) {
                // Actualizar contador
                node.execution_count = node.execution_count.saturating_add(1);
                node.is_active = true;
                
                // Evaluar condiciones de edges usando contexto
                let mut edges_to_process = Vec::new();
                for edge in &self.edges {
                    if edge.from == current_id {
                        // Evaluar condición del edge si existe
                        let should_execute = if let Some(ref condition) = edge.condition {
                            context.evaluate_condition(condition)
                        } else {
                            true // Sin condición, siempre ejecuta
                        };
                        
                        if should_execute && !visited.contains(&edge.to) {
                            edges_to_process.push(edge.to.clone());
                        }
                    }
                }
                
                // Agregar nodos conectados al queue
                for edge_id in edges_to_process {
                    queue.push_back(edge_id);
                }
            }
        }
    }
    
    /// Registra un callback para un evento
    pub fn register_callback(&mut self, event_name: &str, callback_id: &str) {
        self.runtime_context.register_callback(event_name, callback_id);
    }
    
    /// Obtiene callbacks registrados para un evento
    pub fn get_callbacks(&self, event_name: &str) -> Vec<String> {
        self.runtime_context.get_callbacks(event_name)
    }
    
    /// Establece una variable de runtime
    pub fn set_variable(&mut self, name: &str, value: String) {
        self.runtime_context.set_var(name, value);
    }
    
    /// Obtiene una variable de runtime
    pub fn get_variable(&self, name: &str) -> Option<String> {
        self.runtime_context.get_var(name)
    }
    
    /// Elimina una variable de runtime
    pub fn remove_variable(&mut self, name: &str) {
        self.runtime_context.remove_var(name);
    }
    
    pub fn ui(&self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Event Node Manager");
        });
    }

    pub fn get_all_nodes(&self) -> &[EventNode] {
        &self.nodes
    }

    pub fn get_edges(&self) -> &[EventEdge] {
        &self.edges
    }

    pub fn get_node(&self, id: &str) -> Option<&EventNode> {
        self.nodes.iter().find(|n| n.id == id)
    }

    pub fn get_node_mut(&mut self, id: &str) -> Option<&mut EventNode> {
        self.nodes.iter_mut().find(|n| n.id == id)
    }

    pub fn delete_node(&mut self, id: &str) -> bool {
        let len_before = self.nodes.len();
        self.nodes.retain(|n| n.id != id);
        self.edges.retain(|e| e.from != id && e.to != id);
        self.nodes.len() < len_before
    }

    /// Ejecuta un nodo específico y actualiza su contador
    pub fn execute_node(&mut self, id: String) {
        if let Some(node) = self.get_node_mut(&id) {
            // Incrementar contador de ejecución
            node.execution_count = node.execution_count.saturating_add(1);
            node.is_active = true;
            
            // Ejecutar nodos conectados en cascada (respetando condiciones)
            self.execute_connected_nodes(&id);
        }
    }

    /// Ejecuta todos los nodos en orden topológico
    pub fn execute_all(&mut self) {
        let execution_order = self.get_execution_order();
        for node_id in execution_order {
            if let Some(node) = self.get_node_mut(&node_id) {
                node.execution_count = node.execution_count.saturating_add(1);
                node.is_active = true;
            }
        }
    }

    /// Ejecuta nodos conectados en cascada (BFS) respetando condiciones
    fn execute_connected_nodes(&mut self, from_id: &str) {
        let mut visited = std::collections::HashSet::new();
        let mut queue = std::collections::VecDeque::new();
        
        queue.push_back(from_id.to_string());
        
        while let Some(current_id) = queue.pop_front() {
            if visited.contains(&current_id) {
                continue;
            }
            visited.insert(current_id.clone());
            
            if let Some(node) = self.get_node_mut(&current_id) {
                // Actualizar contador
                node.execution_count = node.execution_count.saturating_add(1);
                node.is_active = true;
                
                // Evaluar condiciones de edges
                let mut edges_to_process = Vec::new();
                for edge in &self.edges {
                    if edge.from == current_id {
                        // Evaluar condición del edge si existe
                        let should_execute = if let Some(ref condition) = edge.condition {
                            self.runtime_context.evaluate_condition(condition)
                        } else {
                            true // Sin condición, siempre ejecuta
                        };
                        
                        if should_execute && !visited.contains(&edge.to) {
                            edges_to_process.push(edge.to.clone());
                        }
                    }
                }
                
                // Agregar nodos conectados al queue
                for edge_id in edges_to_process {
                    queue.push_back(edge_id);
                }
            }
        }
    }

    /// Obtiene el orden de ejecución basado en dependencias (topológico)
    fn get_execution_order(&self) -> Vec<String> {
        let mut in_degree = std::collections::HashMap::new();
        let mut adj_list = std::collections::HashMap::new();
        
        // Inicializar grados de entrada
        for node in &self.nodes {
            in_degree.entry(node.id.clone()).or_insert(0);
            adj_list.insert(node.id.clone(), std::collections::VecDeque::new());
        }
        
        // Calcular grados de entrada
        for edge in &self.edges {
            *in_degree.get_mut(&edge.to).unwrap() += 1;
            adj_list.get_mut(&edge.from).unwrap().push_back(edge.to.clone());
        }
        
        // Cola con nodos que no tienen dependencias
        let mut queue = std::collections::VecDeque::new();
        for (node_id, degree) in &in_degree {
            if degree == &0 {
                queue.push_back(node_id.clone());
            }
        }
        
        let mut result = Vec::new();
        
        while let Some(node_id) = queue.pop_front() {
            result.push(node_id.clone());
            
            if let Some(neighbors) = adj_list.get(&node_id) {
                for neighbor in neighbors {
                    *in_degree.get_mut(neighbor).unwrap() -= 1;
                    if in_degree.get(neighbor).unwrap() == &0 {
                        queue.push_back(neighbor.clone());
                    }
                }
            }
        }
        
        result
    }

    /// Obtiene los contadores de ejecución de todos los nodos
    pub fn get_execution_counts(&self) -> std::collections::HashMap<String, u32> {
        self.nodes.iter().map(|n| (n.id.clone(), n.execution_count)).collect()
    }

    /// Verifica si el grafo contiene ciclos
    pub fn has_cycle(&self) -> bool {
        let mut adj_list: std::collections::HashMap<String, std::collections::VecDeque<String>> = std::collections::HashMap::new();
        
        for node in &self.nodes {
            adj_list.insert(node.id.clone(), std::collections::VecDeque::new());
        }
        
        for edge in &self.edges {
            adj_list.get_mut(&edge.from).unwrap().push_back(edge.to.clone());
        }
        
        let mut visited = std::collections::HashSet::new();
        let mut rec_stack = std::collections::HashSet::new();
        
        fn dfs(node_id: &str, adj_list: &std::collections::HashMap<String, std::collections::VecDeque<String>>, 
               visited: &mut std::collections::HashSet<String>, 
               rec_stack: &mut std::collections::HashSet<String>) -> bool {
            visited.insert(node_id.to_string());
            rec_stack.insert(node_id.to_string());
            
            if let Some(neighbors) = adj_list.get(node_id) {
                for neighbor in neighbors.iter() {
                    if !visited.contains(neighbor) {
                        if dfs(neighbor, adj_list, visited, rec_stack) {
                            return true;
                        }
                    } else if rec_stack.contains(neighbor) {
                        return true;
                    }
                }
            }
            
            rec_stack.remove(node_id);
            false
        }
        
        for node in &self.nodes {
            if !visited.contains(&node.id) {
                if dfs(&node.id, &adj_list, &mut visited, &mut rec_stack) {
                    return true;
                }
            }
        }
        
        false
    }

    pub fn create_node(&mut self, event_type: crate::event_nodes::EventType, _position: (f32, f32)) -> String {
        let id = format!("node_{}", self.nodes.len() + 1);
        self.nodes.push(EventNode {
            id: id.clone(),
            event_type,
            position: egui::Pos2::new(_position.0, _position.1),
            execution_count: 0,
            is_active: false,
            data: crate::event_nodes::NodeData::Empty,
            group_id: None,
        });
        id
    }

    /// Crea un nuevo grupo de nodos
    pub fn create_group(&mut self, name: String) -> String {
        let id = format!("group_{}", self.groups.len() + 1);
        let group = NodeGroup {
            id: id.clone(),
            name,
            is_collapsed: false,
            children: Vec::new(),
        };
        self.groups.push(group);
        id
    }

    /// Elimina un grupo de nodos
    pub fn delete_group(&mut self, group_id: &str) -> bool {
        let nodes_in_group: Vec<String> = self.nodes.iter()
            .filter(|n| n.group_id.as_ref() == Some(&group_id.to_string()))
            .map(|n| n.id.clone())
            .collect();
        
        self.groups.retain(|g| g.id != group_id);
        self.nodes.retain(|n| n.group_id != Some(group_id.to_string()));
        self.edges.retain(|e| !nodes_in_group.contains(&e.from) && !nodes_in_group.contains(&e.to));
        
        !nodes_in_group.is_empty()
    }

    /// Cambia el estado de colapsado/expandido de un grupo
    pub fn toggle_group(&mut self, group_id: &str) -> bool {
        let group = self.groups.iter_mut().find(|g| g.id == group_id);
        if let Some(g) = group {
            g.is_collapsed = !g.is_collapsed;
            true
        } else {
            false
        }
    }

    /// Asigna un nodo a un grupo
    pub fn assign_node_to_group(&mut self, node_id: &str, group_id: &str) -> bool {
        // Encontrar el grupo primero (solo lectura)
        let group_found = self.groups.iter().any(|g| g.id == group_id);
        if !group_found {
            return false;
        }
        
        // Ahora obtener el nodo mutable y asignar el grupo
        if let Some(node) = self.get_node_mut(node_id) {
            node.group_id = Some(group_id.to_string());
            true
        } else {
            false
        }
    }

    /// Desasigna un nodo de su grupo
    pub fn unassign_node_from_group(&mut self, node_id: &str) -> bool {
        if let Some(node) = self.get_node_mut(node_id) {
            node.group_id = None;
            true
        } else {
            false
        }
    }

    /// Obtiene todos los nodos de un grupo
    pub fn get_nodes_in_group(&self, group_id: &str) -> Vec<&EventNode> {
        self.nodes.iter()
            .filter(|n| n.group_id.as_ref() == Some(&group_id.to_string()))
            .collect()
    }

    /// Obtiene todos los grupos con al menos un nodo
    pub fn get_active_groups(&self) -> Vec<&NodeGroup> {
        self.groups.iter()
            .filter(|g| !self.get_nodes_in_group(&g.id).is_empty())
            .collect()
    }

    /// Crea un grupo hijo dentro de un grupo padre
    pub fn create_child_group(&mut self, parent_id: &str, name: String) -> String {
        // Verificar que el grupo padre existe
        let parent_exists = self.groups.iter().any(|g| g.id == parent_id);
        if !parent_exists {
            return String::new();
        }
        
        // Crear el nuevo grupo con el padre como referencia
        let id = format!("group_{}", self.groups.len() + 1);
        let group = NodeGroup {
            id: id.clone(),
            name,
            is_collapsed: false,
            children: Vec::new(),
        };
        
        // Agregar el hijo al padre
        self.groups.iter_mut().for_each(|g| {
            if g.id == parent_id {
                g.children.push(id.clone());
            }
        });
        
        self.groups.push(group);
        id
    }

    /// Elimina un grupo y sus hijos (recursivo)
    pub fn delete_group_with_children(&mut self, group_id: &str) -> bool {
        // Obtener todos los IDs de grupos a eliminar (incluyendo hijos)
        let mut ids_to_delete: Vec<String> = Vec::new();
        let mut to_process = vec![group_id.to_string()];
        
        while let Some(current_id) = to_process.pop() {
            if !ids_to_delete.contains(&current_id) {
                ids_to_delete.push(current_id.clone());
                if let Some(group) = self.groups.iter().find(|g| g.id == current_id) {
                    for child_id in &group.children {
                        to_process.push(child_id.clone());
                    }
                }
            }
        }
        
        // Eliminar nodos de todos los grupos a borrar
        for id in ids_to_delete.iter() {
            self.nodes.retain(|n| n.group_id != Some(id.clone()));
        }
        
        // Eliminar edges que conectan nodos de estos grupos
        for id in ids_to_delete.iter() {
            self.edges.retain(|e| {
                !ids_to_delete.contains(&e.from) && !ids_to_delete.contains(&e.to)
            });
        }
        
        // Eliminar los grupos
        for id in ids_to_delete.iter() {
            self.groups.retain(|g| g.id != id.to_string());
        }
        
        !ids_to_delete.is_empty()
    }

    /// Mueve un grupo a ser hijo de otro grupo
    pub fn move_group(&mut self, from_parent_id: &str, to_parent_id: &str, group_id: &str) -> bool {
        // Verificar que el grupo existe
        let group_exists = self.groups.iter().any(|g| g.id == group_id);
        if !group_exists {
            return false;
        }
        
        // Verificar que los grupos padre existen
        let from_parent_exists = self.groups.iter().any(|g| g.id == from_parent_id);
        let to_parent_exists = self.groups.iter().any(|g| g.id == to_parent_id);
        if !from_parent_exists || !to_parent_exists {
            return false;
        }
        
        // Eliminar del padre actual
        self.groups.iter_mut().for_each(|g| {
            if g.id == from_parent_id {
                g.children.retain(|child_id| child_id != group_id);
            }
        });
        
        // Agregar al nuevo padre (si es diferente)
        if from_parent_id != to_parent_id {
            self.groups.iter_mut().for_each(|g| {
                if g.id == to_parent_id {
                    g.children.push(group_id.to_string());
                }
            });
        }
        
        true
    }

    /// Obtiene todos los grupos anidados (padres + hijos)
    pub fn get_group_with_descendants(&self, group_id: &str) -> Vec<&NodeGroup> {
        let mut result = Vec::new();
        let mut to_process = vec![group_id.to_string()];
        
        while let Some(current_id) = to_process.pop() {
            if let Some(group) = self.groups.iter().find(|g| g.id == current_id) {
                result.push(group);
                for child_id in &group.children {
                    if child_id != group_id {
                        to_process.push(child_id.clone());
                    }
                }
            }
        }
        
        result
    }

    /// Obtiene todos los nodos dentro de un grupo y sus hijos (anidados)
    pub fn get_all_nodes_in_group(&self, group_id: &str) -> Vec<&EventNode> {
        let mut result = Vec::new();
        let mut to_process = vec![group_id.to_string()];
        
        while let Some(current_group_id) = to_process.pop() {
            // Agregar nodos del grupo actual
            result.extend(self.get_nodes_in_group(&current_group_id));
            
            // Agregar grupos hijos al proceso
            if let Some(group) = self.groups.iter().find(|g| g.id == current_group_id) {
                for child_id in &group.children {
                    if child_id.as_str() == group_id {
                        continue;
                    }
                    to_process.push(child_id.clone());
                }
            }
        }
        
        result
    }

    pub fn get_graph(&self) -> crate::event_nodes::EventGraph {
        crate::event_nodes::EventGraph {
            nodes: self.nodes.clone(),
            edges: self.edges.clone(),
        }
    }

    pub fn load_graph(&mut self, graph: crate::event_nodes::EventGraph) {
        self.nodes = graph.nodes;
        self.edges = graph.edges;
    }

    /// Obtiene todos los edges
    pub fn get_all_edges(&self) -> Vec<crate::event_nodes::Edge> {
        self.edges.clone()
    }

    /// Elimina un nodo
    pub fn remove_node(&mut self, node_id: &str) -> bool {
        self.nodes.retain(|n| n.id != node_id);
        self.edges.retain(|e| e.from != node_id && e.to != node_id);
        true
    }

    /// Conecta dos nodos
    pub fn connect_nodes(&mut self, from: &str, to: &str, condition: Option<String>) -> bool {
        self.edges.push(EventEdge {
            from: from.to_string(),
            to: to.to_string(),
            condition,
        });
        true
    }

    /// Elimina una conexión
    pub fn disconnect_nodes(&mut self, from: &str, to: &str) -> bool {
        self.edges.retain(|e| !(e.from == from && e.to == to));
        true
    }

    /// Obtiene el nodo seleccionado
    pub fn get_selected_node(&self) -> Option<&crate::event_nodes::EventNode> {
        self.selected_node.as_ref().and_then(|id| self.nodes.iter().find(|n| n.id == *id))
    }

    /// Selecciona un nodo
    pub fn select_node(&mut self, node_id: Option<String>) {
        self.selected_node = node_id;
    }

    /// Elimina un grupo
    pub fn remove_group(&mut self, group_id: &str) -> bool {
        self.groups.retain(|g| g.id != group_id);
        self.nodes.retain(|n| n.group_id != Some(group_id.to_string()));
        true
    }
}

