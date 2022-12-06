use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_pixel_camera::{PixelCameraBundle, PixelCameraPlugin};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.000001, 0.000001, 0.000001)))
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(PixelCameraPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Add a camera that will always fit the virtual resolution 320x180 inside
    // the window.
    commands.spawn(PixelCameraBundle::from_resolution(320, 180));
    // commands.spawn(OrthographicCameraBundle::new_2d());

    let mire_32x32_handle = asset_server.load("mire-32x32.png");
    // let mire_31x31_handle = asset_server.load("mire-31x31.png");

    // Add a 31x31 sprite in the center of the window.
    commands.spawn(SpriteBundle {
        texture: asset_server.load("mire-31x31.png"),
        sprite: Sprite {
            anchor: Anchor::BottomLeft,
            ..Default::default()
        },
        transform: Transform::from_translation(Vec3::new(-16.0, -16.0, 0.0)),
        ..Default::default()
    });

    // Add a 32x32 sprite in the bottom-left corner of our virtual resolution.
    commands.spawn(SpriteBundle {
        texture: asset_server.load("mire-32x32.png"),
        sprite: Sprite {
            anchor: Anchor::BottomLeft,
            ..Default::default()
        },
        transform: Transform::from_translation(Vec3::new(-160.0, -90.0, 0.0)),
        ..Default::default()
    });

    // Add a 32x32 sprite in the bottom-right corner of our virtual resolution.
    commands.spawn(SpriteBundle {
        texture: mire_32x32_handle.clone(),
        sprite: Sprite {
            anchor: Anchor::BottomLeft,
            ..Default::default()
        },
        transform: Transform::from_translation(Vec3::new(160.0 - 32.0, -90.0, 0.0)),
        ..Default::default()
    });

    // Add a 32x32 sprite in the top-left corner of our virtual resolution.
    commands.spawn(SpriteBundle {
        texture: mire_32x32_handle.clone(),
        sprite: Sprite {
            anchor: Anchor::BottomLeft,
            ..Default::default()
        },
        transform: Transform::from_translation(Vec3::new(-160.0, 90.0 - 32.0, 0.0)),
        ..Default::default()
    });

    // Add a 32x32 sprite in the top-right corner of our virtual resolution.
    commands.spawn(SpriteBundle {
        texture: mire_32x32_handle.clone(),
        sprite: Sprite {
            anchor: Anchor::BottomLeft,
            ..Default::default()
        },
        transform: Transform::from_translation(Vec3::new(160.0 - 32.0, 90.0 - 32.0, 0.0)),
        ..Default::default()
    });
}
