use super::{BakeState, FramesMode, ModelLoadState, SpriteBakerSettings};
use bevy::prelude::*;
use std::f32::consts::PI;

/// Sistema para centrar y normalizar el modelo automáticamente
pub fn auto_normalize_model(
    settings: Res<SpriteBakerSettings>,
    mut model_entity: Query<&mut Transform, With<ModelRoot>>,
    children_query: Query<&Children>,
    transform_query: Query<&Transform>,
    mesh_query: Query<&Handle<Mesh>>,
    meshes: Res<Assets<Mesh>>,
) {
    if let Some(entity) = settings.spawned_model_entity {
        let mut transform = model_entity.get_mut(entity).unwrap_or_default();
        let mut min = Vec3::new(f32::INFINITY, f32::INFINITY, f32::INFINITY);
        let mut max = Vec3::new(f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY);
        
        traverse_and_accumulate_bounds(
            entity,
            Transform::default(),
            &children_query,
            &transform_query,
            &mesh_query,
            &meshes,
            &mut min,
            &mut max,
        );
        
        let bounds = max - min;
        let height = bounds.y;
        
        if height > 0.001 {
            let target_height = 1.8;
            let scale_factor = target_height / height;
            transform.scale = transform.scale * Vec3::splat(scale_factor);
            transform.translation = (transform.translation - min) * Vec3::new(1.0, 0.0, 1.0);
            transform.translation.x = (transform.translation.x - bounds.x / 2.0) * scale_factor;
            transform.translation.z = (transform.translation.z - bounds.z / 2.0) * scale_factor;
        }
    }
}

fn traverse_and_accumulate_bounds(
    entity: Entity,
    accum_transform: Transform,
    children_query: &Query<&Children>,
    transform_query: &Query<&Transform>,
    mesh_query: &Query<&Handle<Mesh>>,
    meshes: &Assets<Mesh>,
    min: &mut Vec3,
    max: &mut Vec3,
) {
    let local_transform = transform_query.get(entity).cloned().unwrap_or_default();
    let current_transform = accum_transform * local_transform;
    
    if let Ok(mesh_handle) = mesh_query.get(entity) {
        if let Some(mesh) = meshes.get(mesh_handle) {
            if let Some(bevy::render::mesh::VertexAttributeValues::Float32x3(positions)) =
                mesh.attribute(bevy::render::mesh::Mesh::ATTRIBUTE_POSITION)
            {
                for pos in positions {
                    let vertex_local = Vec3::from(*pos);
                    let vertex_world = current_transform.transform_point(vertex_local);
                    *min = min.min(vertex_world);
                    *max = max.max(vertex_world);
                }
            }
        }
    }
    
    if let Ok(children) = children_query.get(entity) {
        for &child in children.iter() {
            traverse_and_accumulate_bounds(
                child,
                current_transform,
                children_query,
                transform_query,
                mesh_query,
                meshes,
                min,
                max,
            );
        }
    }
}

/// Control forzado del reproductor de animaciones durante baking
pub fn control_animations(
    settings: Res<SpriteBakerSettings>,
    graph_state: Res<AnimationGraphState>,
    mut player_query: Query<&mut AnimationPlayer>,
) {
    for mut player in player_query.iter_mut() {
        if let Some(anim_name) = &settings.selected_animation {
            if let Some(node_idx) = graph_state.animation_nodes.get(anim_name) {
                if !player.animation_is_playing(*node_idx) {
                    player.stop_all();
                    player.play(*node_idx).repeat();
                }
                
                if let Some(active_anim) = player.animation_mut(*node_idx) {
                    if settings.is_playing && settings.bake_state == BakeState::Idle {
                        active_anim.set_speed(settings.animation_speed);
                        active_anim.resume();
                        settings.current_time = active_anim.seek_time();
                    } else {
                        active_anim.set_speed(0.0);
                        active_anim.pause();
                        active_anim.seek_to(settings.current_time);
                    }
                }
            }
        }
    }
}

/// Orquestador principal del baking
pub fn bake_orchestrator_system(
    mut settings: ResMut<SpriteBakerSettings>,
    camera_query: Query<&Transform, With<MainCamera>>,
    time: Res<Time>,
    screenshot_manager: Option<Res<ScreenshotManager>>,
) {
    if settings.bake_state == BakeState::Idle {
        return;
    }
    
    match &mut settings.bake_state {
        BakeState::Capturing {
            animation_idx,
            direction_idx,
            frame_idx,
            wait_frames,
            anim_name,
            duration,
            total_frames,
            active_bakes,
            view_suffix,
        } => {
            // Configurar cámara
            let yaw = *direction_idx as f32 * 45.0;
            settings.camera_yaw = yaw;
            
            // Calcular tiempo de animación
            let time_offset = *frame_idx as f32 / *total_frames as f32 * *duration;
            settings.current_time = time_offset;
            
            // Esperar frames para renderizado
            if *wait_frames > 0 {
                *wait_frames -= 1;
                return;
            }
            
            // Tomar screenshot
            if let Some(screenshot) = &screenshot_manager {
                screenshot.capture("sprite_bake");
            }
            
            // Avanzar
            *frame_idx += 1;
            if *frame_idx >= *total_frames {
                *direction_idx += 1;
                *frame_idx = 0;
            }
            if *direction_idx >= settings.export_directions {
                settings.bake_state = BakeState::Finished {
                    message: format!("Baking completo: {} direcciones", settings.export_directions),
                };
            }
        }
        _ => {}
    }
}

/// Actualizar iluminación para seguir la cámara (headlight effect)
pub fn update_lighting(
    settings: Res<SpriteBakerSettings>,
    mut ambient_light: ResMut<AmbientLight>,
    mut light_query: Query<(&mut DirectionalLight, &mut Transform), With<MainLight>>,
    camera_query: Query<&Transform, (With<MainCamera>, Without<MainLight>)>,
) {
    // Desactivado por defecto - activar en el futuro
    if !settings.lighting_enabled {
        return;
    }
    
    ambient_light.brightness = 0.3;
    
    if let Ok(camera_transform) = camera_query.get_single() {
        let light_dir = *camera_transform.forward();
        let translation = camera_transform.translation;
        let target = translation + light_dir;
        
        for (mut light, mut light_transform) in light_query.iter_mut() {
            light.illuminance = 1000.0;
            light.shadows_enabled = false;
            *light_transform = Transform::from_translation(translation).looking_at(target, Vec3::Y);
        }
    }
}

/// Sistema para mover la cámara alrededor del modelo
pub fn rotate_camera_system(
    mut settings: ResMut<SpriteBakerSettings>,
    camera_query: Query<&mut Transform, With<MainCamera>>,
) {
    if settings.bake_state == BakeState::Idle {
        return;
    }
    
    if let Ok(mut camera) = camera_query.get_single_mut() {
        let yaw = settings.camera_yaw;
        let distance = settings.camera_distance;
        let height = settings.camera_height;
        
        let x = distance * yaw.sin();
        let z = distance * yaw.cos();
        let y = height;
        
        camera.translation = Vec3::new(x, y, z);
    }
}

/// Procesa imagen del framebuffer a PNG
pub fn save_image_to_png(
    bevy_image: &Image,
    path: &std::path::Path,
    target_size: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    let width = bevy_image.width();
    let height = bevy_image.height();
    let mut data = bevy_image.data.clone();
    
    let format = bevy_image.texture_descriptor.format;
    
    // Swap BGRA -> RGBA
    if format == TextureFormat::Bgra8Unorm || format == TextureFormat::Bgra8UnormSrgb {
        for chunk in data.chunks_exact_mut(4) {
            chunk.swap(0, 2);
        }
    }
    
    let img_buffer = image::ImageBuffer::<image::Rgba<u8>, _>::from_raw(width, height, data)
        .ok_or("Failed to create image buffer")?;
    let dynamic_img = image::DynamicImage::ImageRgba8(img_buffer);
    
    // Recortar al centro
    let size = width.min(height);
    let x = (width - size) / 2;
    let y = (height - size) / 2;
    let cropped = dynamic_img.crop_imm(x, y, size, size);
    
    // Redimensionar con Lanczos3
    let resized = cropped.resize(target_size, target_size, image::imageops::FilterType::Lanczos3);
    
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    
    resized.save(path)?;
    Ok(())
}

/// Construye la estructura de carpetas de salida
pub fn build_export_path(
    settings: &SpriteBakerSettings,
    frame_suffix: &str,
    angle: f32,
) -> String {
    let category = if settings.export_is_ropero {
        "Ropero_Global"
    } else {
        "Graficos_Base"
    };
    
    let animation_suffix = settings.selected_animation.as_deref().unwrap_or("idle");
    let angle_str = format!("{:.0}º", angle);
    
    format!(
        "{}/{}/{}/{}/{}",
        settings.export_project_path,
        category,
        settings.export_id_asset,
        format!("{}_{}_{}", settings.export_id_asset, animation_suffix, frame_suffix),
        angle_str
    )
}

