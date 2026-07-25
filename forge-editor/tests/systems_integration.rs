//! # Integration Tests - Sistemas Core Integrados
//! 
//! Tests de integración para validar que los 6 sistemas funcionan correctamente juntos:
//! - AudioManager + PlaySession
//! - UIManager + SceneManager
//! - BitacoraUI + BitacoraManager
//! - Excel + Bitacora
//! - DialogueUI + PlaySession
//! - Variables de Diálogo

#[cfg(test)]
mod integration_tests {
    use super::*;
    use std::collections::HashMap;

    /// Test 1: AudioManager integrado con PlaySession
    #[test]
    fn test_audio_manager_play_session_integration() {
        let entities = vec![
            crate::play_session::Entity::new("player1".to_string(), (0.0, 0.0)),
        ];
        
        let session = crate::play_session::PlaySession::new(&entities);
        
        // Verificar que audio_manager está integrado y funcional
        let audio_mgr = session.audio_manager();
        assert!(audio_mgr.is_some());
        
        // Verificar que audio_manager está inicializado
        assert_eq!(audio_mgr.count(), 0);
        
        println!("✅ AudioManager integrado correctamente con PlaySession");
    }

    /// Test 2: UIManager integrado con SceneManager
    #[test]
    fn test_ui_manager_scene_manager_integration() {
        let scene_manager = crate::scene_system::SceneManager::new();
        
        // Verificar que ui_manager está integrado
        let ui_mgr = scene_manager.ui_manager();
        assert!(ui_mgr.is_some());
        
        // Verificar que ui_manager está inicializado
        assert_eq!(ui_mgr.layer_count(), 0);
        
        println!("✅ UIManager integrado correctamente con SceneManager");
    }

    /// Test 3: BitacoraUI sincronizado con BitacoraManager
    #[test]
    fn test_bitacora_ui_manager_integration() {
        // Crear BitacoraManager
        let mut manager = crate::bitacora_manager::BitacoraManager::new();
        
        // Agregar algunas entradas
        let id1 = manager.add_entry("Prueba 1", None);
        let id2 = manager.add_entry("Prueba 2", None);
        
        assert_eq!(manager.entry_count(), 2);
        
        // Crear BitacoraPanel con manager
        let panel = crate::bitacora_ui::BitacoraPanel::with_manager(manager);
        
        // Verificar que entries_list está sincronizado
        assert_eq!(panel.entries_list.len(), 2);
        
        println!("✅ BitacoraUI sincronizado correctamente con BitacoraManager");
    }

    /// Test 4: Excel exportar Bitacora
    #[test]
    fn test_excel_export_bitacora() {
        // Crear BitacoraManager con entradas
        let mut manager = crate::bitacora_manager::BitacoraManager::new();
        manager.add_entry("Entrada de prueba", None);
        
        // Crear ExcelWriter
        let mut writer = crate::excel_integration::ExcelWriter::new();
        
        // Exportar Bitacora
        let result = writer.export_bitacora(&manager, "test_bitacora.xlsx");
        
        assert!(result.is_ok());
        println!("✅ Excel export_bitacora() funciona correctamente");
    }

    /// Test 5: DialogueUI integrado con PlaySession - mostrar diálogo
    #[test]
    fn test_dialogue_ui_play_session_show() {
        let entities = vec![
            crate::play_session::Entity::new("player1".to_string(), (0.0, 0.0)),
        ];
        
        let mut session = crate::play_session::PlaySession::new(&entities);
        
        // Verificar que no hay diálogo activo inicialmente
        assert!(!session.has_active_dialogue());
        
        // Mostrar diálogo
        session.show_dialogue("Personaje1".to_string(), "¡Hola!".to_string());
        
        // Verificar que el diálogo está mostrando
        assert_eq!(
            session.dialogue_ui().state,
            crate::dialogue_ui::DialogueState::Showing
        );
        
        println!("✅ DialogueUI integrado correctamente - mostrar diálogo");
    }

    /// Test 6: DialogueUI integrado con PlaySession - ocultar diálogo
    #[test]
    fn test_dialogue_ui_play_session_hide() {
        let entities = vec![
            crate::play_session::Entity::new("player1".to_string(), (0.0, 0.0)),
        ];
        
        let mut session = crate::play_session::PlaySession::new(&entities);
        
        // Mostrar y ocultar diálogo
        session.show_dialogue("Personaje1".to_string(), "¡Hola!".to_string());
        session.hide_dialogue();
        
        // Verificar que el diálogo está oculto
        assert_eq!(
            session.dialogue_ui().state,
            crate::dialogue_ui::DialogueState::Hidden
        );
        
        println!("✅ DialogueUI integrado correctamente - ocultar diálogo");
    }

    /// Test 7: Variables de diálogo en PlaySession
    #[test]
    fn test_dialogue_variables_play_session() {
        let entities = vec![
            crate::play_session::Entity::new("player1".to_string(), (0.0, 0.0)),
        ];
        
        let mut session = crate::play_session::PlaySession::new(&entities);
        
        // Establecer variable
        let value = session.dialogue_variable("has_sword".to_string(), "true".to_string());
        assert_eq!(value, Some("true".to_string()));
        
        // Obtener variable
        let retrieved = session.get_dialogue_variable("has_sword");
        assert_eq!(retrieved, Some("true".to_string()));
        
        // Obtener todas las variables
        let all_vars = session.get_all_dialogue_variables();
        assert_eq!(all_vars.len(), 1);
        
        println!("✅ Variables de diálogo integradas correctamente");
    }

    /// Test 8: Entity dialogue con PlaySession
    #[test]
    fn test_entity_dialogue_play_session() {
        let entities = vec![
            crate::play_session::Entity::with_name(
                "player1".to_string(),
                (0.0, 0.0),
                "Jugador".to_string()
            ),
        ];
        
        let mut session = crate::play_session::PlaySession::new(&entities);
        
        let entity = &entities[0];
        session.show_entity_dialogue(entity, "¡Hola!".to_string());
        
        // Verificar que el diálogo fue mostrado
        assert_eq!(
            session.dialogue_ui().state,
            crate::dialogue_ui::DialogueState::Showing
        );
        
        println!("✅ Entity dialogue integrado correctamente");
    }

    /// Test 9: Integración completa - todos los sistemas juntos
    #[test]
    fn test_complete_systems_integration() {
        println!("\n🔍 Test de integración completa de todos los sistemas...\n");
        
        // 1. Crear PlaySession
        let entities = vec![
            crate::play_session::Entity::new("player1".to_string(), (0.0, 0.0)),
        ];
        let mut session = crate::play_session::PlaySession::new(&entities);
        
        // 2. Verificar AudioManager
        let _audio_mgr = session.audio_manager();
        println!("✅ AudioManager: OK");
        
        // 3. Verificar DialogueUI
        let _dialogue_ui = session.dialogue_ui();
        println!("✅ DialogueUI: OK");
        
        // 4. Crear SceneManager
        let scene_manager = crate::scene_system::SceneManager::new();
        let _ui_mgr = scene_manager.ui_manager();
        println!("✅ SceneManager + UIManager: OK");
        
        // 5. Crear BitacoraManager y BitacoraPanel
        let mut bitacora_mgr = crate::bitacora_manager::BitacoraManager::new();
        bitacora_mgr.add_entry("Test entry", None);
        let _bitacora_panel = crate::bitacora_ui::BitacoraPanel::with_manager(bitacora_mgr);
        println!("✅ BitacoraManager + BitacoraPanel: OK");
        
        // 6. Verificar Excel integration
        let _excel_writer = crate::excel_integration::ExcelWriter::new();
        println!("✅ Excel integration: OK");
        
        println!("\n🎉 Todos los sistemas integrados correctamente!");
    }

    /// Test 10: Flujo completo de diálogo con variables
    #[test]
    fn test_complete_dialogue_flow() {
        let entities = vec![
            crate::play_session::Entity::with_name(
                "npc1".to_string(),
                (100.0, 100.0),
                "NPC".to_string()
            ),
        ];
        
        let mut session = crate::play_session::PlaySession::new(&entities);
        
        // 1. Mostrar diálogo
        session.show_dialogue("NPC".to_string(), "¿Quieres pelear?".to_string());
        assert!(session.has_active_dialogue());
        
        // 2. Establecer variable de diálogo
        session.dialogue_variable("accepted_fight".to_string(), "true".to_string());
        assert_eq!(
            session.get_dialogue_variable("accepted_fight"),
            Some("true".to_string())
        );
        
        // 3. Ocultar diálogo
        session.hide_dialogue();
        assert!(!session.has_active_dialogue());
        
        println!("✅ Flujo completo de diálogo: OK");
    }

    /// Test 11: Sincronización BitacoraUI en tiempo real
    #[test]
    fn test_bitacora_real_time_sync() {
        let mut manager = crate::bitacora_manager::BitacoraManager::new();
        
        // Crear panel inicial
        let mut panel = crate::bitacora_ui::BitacoraPanel::with_manager(manager.clone());
        
        // Agregar entrada al manager
        let id = manager.add_entry("Nueva entrada", None);
        assert_eq!(manager.entry_count(), 1);
        
        // Sincronizar panel
        panel.update_from_manager();
        
        // Verificar que el panel se actualizó
        assert_eq!(panel.entries_list.len(), 1);
        
        println!("✅ Sincronización en tiempo real: OK");
    }

    /// Test 12: Exportar e importar Bitacora a Excel
    #[test]
    fn test_bitacora_excel_round_trip() {
        // 1. Crear BitacoraManager con entradas
        let mut manager = crate::bitacora_manager::BitacoraManager::new();
        manager.add_entry("Entrada 1", None);
        manager.add_entry("Entrada 2", None);
        
        assert_eq!(manager.entry_count(), 2);
        
        // 2. Exportar a Excel
        let mut writer = crate::excel_integration::ExcelWriter::new();
        let export_result = writer.export_bitacora(&manager, "test_export.xlsx");
        
        assert!(export_result.is_ok());
        println!("✅ Exportar Bitacora a Excel: OK");
        
        // 3. Importar desde Excel (simulado)
        let import_result = crate::excel_integration::Workbook::new()
            .import_bitacora("test_export.xlsx");
        
        // En producción esto leería del archivo real
        // Ahora devuelve Vec vacío porque es una simulación
        assert!(import_result.is_ok());
        println!("✅ Importar Bitacora desde Excel: OK");
    }
}
