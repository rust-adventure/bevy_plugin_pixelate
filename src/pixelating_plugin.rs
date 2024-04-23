use crate::PixelatedExtension;
use bevy::{
    pbr::{
        ExtendedMaterial, NotShadowCaster,
        NotShadowReceiver,
    },
    prelude::*,
    render::{
        camera::RenderTarget,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension,
            TextureFormat, TextureUsages,
        },
        texture::ImageSampler,
        view::RenderLayers,
    },
};

/// add PixelatedCamera to your 3d camera to
/// use it as the source of the pixelated effect
#[derive(Component)]
pub struct PixelatedCamera;

pub struct PixelatingPlugin;

impl Plugin for PixelatingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<
            ExtendedMaterial<
                StandardMaterial,
                PixelatedExtension,
            >,
        > {
            prepass_enabled: true,
            ..default()
        })
        .add_systems(Startup, setup)
        .add_systems(Update, configure_pixelated_camera);
    }
}

// for saving screenshots of image
#[derive(Resource)]
struct FirstPassImage(Handle<Image>);

#[derive(Resource, Deref)]
pub struct PixelatedPassLayer(pub RenderLayers);

// Marks the main pass cube, to which the texture is applied.
#[derive(Component)]
struct MainPassDisplay;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    let size = Extent3d {
        width: 512,
        height: 288,
        ..default()
    };

    // This is the texture that will be rendered to.
    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        sampler: ImageSampler::nearest(),
        ..default()
    };

    // fill image.data with zeroes
    image.resize(size);

    let image_handle = images.add(image);
    commands.insert_resource(FirstPassImage(
        image_handle.clone(),
    ));

    // This specifies the layer used for the first pass, which will be attached to the first pass camera and cube.
    let pixelated_pass_layer = RenderLayers::layer(1);
    commands.insert_resource(PixelatedPassLayer(
        pixelated_pass_layer,
    ));

    // Display the pixelated image we generated with the first camera
    // it is likely that not only the size, but the approach used here
    // should change.
    // ex: doesn't really need to be a PbrBundle.
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::new(
                Vec2::new(16., 9.) * 1.5,
            ))),
            material: materials.add(StandardMaterial {
                base_color_texture: Some(image_handle),
                unlit: true,
                ..default()
            }),
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        },
        NotShadowCaster,
        NotShadowReceiver,
        MainPassDisplay,
    ));

    // The main pass camera.
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 15.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        tonemapping: bevy::core_pipeline::tonemapping::Tonemapping::TonyMcMapface,
        ..default()
    });
}

// Turns any user-supplied camera (labelled with `PixelatedCamera`)
// into the camera that renders to the pixelated texture
fn configure_pixelated_camera(
    mut commands: Commands,
    mut cameras: Query<
        (Entity, &mut Camera),
        Added<PixelatedCamera>,
    >,
    image: Res<FirstPassImage>,
    pixelated_pass_layer: Res<PixelatedPassLayer>,
) {
    for (entity, mut camera) in &mut cameras {
        camera.order = -1;
        camera.target =
            RenderTarget::Image(image.0.clone());
        commands
            .entity(entity)
            .insert(pixelated_pass_layer.0);
    }
}
