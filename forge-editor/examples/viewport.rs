//! # Viewport Example
//! 
//! Ejemplo de uso del Viewport API

fn main() {
    println!("=== Viewport Example ===\n");

    // Crear nuevo viewport
    let mut viewport = forge_editor::viewport::Viewport::new();
    println!("✅ Viewport creado\n");

    // Valores por defecto
    println!("✅ Zoom inicial: {:.2}", viewport.camera_zoom);
    println!("✅ Offset inicial: {:?}", viewport.camera_offset);
    println!("✅ Grid visible: {}", viewport.show_grid);
    println!("✅ Grid size: {:.2}", viewport.grid_size);

    // Modificar zoom
    viewport.zoom_in();
    println!("\n✅ Zoom aumentado: {:.2}", viewport.camera_zoom);

    viewport.zoom_out();
    println!("✅ Zoom disminuido: {:.2}", viewport.camera_zoom);

    // Resetear zoom
    viewport.reset_zoom();
    println!("✅ Zoom reseteado: {:.2}", viewport.camera_zoom);

    // Establecer zoom manual
    viewport.set_zoom(2.0);
    println!("✅ Zoom establecido: {:.2}", viewport.camera_zoom);

    // Agregar selección
    viewport.add_selection(0);
    viewport.add_selection(1);
    viewport.add_selection(2);
    println!("✅ Entidades seleccionadas: {}", viewport.selected_entities.len());

    // Eliminar selección
    viewport.remove_selection(0);
    println!("✅ Entidades seleccionadas: {}", viewport.selected_entities.len());

    // Mover cámara
    viewport.set_position(100.0, 200.0);
    println!("✅ Cámara movida a: {:?}", viewport.camera_offset);

    // Actualizar viewport
    viewport.update(&egui::Context::default(), &egui::Ui::new(), &egui::Vec2::new(800.0, 600.0));
    println!("\n✅ Viewport actualizado\n");
}
