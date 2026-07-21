//! # Export Manager Example
//! 
//! Ejemplo de uso del Export Manager API

fn main() {
    println!("=== Export Manager Example ===\n");

    // Crear export manager
    let mut export_manager = forge_editor::map_export::ExportManager::new();
    println!("✅ Export manager creado\n");

    // Crear datos de física
    let physics = forge_editor::map_export::PhysicsExport {
        blocks: vec![],
    };
    export_manager.set_physics(physics);
    println!("✅ Física configurada\n");

    // Crear datos de partículas
    let particles = forge_editor::map_export::ParticlesExport {
        particles: vec![],
    };
    export_manager.set_particles(particles);
    println!("✅ Partículas configuradas\n");

    // Crear datos de animaciones
    let animations = forge_editor::map_export::AnimationsExport {
        animations: vec![],
    };
    export_manager.set_animations(animations);
    println!("✅ Animaciones configuradas\n");

    // Crear datos de diálogos
    let dialogues = forge_editor::map_export::DialoguesExport {
        dialogues: vec![],
    };
    export_manager.set_dialogues(dialogues);
    println!("✅ Diálogos configurados\n");

    // Crear datos de eventos
    let events = forge_editor::map_export::EventsExport {
        events: vec![],
    };
    export_manager.set_events(events);
    println!("✅ Eventos configurados\n");

    // Exportar a formato .map
    let map_content = export_manager.export_to_map();
    println!("✅ Proyecto exportado a formato .map\n");

    // Mostrar contenido exportado
    println!("Contenido del archivo .map:\n{}", map_content);

    // Importar desde formato .map
    export_manager.import_from_map(&map_content);
    println!("✅ Proyecto importado desde formato .map\n");

    println!("Resumen del proyecto exportado:");
    println!("  Física: {:?}", export_manager.physics.is_some());
    println!("  Partículas: {:?}", export_manager.particles.is_some());
    println!("  Animaciones: {:?}", export_manager.animations.is_some());
    println!("  Diálogos: {:?}", export_manager.dialogues.is_some());
    println!("  Eventos: {:?}", export_manager.events.is_some());
}
