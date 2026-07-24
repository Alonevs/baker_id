//! Delta Sync Optimizado
//!
//! Sistema para enviar solo los cambios reales, no toda la escena.
//! Reduce tráfico de red de 100% a ~10%.

use crate::{EntityId, SyncEvent, Scene};
use std::collections::HashSet;
use uuid::Uuid;

/// Encoder de delta para sincronización eficiente
pub struct DeltaEncoder {
    /// Entidades modificadas en este frame
    pending_changes: HashSet<EntityId>,
}

impl DeltaEncoder {
    /// Crea un nuevo DeltaEncoder
    pub fn new() -> Self {
        Self {
            pending_changes: HashSet::new(),
        }
    }

    /// Marca una entidad como modificada
    pub fn mark_modified(&mut self, entity_id: EntityId) {
        self.pending_changes.insert(entity_id);
    }

    /// Genera deltas entre dos escenas
    pub fn encode_deltas(&self, old_scene: &Scene, new_scene: &Scene) -> Vec<SyncEvent> {
        let mut deltas = Vec::new();

        // Obtener IDs de todas las entidades como Uuid
        let old_ids: HashSet<Uuid> = old_scene.nodes.keys().cloned().collect();
        let new_ids: HashSet<Uuid> = new_scene.nodes.keys().cloned().collect();

        // Detectar entidades eliminadas
        let removed_ids: HashSet<Uuid> = old_ids.difference(&new_ids).cloned().collect();

        for uuid in removed_ids {
            if let Some(_node) = old_scene.nodes.get(&uuid) {
                let entity_id = EntityId(uuid.as_u128() as i32);
                deltas.push(SyncEvent::entity_removed(entity_id));
            }
        }

        // Detectar entidades añadidas
        let added_ids: HashSet<Uuid> = new_ids.difference(&old_ids).cloned().collect();

        for uuid in added_ids {
            if let Some(node) = new_scene.nodes.get(&uuid) {
                let entity_id = EntityId(uuid.as_u128() as i32);
                deltas.push(SyncEvent::entity_added(
                    entity_id,
                    node.entity_type.clone(),
                    Some(serde_json::to_value(&node).unwrap_or_default()),
                ));
            }
        }

        // Detectar cambios en entidades existentes
        let common_ids: HashSet<Uuid> = old_ids.intersection(&new_ids).cloned().collect();

        for uuid in common_ids {
            if let (Some(old_node), Some(new_node)) = (
                old_scene.nodes.get(&uuid),
                new_scene.nodes.get(&uuid),
            ) {
                let entity_id = EntityId(uuid.as_u128() as i32);

                // Verificar cambios de transformación
                if old_node.transform != new_node.transform {
                    deltas.push(SyncEvent::transform_changed(
                        entity_id,
                        new_node.transform.clone(),
                    ));
                }

                // Verificar cambios de tipo
                if old_node.entity_type != new_node.entity_type {
                    deltas.push(SyncEvent::component_changed(
                        entity_id,
                        "entity_type".to_string(),
                        Some(serde_json::to_value(&new_node.entity_type).unwrap_or_default()),
                    ));
                }

                // Verificar cambios de nombre
                if old_node.name != new_node.name {
                    deltas.push(SyncEvent::component_changed(
                        entity_id,
                        "name".to_string(),
                        Some(serde_json::to_value(&new_node.name).unwrap_or_default()),
                    ));
                }
            }
        }

        deltas
    }
}

impl Default for DeltaEncoder {
    fn default() -> Self {
        Self::new()
    }
}
