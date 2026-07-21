//! # Property Panel Example
//! 
//! Ejemplo de uso del Property Panel API

fn main() {
    println!("=== Property Panel Example ===\n");

    // Crear nuevo panel
    let mut panel = forge_editor::property_panel::PropertyPanel::new();
    println!("✅ Panel creado\n");

    // Seleccionar entidad
    let entity_id = forge_editor::property_panel::EntityId(1);
    panel.set_selected_entity(entity_id);
    println!("✅ Entidad seleccionada: {}", entity_id.0);

    // Obtener propiedades
    if let Some(props) = panel.get_transform_properties() {
        println!("✅ Propiedades de transformación:");
        println!("   Position: {:?}", props.position);
        println!("   Rotation: {:?}", props.rotation);
        println!("   Scale: {:?}", props.scale);
        println!("   Visible: {}", props.visible);
    }

    // Establecer propiedades de transformación
    let transform = forge_editor::property_panel::TransformProperties {
        position: (100.0, 200.0, 0.0),
        rotation: (45.0, 90.0, 0.0),
        scale: (2.0, 2.0, 2.0),
        visible: true,
    };
    panel.set_transform_props(transform);
    println!("\n✅ Propiedades actualizadas\n");

    // Agregar componentes
    panel.add_component("Transform");
    panel.add_component("Rigidbody");
    panel.add_component("Sprite");
    println!("✅ Componentes agregados: 3\n");

    // Agregar scripts
    panel.add_script("PlayerScript");
    panel.add_script("EnemyAI");
    println!("✅ Scripts agregados: 2\n");

    // Cambiar pestaña
    panel.set_tab(forge_editor::property_panel::PropertyTab::Component);
    println!("✅ Pestaña cambiada a Component\n");

    // Obtener widgets
    let widgets = panel.get_widgets();
    println!("✅ Widgets en panel: {}", widgets.len());
}
