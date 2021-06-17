use bevy::prelude::*;
use bevy_pixel_camera::{PixelCameraBundle, PixelCameraPlugin, PixelSpriteQuad};

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(PixelCameraPlugin)
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_startup_system(setup.system())
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    quad: Res<PixelSpriteQuad>,
) {
    // Add a camera that will always fit the virtual resolution 320x180 inside
    // the window.
    commands.spawn_bundle(PixelCameraBundle::from_resolution(320, 180));

    let mire_32x32_handle = materials.add(asset_server.load("mire-32x32.png").into());
    let mire_31x31_handle = materials.add(asset_server.load("mire-31x31.png").into());

    // Add a 31x31 sprite in the center of the window.
    commands.spawn_bundle(SpriteBundle {
        material: mire_31x31_handle.clone(),
        mesh: quad.clone().into(),
        transform: Transform::from_translation(Vec3::new(-16.0, -16.0, 0.0)),
        ..Default::default()
    });

    // Add a 32x32 sprite in the bottom-left corner of our virtual resolution.
    commands.spawn_bundle(SpriteBundle {
        material: mire_32x32_handle.clone(),
        mesh: quad.clone().into(),
        transform: Transform::from_translation(Vec3::new(-160.0, -90.0, 0.0)),
        ..Default::default()
    });

    // Add a 32x32 sprite in the bottom-right corner of our virtual resolution.
    commands.spawn_bundle(SpriteBundle {
        material: mire_32x32_handle.clone(),
        mesh: quad.clone().into(),
        transform: Transform::from_translation(Vec3::new(160.0 - 32.0, -90.0, 0.0)),
        ..Default::default()
    });

    // Add a 32x32 sprite in the top-left corner of our virtual resolution.
    commands.spawn_bundle(SpriteBundle {
        material: mire_32x32_handle.clone(),
        mesh: quad.clone().into(),
        transform: Transform::from_translation(Vec3::new(-160.0, 90.0 - 32.0, 0.0)),
        ..Default::default()
    });

    // Add a 32x32 sprite in the top-right corner of our virtual resolution.
    commands.spawn_bundle(SpriteBundle {
        material: mire_32x32_handle.clone(),
        mesh: quad.clone().into(),
        transform: Transform::from_translation(Vec3::new(160.0 - 32.0, 90.0 - 32.0, 0.0)),
        ..Default::default()
    });
}
