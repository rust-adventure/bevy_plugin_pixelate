use bevy::{
    core_pipeline::{
        clear_color::ClearColorConfig,
        prepass::{DepthPrepass, NormalPrepass},
    },
    pbr::{
        ExtendedMaterial, NotShadowCaster,
        NotShadowReceiver,
    },
    prelude::*,
    render::view::ColorGrading, gltf::GltfNode,
};
use bevy_asset_loader::{asset_collection::AssetCollection, loading_state::{LoadingStateAppExt, LoadingState, config::ConfigureLoadingState}};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_scene_hook::{HookedSceneBundle,HookPlugin, SceneHook};
use gen_04_pixels::{
    colors,
    pixelating_plugin::{
        PixelatedCamera,
        PixelatedPassLayer, PixelatingPlugin,
    },
    rotators::{
        circle_rotator_system, rotator_system, Rotate, light_rotator_system, CircleRotate,
    },
    screenshots::screenshot_on_spacebar,
    PixelatedExtension,
};
use std::f32::consts::{FRAC_PI_4, PI};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest()),
            WorldInspectorPlugin::new(),
            HookPlugin
        ))
        .add_plugins(PixelatingPlugin)
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
        .add_state::<MyStates>()
        .add_loading_state(
            LoadingState::new(MyStates::AssetLoading)
                .continue_to_state(MyStates::Next)
                .load_collection::<CarAssets>(),
        )
        .add_systems(OnEnter(MyStates::Next), (setup_camera, setup_scene, setup_lights))
        .run();
}

#[derive(AssetCollection, Resource)]
struct CarAssets {
    #[asset(path = "car-kit/taxi.glb#Node0")]
    taxi: Handle<GltfNode>,
    #[asset(path = "car-kit/taxi.glb#Scene0")]
    taxi_scene: Handle<Scene>,
}



#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum MyStates {
    #[default]
    AssetLoading,
    Next,
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
    materials: Res<Assets<StandardMaterial>>,
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
    nodes: Res<Assets<GltfNode>>,
    cars: Res<CarAssets>
) {
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
                    perceptual_roughness: 1.0,
                    ..Default::default()
                },
                extension: PixelatedExtension {
                    quantize_steps: 15,
                },
            }),
            ..default()
        },
        pixelated_pass_layer.0,
    ));
    // cubes
    commands.spawn((
        MaterialMeshBundle {
            mesh: meshes
                .add(Mesh::from(shape::Cube { size: 1.0 })),
            transform: Transform::from_xyz(6.0, 4., -20.0),
            material: pixelated.add(ExtendedMaterial {
                base: StandardMaterial {
                    base_color: colors::RED,
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
    // commands.spawn((SceneBundle {
    //     scene: cars.taxi_scene.clone(),
    //     transform: Transform::from_xyz(0.,-4.,0.,)
    //         .with_rotation(Quat::from_rotation_y(
    //             -FRAC_PI_4,
    //     )),
    //     ..default()
    // }, pixelated_pass_layer.0));
    let layer = pixelated_pass_layer.0.clone();
   
    commands.spawn((HookedSceneBundle {
        scene: SceneBundle {
            scene: cars.taxi_scene.clone(),
            transform: Transform::from_xyz(0.,0.,0.,)
                .with_rotation(Quat::from_rotation_y(
                    -FRAC_PI_4,
                )),
            ..default()
        },
        hook: SceneHook::new(move |entity, cmds| {
            // only operate on entities with a `Handle<StandardMaterial>`.
            let Some(_) = entity.get::<Handle<StandardMaterial>>() else {
                return;
            };
            cmds.insert(layer);
            // cmds.insert();
            // materials;
            // let std_material = mats.get(mat).expect("we should already have checked to see if there is a standardmaterial on this entity");
            // mats;
            
            // match entity.get::<Name>().map(|t|t.as_str()) {
            //     // Some("Pile") => cmds.insert(Pile(PileType::Drawing)),
            //     // Some("Card") => cmds.insert(Card),
            //     name => {
            //         dbg!(name);
                    
            //         cmds
            //     }
            // };
        }),
    },
    CircleRotate
));
// let taxi: Handle<GltfNode> = asset_server.load("car-kit/taxi.glb#Node0");
// let taxi = nodes.get(&cars.taxi).expect("a taxi");
// dbg!(&taxi);
    // commands.spawn(MaterialMeshBundle {
    //     mesh: asset_server.load("car-kit/taxi.glb#Mesh1"),
    //     transform: Transform::from_xyz(0.,0.,0.,)
    //         .with_rotation(Quat::from_rotation_y(
    //             FRAC_PI_4,
    //     )),
    //     material: materials.add( StandardMaterial {
    //             base_color: colors::RED,
    //             perceptual_roughness: 1.0,
    //             ..Default::default()
    //         },
    //     ),
    //     ..default()
    // });
}

fn setup_lights(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    pixelated_pass_layer: Res<PixelatedPassLayer>,
) {

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

