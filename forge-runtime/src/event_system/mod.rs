use forge_editor::event_node_manager::EventNodeManager;

/// Runtime Event Manager que integra EventNodeManager
pub struct EventManager {
    /// Manager de nodos de evento
    pub node_manager: EventNodeManager,
}

impl EventManager {
    /// Crea un nuevo EventManager con EventNodeManager inicializado
    pub fn new() -> Self {
        let mut node_manager = EventNodeManager::new();
        node_manager.init_runtime();
        Self { node_manager }
    }

    /// Obtiene referencia al EventNodeManager
    pub fn get_node_manager(&self) -> &EventNodeManager {
        &self.node_manager
    }

    /// Obtiene mutación al EventNodeManager
    pub fn get_node_manager_mut(&mut self) -> &mut EventNodeManager {
        &mut self.node_manager
    }

    /// Ejecuta un nodo específico
    pub fn execute_node(&mut self, node_id: String) {
        self.node_manager.execute_node(node_id);
    }

    /// Ejecuta todos los nodos en orden topológico
    pub fn execute_all_nodes(&mut self) {
        self.node_manager.execute_all();
    }

    /// Ejecuta un nodo con contexto de runtime
    pub fn execute_node_with_context(&mut self, node_id: String, _context: &mut forge_editor::event_nodes::RuntimeContext) {
        self.node_manager.execute_node_with_context(node_id, _context);
    }

    /// Registra un callback para un evento
    pub fn register_callback(&mut self, event_name: &str, callback_id: &str) {
        self.node_manager.register_callback(event_name, callback_id);
    }

    /// Obtiene callbacks registrados para un evento
    pub fn get_callbacks(&self, event_name: &str) -> Vec<String> {
        self.node_manager.get_callbacks(event_name)
    }

    /// Establece una variable de runtime
    pub fn set_variable(&mut self, name: &str, value: String) {
        self.node_manager.set_variable(name, value);
    }

    /// Obtiene una variable de runtime
    pub fn get_variable(&self, name: &str) -> Option<String> {
        self.node_manager.get_variable(name)
    }

    /// Elimina una variable de runtime
    pub fn remove_variable(&mut self, name: &str) {
        self.node_manager.remove_variable(name);
    }

    /// Obtiene el grafo de eventos
    pub fn get_graph(&self) -> forge_editor::event_nodes::EventGraph {
        self.node_manager.get_graph()
    }

    /// Carga un grafo de eventos
    pub fn load_graph(&mut self, graph: forge_editor::event_nodes::EventGraph) {
        self.node_manager.load_graph(graph);
    }

    /// Verifica si el grafo tiene ciclos
    pub fn has_cycle(&self) -> bool {
        self.node_manager.has_cycle()
    }

    /// Obtiene contadores de ejecución
    pub fn get_execution_counts(&self) -> std::collections::HashMap<String, u32> {
        self.node_manager.get_execution_counts()
    }

    /// Obtiene todos los nodos
    pub fn get_all_nodes(&self) -> &[forge_editor::event_nodes::EventNode] {
        self.node_manager.get_all_nodes()
    }

    /// Obtiene todos los edges
    pub fn get_all_edges(&self) -> Vec<forge_editor::event_nodes::Edge> {
        self.node_manager.get_all_edges()
    }

    /// Crea un nuevo nodo de evento
    pub fn create_node(&mut self, event_type: forge_editor::event_nodes::EventType, position: (f32, f32)) -> String {
        self.node_manager.create_node(event_type, position)
    }

    /// Elimina una conexión
    pub fn disconnect_nodes(&mut self, from: &str, to: &str) -> bool {
        self.node_manager.disconnect_nodes(from, to)
    }

    /// Obtiene el nodo seleccionado
    pub fn get_selected_node(&self) -> Option<&forge_editor::event_nodes::EventNode> {
        self.node_manager.get_selected_node()
    }

    /// Selecciona un nodo
    pub fn select_node(&mut self, node_id: Option<String>) {
        self.node_manager.select_node(node_id);
    }

    /// Crea un grupo
    pub fn create_group(&mut self, name: String) -> String {
        self.node_manager.create_group(name)
    }

    /// Elimina un grupo
    pub fn remove_group(&mut self, group_id: &str) -> bool {
        self.node_manager.remove_group(group_id)
    }

    /// Asigna un nodo a un grupo
    pub fn assign_node_to_group(&mut self, node_id: &str, group_id: &str) {
        self.node_manager.assign_node_to_group(node_id, group_id);
    }

    /// Obtiene todos los grupos activos
    pub fn get_active_groups(&self) -> Vec<&forge_editor::event_nodes::NodeGroup> {
        self.node_manager.get_active_groups()
    }

    /// Ejecuta el grafo de eventos
    pub fn run_graph(&mut self) {
        self.node_manager.execute_all();
    }

    /// Verifica si un evento debería ejecutarse basado en condiciones
    pub fn should_execute(&self, condition: &str) -> bool {
        self.node_manager.runtime_context.evaluate_condition(condition)
    }
}

impl Default for EventManager {
    fn default() -> Self {
        Self::new()
    }
}
