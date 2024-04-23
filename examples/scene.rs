use bevy::{
    core_pipeline::{
        clear_color::ClearColorConfig,
        prepass::{DepthPrepass, NormalPrepass},
    },
    pbr::{
        ExtendedMaterial, NotShadowCaster,
        NotShadowReceiver, OpaqueRendererMethod,
    },
    prelude::*,
    render::view::ColorGrading,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use gen_04_pixels::{
    colors,
    pixelating_plugin::{
        PixelatedCamera,
        PixelatedPassLayer, PixelatingPlugin,
    },
    rotators::{
        circle_rotator_system, rotator_system, Rotate, light_rotator_system,
    },
    screenshots::screenshot_on_spacebar,
    PixelatedExtension,
};
use std::f32::consts::{FRAC_PI_4, FRAC_PI_8, PI};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest()),
            WorldInspectorPlugin::new(),
        ))
        .add_plugins(PixelatingPlugin)
        .add_systems(
            PostStartup,
            (setup_camera, setup_scene),
        )
        .add_systems(
            Update,
            (
                circle_rotator_system,
                light_rotator_system,
                rotator_system,
                screenshot_on_spacebar,
            ),
        )
        .insert_resource(Msaa::Off)
        .run();
}

fn setup_camera(
    mut commands: Commands,
) {
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
            transform: Transform::from_translation(
                Vec3::new(0.0, 10.0, 15.0),
            )
            .looking_at(Vec3::new(0., 4., 0.), Vec3::Y),
            tonemapping: bevy::core_pipeline::tonemapping::Tonemapping::TonyMcMapface,
            color_grading: ColorGrading {
                post_saturation: 1.8,
                ..default()
            },
            projection: Projection::Orthographic(OrthographicProjection{
                // near: todo!(),
                // far: todo!(),
                // viewport_origin: todo!(),
                // scaling_mode: todo!(),
                scale: 0.1,
                // area: todo!()
                ..default()
            }),
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

// setup is just responsible for the scene setup
// all camera setup, etc is done by the plugin
fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut pixelated: ResMut<
        Assets<
            ExtendedMaterial<
                StandardMaterial,
                PixelatedExtension,
            >,
        >,
    >,
    asset_server: Res<AssetServer>,
    pixelated_pass_layer: Res<PixelatedPassLayer>,
) {
    // cubes
    commands.spawn((
        MaterialMeshBundle {
            mesh: meshes
                .add(Mesh::from(shape::Cube { size: 1.0 })),
            transform: Transform::from_xyz(6.0, 4., -20.0),
            material: pixelated.add(ExtendedMaterial {
                base: StandardMaterial {
                    base_color: colors::RED,
                    // can be used in forward or deferred mode.
                    opaque_render_method:
                        OpaqueRendererMethod::Auto,
                    // in deferred mode, only the PbrInput can be modified (uvs, color and other material properties),
                    // in forward mode, the output can also be modified after lighting is applied.
                    // see the fragment shader `extended_material.wgsl` for more info.
                    // Note: to run in deferred mode, you must also add a `DeferredPrepass` component to the camera and either
                    // change the above to `OpaqueRendererMethod::Deferred` or add the `DefaultOpaqueRendererMethod` resource.
                    perceptual_roughness: 1.0,
                    ..Default::default()
                },
                extension: PixelatedExtension {
                    quantize_steps: 5,
                },
            }),
            ..default()
        },
        Rotate,
        pixelated_pass_layer.0,
    ));
    commands.spawn((
        MaterialMeshBundle {
            mesh: meshes
                .add(Mesh::from(shape::Cube { size: 2.0 })),
            transform: Transform::from_xyz(0.0, 0., 0.0),
            material: pixelated.add(ExtendedMaterial {
                base: StandardMaterial {
                    base_color: colors::GREEN,
                    // can be used in forward or deferred mode.
                    opaque_render_method:
                        OpaqueRendererMethod::Auto,
                    // in deferred mode, only the PbrInput can be modified (uvs, color and other material properties),
                    // in forward mode, the output can also be modified after lighting is applied.
                    // see the fragment shader `extended_material.wgsl` for more info.
                    // Note: to run in deferred mode, you must also add a `DeferredPrepass` component to the camera and either
                    // change the above to `OpaqueRendererMethod::Deferred` or add the `DefaultOpaqueRendererMethod` resource.
                    perceptual_roughness: 1.0,
                    ..Default::default()
                },
                extension: PixelatedExtension {
                    quantize_steps: 15,
                },
            }),
            ..default()
        },
        Rotate,
        pixelated_pass_layer.0,
    ));
    commands.spawn((
        MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(shape::Torus {
                radius: 4.,
                ring_radius: 2.,
                // subdivisions_segments: todo!(),
                // subdivisions_sides: todo!(),
                ..default()
            })),
            transform: Transform::from_xyz(0.0, 0., 0.0),
            material: pixelated.add(ExtendedMaterial {
                base: StandardMaterial {
                    base_color: colors::GREEN,
                    // can be used in forward or deferred mode.
                    opaque_render_method:
                        OpaqueRendererMethod::Auto,
                    // in deferred mode, only the PbrInput can be modified (uvs, color and other material properties),
                    // in forward mode, the output can also be modified after lighting is applied.
                    // see the fragment shader `extended_material.wgsl` for more info.
                    // Note: to run in deferred mode, you must also add a `DeferredPrepass` component to the camera and either
                    // change the above to `OpaqueRendererMethod::Deferred` or add the `DefaultOpaqueRendererMethod` resource.
                    perceptual_roughness: 1.0,
                    ..Default::default()
                },
                extension: PixelatedExtension {
                    quantize_steps: 15,
                },
            }),
            ..default()
        },
        Rotate,
        pixelated_pass_layer.0,
    ));
    commands.spawn((
        MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(shape::Cylinder {
                radius: 2.,
                height: 4.,
                ..default()
            })),
            transform: Transform::from_xyz(-15.0, 2., 0.0),
            material: pixelated.add(ExtendedMaterial {
                base: StandardMaterial {
                    base_color: colors::SAPPHIRE,
                    // can be used in forward or deferred mode.
                    opaque_render_method:
                        OpaqueRendererMethod::Auto,
                    // in deferred mode, only the PbrInput can be modified (uvs, color and other material properties),
                    // in forward mode, the output can also be modified after lighting is applied.
                    // see the fragment shader `extended_material.wgsl` for more info.
                    // Note: to run in deferred mode, you must also add a `DeferredPrepass` component to the camera and either
                    // change the above to `OpaqueRendererMethod::Deferred` or add the `DefaultOpaqueRendererMethod` resource.
                    perceptual_roughness: 1.0,
                    emissive: colors::SAPPHIRE,
                    ..Default::default()
                },
                extension: PixelatedExtension {
                    quantize_steps: 15,
                },
            }),
            ..default()
        },
        // Rotate,
        pixelated_pass_layer.0,
    ));
    commands.spawn((
        MaterialMeshBundle {
            mesh: meshes
                .add(Mesh::from(shape::Cube { size: 2.0 })),
            transform: Transform::from_xyz(5.0, 4., -5.0),
            material: pixelated.add(ExtendedMaterial {
                base: StandardMaterial {
                    base_color: colors::RED,
                    // can be used in forward or deferred mode.
                    opaque_render_method:
                        OpaqueRendererMethod::Auto,
                    // in deferred mode, only the PbrInput can be modified (uvs, color and other material properties),
                    // in forward mode, the output can also be modified after lighting is applied.
                    // see the fragment shader `extended_material.wgsl` for more info.
                    // Note: to run in deferred mode, you must also add a `DeferredPrepass` component to the camera and either
                    // change the above to `OpaqueRendererMethod::Deferred` or add the `DefaultOpaqueRendererMethod` resource.
                    perceptual_roughness: 1.0,
                    ..Default::default()
                },
                extension: PixelatedExtension {
                    quantize_steps: 15,
                },
            }),
            ..default()
        },
        Rotate,
        pixelated_pass_layer.0,
    ));

    for i in 0..10 {
        commands.spawn((
            MaterialMeshBundle {
                mesh: meshes.add(Mesh::from(shape::Cube {
                    size: 2.0,
                })),
                transform: Transform::from_xyz(
                    -8.0,
                    2. * i as f32 + 0.5,
                    -4.0,
                )
                .with_rotation(
                    Quat::from_rotation_y(
                        i as f32 * FRAC_PI_8,
                    ),
                ),
                material: pixelated.add(ExtendedMaterial {
                    base: StandardMaterial {
                        base_color: colors::LAVENDER,
                        // can be used in forward or deferred mode.
                        opaque_render_method:
                            OpaqueRendererMethod::Auto,
                        // in deferred mode, only the PbrInput can be modified (uvs, color and other material properties),
                        // in forward mode, the output can also be modified after lighting is applied.
                        // see the fragment shader `extended_material.wgsl` for more info.
                        // Note: to run in deferred mode, you must also add a `DeferredPrepass` component to the camera and either
                        // change the above to `OpaqueRendererMethod::Deferred` or add the `DefaultOpaqueRendererMethod` resource.
                        perceptual_roughness: 1.0,
                        ..Default::default()
                    },
                    extension: PixelatedExtension {
                        quantize_steps: 15,
                    },
                }),
                ..default()
            },
            // Rotate,
            pixelated_pass_layer.0,
        ));
    }
    commands.spawn((
        MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(shape::UVSphere {
                radius: 1.,
                ..default()
            })),
            transform: Transform::from_xyz(6.0, 4., 0.0),
            material: pixelated.add(ExtendedMaterial {
                base: StandardMaterial {
                    base_color: colors::RED,
                    // can be used in forward or deferred mode.
                    opaque_render_method:
                        OpaqueRendererMethod::Auto,
                    // in deferred mode, only the PbrInput can be modified (uvs, color and other material properties),
                    // in forward mode, the output can also be modified after lighting is applied.
                    // see the fragment shader `extended_material.wgsl` for more info.
                    // Note: to run in deferred mode, you must also add a `DeferredPrepass` component to the camera and either
                    // change the above to `OpaqueRendererMethod::Deferred` or add the `DefaultOpaqueRendererMethod` resource.
                    perceptual_roughness: 1.0,
                    ..Default::default()
                },
                extension: PixelatedExtension {
                    quantize_steps: 5,
                },
            }),
            ..default()
        },
        Rotate,
        pixelated_pass_layer.0,
    ));

    commands.spawn((
        MaterialMeshBundle {
            mesh: meshes.add(
                Mesh::try_from(shape::Plane {
                    size: 30.,
                    subdivisions: 1,
                })
                .unwrap(),
            ),
            transform: Transform::from_xyz(0.0, 0.0, 0.0)
                .with_rotation(Quat::from_rotation_y(
                    FRAC_PI_4,
                )),
            material: pixelated.add(ExtendedMaterial {
                base: StandardMaterial {
                    base_color: colors::BASE,
                    // can be used in forward or deferred mode.
                    opaque_render_method:
                        OpaqueRendererMethod::Auto,
                    // in deferred mode, only the PbrInput can be modified (uvs, color and other material properties),
                    // in forward mode, the output can also be modified after lighting is applied.
                    // see the fragment shader `extended_material.wgsl` for more info.
                    // Note: to run in deferred mode, you must also add a `DeferredPrepass` component to the camera and either
                    // change the above to `OpaqueRendererMethod::Deferred` or add the `DefaultOpaqueRendererMethod` resource.
                    perceptual_roughness: 1.0,
                    ..Default::default()
                },
                extension: PixelatedExtension {
                    quantize_steps: 15,
                },
            }),
            // material: materials.add(StandardMaterial {
            //     base_color: colors::FLAMINGO,
            //     // can be used in forward or deferred mode.
            //     opaque_render_method:
            //         OpaqueRendererMethod::Auto,
            //     // in deferred mode, only the PbrInput can be modified (uvs, color and other material properties),
            //     // in forward mode, the output can also be modified after lighting is applied.
            //     // see the fragment shader `extended_material.wgsl` for more info.
            //     // Note: to run in deferred mode, you must also add a `DeferredPrepass` component to the camera and either
            //     // change the above to `OpaqueRendererMethod::Deferred` or add the `DefaultOpaqueRendererMethod` resource.
            //     perceptual_roughness: 1.0,
            //     ..Default::default()
            // }),
            ..default()
        },
        // ShadowR
        pixelated_pass_layer.0,
    ));

    let parallax_material =
        pixelated.add(ExtendedMaterial {
            base: StandardMaterial {
                perceptual_roughness: 0.4,
                base_color: colors::LAVENDER,
                // base_color_texture: Some(
                //     asset_server
                //         .load("parallax/cube_color.png"),
                // ),
                normal_map_texture: Some(
                    asset_server
                        .load("parallax/cube_normal.png"),
                ),
                // The depth map is a greyscale texture where black is the highest level and
                // white the lowest.
                depth_map: Some(
                    asset_server
                        .load("parallax/cube_depth.png"),
                ),
                parallax_depth_scale: 0.09,
                parallax_mapping_method:
                    ParallaxMappingMethod::Occlusion,
                max_parallax_layer_count: 5.0_f32.exp2(),
                ..default()
            },
            extension: PixelatedExtension {
                quantize_steps: 15,
            },
        });
    commands.spawn((
        MaterialMeshBundle {
            transform: Transform::from_xyz(0.0, 4.0, -10.0),
            mesh: meshes.add(
                // NOTE: for normal maps and depth maps to work, the mesh
                // needs tangents generated.
                Mesh::from(shape::Cube { size: 4.0 })
                    .with_generated_tangents()
                    .unwrap(),
            ),
            material: parallax_material.clone(),
            ..default()
        },
        pixelated_pass_layer.0,
        Rotate,
    ));

    // Light
    // NOTE: Currently lights are shared between passes - see https://github.com/bevyengine/bevy/issues/3462
    for i in 0..10 {
        let transform = Transform::from_xyz(
            i as f32 * 10.0,
            4.0,
            (i as f32 * 3.) - 15.,
        );
        let light_color = Color::Lcha {
            lightness: 1.,
            chroma: 1.,
            hue: 360. / 10. * i as f32,
            alpha: 1.,
        };
        commands
            .spawn(PointLightBundle {
                transform,
                point_light: PointLight {
                    // intensity: (),
                    // range: (),
                    color: light_color,
                    shadows_enabled: true,
                    ..default()
                },
                ..default()
            })
            .with_children(|parent| {
                parent.spawn((
                    MaterialMeshBundle {
                        mesh: meshes.add(
                            Mesh::try_from(
                                shape::UVSphere {
                                    radius: 0.5,
                                    ..default()
                                },
                            )
                            .unwrap(),
                        ),

                        material: materials.add(
                            StandardMaterial {
                                base_color: light_color,
                                unlit: true,
                                ..Default::default()
                            },
                        ),
                        ..default()
                    },
                    NotShadowCaster,
                    NotShadowReceiver,
                    pixelated_pass_layer.0,
                ));
            });
    }

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.2,
    });
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 10000.,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 20.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.)
                + Quat::from_rotation_z(-PI),
            ..default()
        },
        // The default cascade config is designed to handle large scenes.
        // As this example has a much smaller world, we can tighten the shadow
        // bounds for better visual quality.
        // cascade_shadow_config: CascadeShadowConfigBuilder {
        //     first_cascade_far_bound: 4.0,
        //     maximum_distance: 1000.0,
        //     ..default()
        // }
        // .into(),
        ..default()
    });
}
