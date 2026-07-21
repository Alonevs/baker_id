use forge_editor::export_manager::ExportManager;

fn main() {
    // Crear export manager
    let mut export_manager = ExportManager::new();
    
    // Configurar nombre del proyecto
    export_manager.set_name("Mi Proyecto de Juego");
    
    // Añadir entidad
    export_manager.add_entity(
        "entity_1",
        "Player",
        (0.0, 0.0, 0.0),
        (0.0, 0.0, 0.0),
        (1.0, 1.0, 1.0),
        true,
        vec!["Transform".to_string(), "Rigidbody".to_string()],
        vec!["PlayerScript".to_string()],
    );
    
    // Añadir cuerpo físico
    export_manager.add_body(
        "body_1",
        (0.0, 0.0, 0.0),
        (0.0, 0.0, 0.0),
        (1.0, 1.0, 1.0),
        (0.0, 0.0, 0.0),
    );
    
    // Añadir partícula
    export_manager.add_particle(
        (0.0, 0.0, 0.0),
        (0.0, 0.0, 0.0),
        (1.0, 1.0, 1.0),
        (1.0, 0.0, 0.0),
        10.0,
        100.0,
    );
    
    // Añadir frame de sprite
    export_manager.add_sprite_frame(
        "particles.png",
        (0.0, 0.0, 32.0, 32.0),
        0.1,
    );
    
    // Añadir animación
    export_manager.add_animation(
        "jump",
        vec![
            (0, 0.0, "Linear".to_string()),
            (10, 1.0, "Linear".to_string()),
            (20, 0.0, "Linear".to_string()),
        ],
    );
    
    // Añadir diálogo
    export_manager.add_dialogue(
        "dialogue_1",
        "Player",
        "Hello, world!",
        vec![
            ("Continue".to_string(), "next".to_string()),
        ],
    );
    
    // Añadir evento
    export_manager.add_event(
        "event_1",
        "Start Game",
        "OnStart",
        "PlayMusic",
        vec!["theme.mp3".to_string()],
    );
    
    // Exportar proyecto
    if let Err(e) = export_manager.export("project.map") {
        eprintln!("Error al exportar: {}", e);
    } else {
        println!("Proyecto exportado exitosamente a project.map");
    }
    
    // Importar proyecto
    if let Ok(project) = ExportManager::import("project.map") {
        println!("Proyecto importado: {}", project.name);
        println!("Entidades: {}", project.entities.len());
        println!("Cuerpos: {}", project.physics.bodies.len());
        println!("Partículas: {}", project.particles.particles.len());
        println!("Animaciones: {}", project.animations.animations.len());
        println!("Diálogos: {}", project.dialogues.len());
        println!("Eventos: {}", project.events.len());
    }
}

