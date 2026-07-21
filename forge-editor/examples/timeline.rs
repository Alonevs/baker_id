//! # Timeline Editor Example
//! 
//! Ejemplo de uso del Timeline Editor API

fn main() {
    println!("=== Timeline Editor Example ===\n");

    // Crear nuevo editor
    let panel = forge_editor::property_panel::PropertyPanel::new();
    let mut timeline = forge_editor::timeline::TimelineEditor::new(panel);
    println!("✅ Timeline creado\n");

    // Crear widgets
    timeline.create_widgets();
    println!("✅ Widgets creados: {}", timeline.get_widgets().len());

    // Agregar keyframes
    timeline.add_keyframe(0, 0.0, "Linear");
    timeline.add_keyframe(10, 1.0, "Linear");
    timeline.add_keyframe(20, 0.0, "Linear");
    println!("✅ Keyframes agregados: 3\n");

    // Obtener valor interpolado
    let value_at_frame_15 = timeline.interpolate(15.0);
    println!("✅ Valor en frame 15: {:.2}", value_at_frame_15);

    // Cambiar frame actual
    timeline.set_current_frame(15);
    println!("✅ Frame actual: {}", timeline.get_current_frame());

    // Obtener panel de propiedades
    let props_panel = timeline.get_property_panel();
    println!("✅ Panel de propiedades obtenido\n");

    // Actualizar
    timeline.update(&egui::Context::default(), &egui::Ui::new());
    println!("✅ Timeline actualizado\n");
}
