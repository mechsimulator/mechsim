//! This example demonstrates the built-in 3d shapes in Bevy.
//! The scene includes a patterned texture and a rotation for visualizing the normals and UVs.

use std::{f32::consts::PI, path::Path};

use bevy::{
    prelude::*,
    render::{render_resource::{Extent3d, TextureDimension, TextureFormat}, mesh::{MeshVertexAttribute, MeshPlugin}}, asset::diagnostic::AssetCountDiagnosticsPlugin, diagnostic::FrameTimeDiagnosticsPlugin, sprite::MaterialMesh2dBundle,
};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use log::LogPlugin;
use rand::prelude::*;
use bevy::render::mesh::{self, PrimitiveTopology};
use bevy::input::mouse::MouseMotion;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use mrr::{MrrDeserializer, Assembly, MrrPlugin};
use ui::UIPlugin;

pub mod mrr;
pub mod ui;
pub mod log;

fn setup_models(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assembly: Res<Assembly>
) {

    for part in &assembly.parts {

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        
        let body = &part.bodies[0];
        dbg!(body.verticies.len());
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, body.verticies.chunks(3).map(|v| [v[0] / 6., v[1] / 6., v[2] / 6.]).collect::<Vec<[f32; 3]>>());
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, body.verticies.chunks(3).map(|v| [v[0], v[1], v[2]]).collect::<Vec<[f32; 3]>>());
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, body.uvs.chunks(2).map(|v| [v[0], v[1]]).collect::<Vec<[f32; 2]>>());
        
        mesh.set_indices(Some(mesh::Indices::U32(body.indicies.iter().map(|&x| x as u32).collect())));
    
        let mut rng = rand::thread_rng();


        commands.spawn(PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(Color::rgb(rng.gen(), rng.gen(), rng.gen()).into()),
            ..default()
        });
    }

    for part in &assembly.parts {
        if !part.name.contains("Bearing") {
            continue;
        }

        let mut mesh = Mesh::new(PrimitiveTopology::LineList);
        
        let body = &part.bodies[0];
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, body.verticies.chunks(3).map(|v| [v[0] / 6., v[1] / 6., v[2] / 6.]).collect::<Vec<[f32; 3]>>());
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, body.verticies.chunks(3).map(|v| [v[0], v[1], v[2]]).collect::<Vec<[f32; 3]>>());
        // mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0., 1., 0.]; body.verticies.len() / 3]);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, body.uvs.chunks(2).map(|v| [v[0], v[1]]).collect::<Vec<[f32; 2]>>());
        
        mesh.set_indices(Some(mesh::Indices::U32(body.indicies.iter().map(|&x| x as u32).collect())));
    
        let mut rng = rand::thread_rng();


        commands.spawn(PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(Color::rgb(rng.gen(), rng.gen(), rng.gen()).into()),
            ..default()
        });
    }

}

#[derive(Resource, Default)]
struct Age(i32);
#[derive(Resource, Default)]
struct Name(String);

fn ui_example_system(mut contexts: EguiContexts, mut age: ResMut<Age>, mut name: ResMut<Name>) {
    egui::Window::new("Hello").show(contexts.ctx_mut(), |ui| {
        ui.heading("My egui Application");
        ui.horizontal(|ui| {
            ui.label("Your name: ");
            ui.text_edit_singleline(&mut name.0);
        });
        ui.add(egui::Slider::new(&mut age.0, 0..=1000000).text("age"));
        if ui.button("Click each year").clicked() {
            age.0 += 1;
        }
        ui.label(format!("Hello '{}', age {}", name.0, age.0));
    });
}

#[derive(Component)]
struct JointIndicator; 

fn joint_icons_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    assembly: Res<Assembly>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for joint in &assembly.joints {
        let vec = joint.1.0;
        dbg!(vec);
        // commands.spawn((PbrBundle {
        //     mesh: meshes.add(Mesh::from(shape::Cube { size: 0.3 })),
        //     material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        //     transform: Transform::from_xyz((vec.x / 6.) as f32, (vec.y / 6.) as f32, (vec.z / 6.) as f32),
        //     ..default()
        // }, JointIndicator));

        commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
            material: materials.add(StandardMaterial::from(Color::PURPLE)),
            transform: Transform::from_xyz((vec.x / 6.) as f32, (vec.y / 6.) as f32, (vec.z / 6.) as f32),
            ..default()
        });
        // commands.spawn(
        //     MaterialMesh2dBundle {
        //         mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
        //         transform: Transform::from_xyz((vec.x / 6.) as f32, (vec.y / 6.) as f32, (vec.z / 6.) as f32),
        //         material: materials.add(ColorMaterial::from(Color::PURPLE)),
        //         ..default()
        //     }
        // );
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(UIPlugin)
        .add_plugin(MrrPlugin)
        .add_plugin(LogPlugin)
        .add_system(ui_example_system)
        .add_startup_system(setup)
        .add_startup_system(setup_models)
        .add_system(rotate)
        .add_system(bevy::window::close_on_esc)
        .add_system(camera_controller)
        .add_startup_system(joint_icons_setup)
        .insert_resource(MrrDeserializer::load(Path::new("C:\\Users\\Public\\MechSim\\assemblies\\ChassisBot v3.mrr")).unwrap().deserialize_assembly().unwrap())
        .insert_resource(Age::default())
        .insert_resource(Name::default())
        .run();

}

/// A marker component for our shapes so we can query them separately from the ground plane
#[derive(Component)]
struct Shape;

const X_EXTENT: f32 = 14.5;

fn move_light(
    mut parent: Query<(Entity, &mut Children), With<Camera>>,
    camera_transform: Query<&Transform, With<Camera>>,
    mut light_transform: Query<&mut Transform, With<PointLight>>
) {
    if let Ok((entity, children)) = parent.get_single() {
        if let Ok(parent_transform) = camera_transform.get(entity) {
            for child in children {
                if let Ok(mut transform) = light_transform.get(*child) {
                    transform = parent_transform;
                }
            }
        }
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let debug_material = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(uv_debug_texture())),
        ..default()
    });

    let shapes = [
        // meshes.add(shape::Cube::default().into()),
        // meshes.add(shape::Box::default().into()),
        // meshes.add(shape::Capsule::default().into()),
        // meshes.add(shape::Torus::default().into()),
        // meshes.add(shape::Cylinder::default().into()),
        // meshes.add(shape::Icosphere::default().try_into().unwrap()),
        // meshes.add(shape::UVSphere::default().into()),
    ];

    let num_shapes = shapes.len();

    for (i, shape) in shapes.into_iter().enumerate() {
        commands.spawn((
            PbrBundle {
                mesh: shape,
                material: debug_material.clone(),
                transform: Transform::from_xyz(
                    -X_EXTENT / 2. + i as f32 / (num_shapes - 1) as f32 * X_EXTENT,
                    2.0,
                    0.0,
                )
                .with_rotation(Quat::from_rotation_x(-PI / 4.)),
                ..default()
            },
            Shape,
        ));
    }

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 9000.0,
            range: 1000.,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(8.0, 16.0, 8.0),
        ..default()
    });

    // ground plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(50.0).into()),
        material: materials.add(Color::SILVER.into()),
        ..default()
    });

    let parent = commands.spawn((Camera3dBundle {
        transform: Transform::from_xyz(0.0, 6., 12.0).looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
        ..default()
    },
    CameraController::default())).with_children(|parent| {
        parent.spawn(PointLightBundle {
            point_light: PointLight {
                intensity: 9000.0,
                range: 1000.,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(8.0, 16.0, 8.0),
            ..default() 
        });
    });
}

fn rotate(mut query: Query<&mut Transform, With<Shape>>, time: Res<Time>) {
    for mut transform in &mut query {
        transform.rotate_y( f32::sin(time.delta_seconds() / 4.));
        transform.rotate_x( f32::sin(time.delta_seconds() / 4.));
    }
}

#[derive(Component)]
pub struct CameraController {
    pub enabled: bool,
    pub initialized: bool,
    pub sensitivity: f32,
    pub key_forward: KeyCode,
    pub key_back: KeyCode,
    pub key_left: KeyCode,
    pub key_right: KeyCode,
    pub key_up: KeyCode,
    pub key_down: KeyCode,
    pub key_run: KeyCode,
    pub mouse_key_enable_mouse: MouseButton,
    pub keyboard_key_enable_mouse: KeyCode,
    pub walk_speed: f32,
    pub run_speed: f32,
    pub friction: f32,
    pub pitch: f32,
    pub yaw: f32,
    pub velocity: Vec3,
}

impl Default for CameraController {
    fn default() -> Self {
        Self {
            enabled: true,
            initialized: false,
            sensitivity: 0.5,
            key_forward: KeyCode::W,
            key_back: KeyCode::S,
            key_left: KeyCode::A,
            key_right: KeyCode::D,
            key_up: KeyCode::E,
            key_down: KeyCode::Q,
            key_run: KeyCode::LShift,
            mouse_key_enable_mouse: MouseButton::Left,
            keyboard_key_enable_mouse: KeyCode::M,
            walk_speed: 2.0,
            run_speed: 6.0,
            friction: 0.5,
            pitch: 0.0,
            yaw: 0.0,
            velocity: Vec3::ZERO,
        }
    }
}

pub fn camera_controller(
    time: Res<Time>,
    mut mouse_events: EventReader<MouseMotion>,
    mouse_button_input: Res<Input<MouseButton>>,
    key_input: Res<Input<KeyCode>>,
    mut move_toggled: Local<bool>,
    mut query: Query<(&mut Transform, &mut CameraController), With<Camera>>,
) {
    let dt = time.delta_seconds();

    if let Ok((mut transform, mut options)) = query.get_single_mut() {
        if !options.initialized {
            let (yaw, pitch, _roll) = transform.rotation.to_euler(EulerRot::YXZ);
            options.yaw = yaw;
            options.pitch = pitch;
            options.initialized = true;
        }
        if !options.enabled {
            return;
        }

        // Handle key input
        let mut axis_input = Vec3::ZERO;
        if key_input.pressed(options.key_forward) {
            axis_input.z += 1.0;
        }
        if key_input.pressed(options.key_back) {
            axis_input.z -= 1.0;
        }
        if key_input.pressed(options.key_right) {
            axis_input.x += 1.0;
        }
        if key_input.pressed(options.key_left) {
            axis_input.x -= 1.0;
        }
        if key_input.pressed(options.key_up) {
            axis_input.y += 1.0;
        }
        if key_input.pressed(options.key_down) {
            axis_input.y -= 1.0;
        }
        if key_input.just_pressed(options.keyboard_key_enable_mouse) {
            *move_toggled = !*move_toggled;
        }

        // Apply movement update
        if axis_input != Vec3::ZERO {
            let max_speed = if key_input.pressed(options.key_run) {
                options.run_speed
            } else {
                options.walk_speed
            };
            options.velocity = axis_input.normalize() * max_speed;
        } else {
            let friction = options.friction.clamp(0.0, 1.0);
            options.velocity *= 1.0 - friction;
            if options.velocity.length_squared() < 1e-6 {
                options.velocity = Vec3::ZERO;
            }
        }
        let forward = transform.forward();
        let right = transform.right();
        transform.translation += options.velocity.x * dt * right
            + options.velocity.y * dt * Vec3::Y
            + options.velocity.z * dt * forward;

        // Handle mouse input
        let mut mouse_delta = Vec2::ZERO;
        if mouse_button_input.pressed(options.mouse_key_enable_mouse) || *move_toggled {
            for mouse_event in mouse_events.iter() {
                mouse_delta += mouse_event.delta;
            }
        }

        if mouse_delta != Vec2::ZERO {
            // Apply look update
            options.pitch = (options.pitch - mouse_delta.y * 0.5 * options.sensitivity * dt)
                .clamp(-PI / 2., PI / 2.);
            options.yaw -= mouse_delta.x * options.sensitivity * dt;
            transform.rotation = Quat::from_euler(EulerRot::ZYX, 0.0, options.yaw, options.pitch);
        }
    }
}

/// Creates a colorful test pattern
fn uv_debug_texture() -> Image {
    const TEXTURE_SIZE: usize = 8;

    let mut palette: [u8; 32] = [
        255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 102, 255, 102, 255,
        198, 255, 102, 198, 255, 255, 121, 102, 255, 255, 236, 102, 255, 255,
    ];

    let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
    for y in 0..TEXTURE_SIZE {
        let offset = TEXTURE_SIZE * y * 4;
        texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
        palette.rotate_right(4);
    }

    Image::new_fill(
        Extent3d {
            width: TEXTURE_SIZE as u32,
            height: TEXTURE_SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &texture_data,
        TextureFormat::Rgba8UnormSrgb,
    )
}

