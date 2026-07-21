//! Tipos de datos para diálogos y nodos de eventos

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// ID único de diálogo en formato: `dial_[mapa]_[entidad]_[checkpoint]`
/// Ejemplo: `dial_bosque1_jaime_p1_prog1`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DialogueId {
    pub map_name: String,
    pub entity_id: String,
    pub checkpoint: u32,
}

impl DialogueId {
    pub fn new(map_name: impl Into<String>, entity_id: impl Into<String>, checkpoint: u32) -> Self {
        Self {
            map_name: map_name.into(),
            entity_id: entity_id.into(),
            checkpoint,
        }
    }

    pub fn full_id(&self) -> String {
        format!(
            "dial_{}_{}_p{}_prog{}",
            self.map_name,
            self.entity_id,
            self.checkpoint / 100,
            self.checkpoint % 100
        )
    }
}

impl std::fmt::Display for DialogueId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.full_id())
    }
}

/// Estilo de diálogo visual
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum DialogueStyle {
    /// Burbuja inflada translúcida (estilo clásico RPG)
    BubbleTransparent,
    /// Caja recta con sombra (estilo moderno)
    BoxShadow,
    /// Estilo cómic con flecha
    ComicArrow,
    /// Estilo narrador (centrado en pantalla)
    Narrator,
}

impl std::fmt::Display for DialogueStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DialogueStyle::BubbleTransparent => write!(f, "Bubble"),
            DialogueStyle::BoxShadow => write!(f, "Box"),
            DialogueStyle::ComicArrow => write!(f, "Comic"),
            DialogueStyle::Narrator => write!(f, "Narrator"),
        }
    }
}

/// Opción de diálogo (decisión del jugador)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueOption {
    pub text: String,
    pub action: Option<DialogueAction>,
    pub next_checkpoint: Option<u32>,
}

/// Acción que se ejecuta al seleccionar una opción
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DialogueAction {
    /// Dar/Recibir ítem
    GiveItem { item_id: String },
    /// Comprobar condición
    CheckCondition { variable: String, value: bool },
    /// Mostrar cinemática
    PlayCinematic { cinematic_id: String },
    /// Cambiar mapa
    ChangeMap { map_name: String },
    /// Ejecutar evento personalizado
    CustomEvent { event_id: String },
}

/// Nodo de diálogo en el grafo de eventos
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueNode {
    pub id: Uuid,
    pub text: String,
    pub style: DialogueStyle,
    pub speaker: Option<String>,
    pub options: Vec<DialogueOption>,
    pub metadata: Metadata,
}

impl DialogueNode {
    pub fn new(id: Uuid, text: impl Into<String>) -> Self {
        Self {
            id,
            text: text.into(),
            style: DialogueStyle::BubbleTransparent,
            speaker: None,
            options: Vec::new(),
            metadata: Metadata::default(),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Metadata {
    pub map_reference: Option<String>,
    pub entity_reference: Option<String>,
    pub checkpoint: u32,
    pub flags: Vec<String>,
}

/// Tipo de nodo en Event Forge
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum NodeType {
    /// Nodo de diálogo de texto
    Dialogue,
    /// Nodo de decisión multiopción (hasta 4 opciones)
    Decision,
    /// Nodo de acción (dar ítem, cambiar estado, etc.)
    Action,
    /// Nodo de condición (if/else)
    Condition,
    /// Nodo de cinemática
    Cinematic,
    /// Nodo de fin de evento
    EndEvent,
}

impl std::fmt::Display for NodeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeType::Dialogue => write!(f, "🟡 DIÁLOGO"),
            NodeType::Decision => write!(f, "🔷 DECISIÓN"),
            NodeType::Action => write!(f, "🔵 ACCIÓN"),
            NodeType::Condition => write!(f, "🟣 CONDICIÓN"),
            NodeType::Cinematic => write!(f, "🎬 CINEMÁTICA"),
            NodeType::EndEvent => write!(f, "⏹️ FIN"),
        }
    }
}

/// Conector de nodo (para el grafo de eventos)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeConnector {
    pub from_node: Uuid,
    pub from_port: u32,
    pub to_node: Uuid,
    pub label: Option<String>,
}
