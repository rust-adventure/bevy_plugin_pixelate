use bevy::{
    pbr::ExtendedMaterial,
    prelude::*,
    render::{render_resource::{
        Extent3d, TextureDimension, TextureFormat,
    }, view::ColorGrading}, core_pipeline::{clear_color::ClearColorConfig, prepass::{DepthPrepass, NormalPrepass}},
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use gen_04_pixels::{
    colors,
    pixelating_plugin::{
        PixelatedCamera, PixelatedPassLayer,
        PixelatingPlugin,
    },
    rotators::{
        circle_rotator_system, rotator_system, light_rotator_system,
    },
    screenshots::screenshot_on_spacebar,
    PixelatedExtension,
};
use std::f32::consts::PI;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest()),
            WorldInspectorPlugin::new(),
        ))
        .add_plugins(PixelatingPlugin)
        .add_systems(PostStartup, setup)
        .add_systems(
            Update,
            (
                circle_rotator_system,
                light_rotator_system,
                rotator_system,
                screenshot_on_spacebar,
                rotate,
            ),
        )
        .insert_resource(Msaa::Off)
        .run();
}

#[derive(Component)]
struct Shape;

const X_EXTENT: f32 = 14.5;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    // mut materials: ResMut<Assets<StandardMaterial>>,
    pixelated_pass_layer: Res<PixelatedPassLayer>,
    mut pixelated: ResMut<
        Assets<
            ExtendedMaterial<
                StandardMaterial,
                PixelatedExtension,
            >,
        >,
    >,
) {
    // let debug_material = materials.add(StandardMaterial {
    //     base_color_texture: Some(
    //         images.add(uv_debug_texture()),
    //     ),
    //     ..default()
    // });

    let debug_material = pixelated.add(ExtendedMaterial {
        base: StandardMaterial {
            // base_color: colors::RED,
            base_color_texture: Some(
                images.add(uv_debug_texture()),
            ),
            perceptual_roughness: 1.0,
            ..Default::default()
        },
        extension: PixelatedExtension {
            quantize_steps: 5,
        },
    });
    let shapes = [
        meshes.add(shape::Cube::default().into()),
        meshes.add(shape::Box::default().into()),
        meshes.add(shape::Capsule::default().into()),
        meshes.add(shape::Torus::default().into()),
        meshes.add(shape::Cylinder::default().into()),
        meshes.add(
            shape::Icosphere::default().try_into().unwrap(),
        ),
        meshes.add(shape::UVSphere::default().into()),
    ];

    let num_shapes = shapes.len();

    for (i, shape) in shapes.into_iter().enumerate() {
        commands.spawn((
            MaterialMeshBundle {
                mesh: shape,
                material: debug_material.clone(),
                transform: Transform::from_xyz(
                    -X_EXTENT / 2.
                        + i as f32
                            / (num_shapes - 1) as f32
                            * X_EXTENT,
                    2.0,
                    0.0,
                )
                .with_rotation(
                    Quat::from_rotation_x(-PI / 4.),
                ),
                ..default()
            },
            Shape,
            pixelated_pass_layer.0
        ));
    }

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 4500.0,
            range: 100.,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(8.0, 16.0, 8.0),
        ..default()
    });

    // ground plane
    commands.spawn((MaterialMeshBundle {
        mesh: meshes
            .add(shape::Plane::from_size(50.0).into()),
        material: pixelated.add(ExtendedMaterial {
            base: StandardMaterial {
                base_color: colors::BASE,
                perceptual_roughness: 1.0,
                ..Default::default()
            },
            extension: PixelatedExtension {
                quantize_steps: 15,
            },
        }),
        ..default()
    }, pixelated_pass_layer.0));

    commands.spawn((
        Camera3dBundle {
            camera_3d: Camera3d {
                clear_color: ClearColorConfig::Custom(
                    colors::SKY,
                ),
                ..default()
            }, 
            camera: Camera {
                hdr: true,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 6., 12.0)
            .looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
            tonemapping: bevy::core_pipeline::tonemapping::Tonemapping::TonyMcMapface,
            color_grading: ColorGrading {
                post_saturation: 1.2,
                ..default()
            },
            ..default()
        },
        // depth prepass is required for pixelated.wgsl       
        DepthPrepass,
        // normal prepass is required for pixelated.wgsl
        NormalPrepass,
        // PixelatedCamera causes this camera to be used to generate the
        // pixelated scene
        PixelatedCamera,
    ));
}

fn rotate(
    mut query: Query<&mut Transform, With<Shape>>,
    time: Res<Time>,
) {
    for mut transform in &mut query {
        transform.rotate_y(time.delta_seconds() / 2.);
    }
}

/// Creates a colorful test pattern
fn uv_debug_texture() -> Image {
    const TEXTURE_SIZE: usize = 8;

    let mut palette: [u8; 32] = [
        255, 102, 159, 255, 255, 159, 102, 255, 236, 255,
        102, 255, 121, 255, 102, 255, 102, 255, 198, 255,
        102, 198, 255, 255, 121, 102, 255, 255, 236, 102,
        255, 255,
    ];

    let mut texture_data =
        [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
    for y in 0..TEXTURE_SIZE {
        let offset = TEXTURE_SIZE * y * 4;
        texture_data[offset..(offset + TEXTURE_SIZE * 4)]
            .copy_from_slice(&palette);
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

