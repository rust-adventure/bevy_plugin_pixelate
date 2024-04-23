use bevy::{
    prelude::*,
    render::view::screenshot::ScreenshotManager,
    window::PrimaryWindow,
};

pub fn screenshot_on_spacebar(
    input: Res<Input<KeyCode>>,
    main_window: Query<Entity, With<PrimaryWindow>>,
    mut screenshot_manager: ResMut<ScreenshotManager>,
    mut counter: Local<u32>,
    // first_pass_image: Res<FirstPassImage>,
    // images: Res<Assets<Image>>,
) {
    if input.just_pressed(KeyCode::Space) {
        let path = format!("./screenshot-{}.png", *counter);
        *counter += 1;
        screenshot_manager
            .save_screenshot_to_disk(
                main_window.single(),
                path,
            )
            .unwrap();

        // dbg!(images.iter().count());
        // for (i, (id, image)) in images.iter().enumerate() {
        //     let path = format!(
        //         "./screenshot-all-{}-{i}.png",
        //         *counter
        //     );
        //     match image.texture_descriptor.format {
        //         TextureFormat::R8Unorm
        //         | TextureFormat::Rg8Unorm
        //         | TextureFormat::Rgba8UnormSrgb
        //         | TextureFormat::Bgra8UnormSrgb => {
        //             dbg!(id);
        //             // let i =
        //             // images.get(&first_pass_image.0).unwrap();
        //             // println!("{:?}", &i.data);
        //             dbg!(image.size());
        //             let dyn_image = image
        //                 .clone()
        //                 .try_into_dynamic()
        //                 .unwrap();
        //             // dbg!(dyn_image.as_bytes());
        //             dyn_image.save(path).unwrap();
        //         }
        //         _ => {}
        //     }
        // }
    }
}
