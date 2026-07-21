//! # Component Editor Example
//! 
//! Ejemplo de uso del Component Editor API

fn main() {
    println!("=== Component Editor Example ===\n");

    // Crear nuevo editor
    let panel = forge_editor::property_panel::PropertyPanel::new();
    let mut component = forge_editor::component_editor::ComponentEditor::new(panel);
    println!("✅ Component Editor creado\n");

    // Crear widgets
    component.create_widgets();
    println!("✅ Widgets creados: {}", component.get_widgets().len());

    // Obtener propiedades de transformación
    if let Some(props) = component.get_transform_properties() {
        println!("✅ Propiedades de transformación:");
        println!("   Position: {:?}", props.position);
    }

    // Obtener propiedades de componentes
    if let Some(props) = component.get_component_properties() {
        println!("✅ Componentes: {}", props.components.len());
    }

    // Obtener propiedades de scripts
    if let Some(props) = component.get_script_properties() {
        println!("✅ Scripts: {}", props.scripts.len());
    }

    // Actualizar
    component.update(&egui::Context::default(), &egui::Ui::new(), &egui::Vec2::new(800.0, 600.0));
    println!("\n✅ Component Editor actualizado\n");
}
