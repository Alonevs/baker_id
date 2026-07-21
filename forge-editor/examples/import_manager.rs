//! # Import Manager Example
//! 
//! Ejemplo de uso del Import Manager API

fn main() {
    println!("=== Import Manager Example ===\n");

    // Crear import manager
    let mut import_manager = forge_editor::ImportManager::new();
    
    // Importar proyecto
    if let Err(e) = import_manager.import("project.map") {
        eprintln!("Error al importar: {}", e);
        return;
    }
    println!("✅ Proyecto importado\n");

    // Obtener nombre del proyecto
    println!("Nombre del proyecto: {}", import_manager.get_name());
    println!("\nResumen del proyecto:");
    println!("{}", import_manager.get_summary());

    // Validar proyecto
    if let Err(e) = import_manager.validate() {
        eprintln!("Validación fallida: {}", e);
    } else {
        println!("\n✅ Proyecto validado correctamente\n");
    }

    // Obtener información
    let project = import_manager.get_project();
    println!("Entidades: {}", project.entities.len());
    println!("Cuerpos físicos: {}", project.physics.bodies.len());
    println!("Partículas: {}", project.particles.particles.len());
    println!("Animaciones: {}", project.animations.animations.len());
    println!("Diálogos: {}", project.dialogues.len());
    println!("Eventos: {}", project.events.len());

    // Iterar sobre entidades
    println!("\nEntidades:");
    for entity in project.entities.iter() {
        println!("  - {}: {} ({}: {})", 
            entity.id, 
            entity.name,
            "componentes",
            entity.components.len()
        );
    }

    // Iterar sobre diálogos
    println!("\nDiálogos:");
    for dialogue in project.dialogues.iter() {
        println!("  - [{}] {}", dialogue.speaker, dialogue.text);
    }

    // Iterar sobre eventos
    println!("\nEventos:");
    for event in project.events.iter() {
        println!("  - {}: {} -> {}", event.name, event.trigger, event.action);
    }
}
