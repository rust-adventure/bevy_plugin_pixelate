> [!IMPORTANT]  
> This is currently just a demo, and is using an old (0.12) version of Bevy! Take a look if you're interested, but don't expect to be able to install and use it.

# bevy_pixelate_3d

Here are some examples of what the code does

![screenshot all-meshes](readme/screenshot-all-meshes.avif)

![screenshot lil-car](readme/screenshot-lil-car.avif)

![screenshot scene](readme/screenshot-scene.avif)

```rust
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
                // hdr: not required, but fun!
                hdr: true,
                ..default()
            },
            transform: Transform::from_translation(
                Vec3::new(0.0, 10.0, 15.0),
            )
            .looking_at(Vec3::new(0., 4., 0.), Vec3::Y),
            // I like TonyMcMapface
            tonemapping: bevy::core_pipeline::tonemapping::Tonemapping::TonyMcMapface,
            // Boosting the saturation can be interesting, but is not required
            color_grading: ColorGrading {
                post_saturation: 1.8,
                ..default()
            },
            projection: Projection::Orthographic(OrthographicProjection{
                scale: 0.1,
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
```

```rust
mut pixelated: ResMut<
    Assets<
        ExtendedMaterial<
            StandardMaterial,
            PixelatedExtension,
        >,
    >,
>,
```

## Credits

- `assets/taxi` is from Kenney's [car-kit](https://kenney.nl/assets/car-kit) (Creative Commons CC0)
- `assets/parallax` is from [the Bevy repo](https://github.com/bevyengine/bevy/tree/c593ee1055047a64501efa5de5885c9d85547af3/assets/textures/parallax_example) and used in the Bevy parallax example
- oklab functions are mentioned inline in the wgsl shader
