//! # Transform Editor Example
//! 
//! Ejemplo de uso del Transform Editor API

fn main() {
    println!("=== Transform Editor Example ===\n");

    // Crear nuevo editor
    let panel = forge_editor::property_panel::PropertyPanel::new();
    let mut transform = forge_editor::transform_editor::TransformEditor::new(panel);
    println!("✅ Transform Editor creado\n");

    // Crear widgets
    transform.create_widgets();
    println!("✅ Widgets creados: {}", transform.get_widgets().len());

    // Obtener propiedades de transformación
    if let Some(props) = transform.get_transform_properties() {
        println!("✅ Propiedades de transformación:");
        println!("   Position: {:?}", props.position);
        println!("   Rotation: {:?}", props.rotation);
        println!("   Scale: {:?}", props.scale);
    }

    // Obtener propiedades de componentes
    if let Some(props) = transform.get_component_properties() {
        println!("✅ Componentes: {}", props.components.len());
    }

    // Obtener propiedades de scripts
    if let Some(props) = transform.get_script_properties() {
        println!("✅ Scripts: {}", props.scripts.len());
    }

    // Actualizar
    transform.update(&egui::Context::default(), &egui::Ui::new(), &egui::Vec2::new(800.0, 600.0));
    println!("\n✅ Transform Editor actualizado\n");
}
