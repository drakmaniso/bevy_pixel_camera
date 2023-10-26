use bevy::prelude::*;
use bevy_pixel_camera::{PixelCameraPlugin, PixelViewport, PixelZoom};

const WIDTH: i32 = 320;
const HEIGHT: i32 = 180;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(PixelCameraPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Add a camera that will always fit the virtual resolution WIDTH x HEIGHT
    // inside the window.
    commands.spawn((
        Camera2dBundle::default(),
        PixelZoom::FitSize {
            width: WIDTH,
            height: HEIGHT,
        },
        PixelViewport,
    ));

    let mire_handle = asset_server.load("mire-64x64.png");

    // Add a mire sprite in the center of the window.
    commands.spawn(SpriteBundle {
        texture: mire_handle.clone(),
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        ..Default::default()
    });

    // Add a mire sprite in the bottom-left corner of our virtual resolution.
    commands.spawn(SpriteBundle {
        texture: mire_handle.clone(),
        transform: Transform::from_translation(Vec3::new(
            -(WIDTH / 2) as f32,
            -(HEIGHT / 2) as f32,
            0.0,
        )),
        ..Default::default()
    });

    // Add a mire sprite in the bottom-right corner of our virtual resolution.
    commands.spawn(SpriteBundle {
        texture: mire_handle.clone(),
        transform: Transform::from_translation(Vec3::new(
            (WIDTH / 2) as f32,
            -(HEIGHT / 2) as f32,
            0.0,
        )),
        ..Default::default()
    });

    // Add a mire sprite in the top-left corner of our virtual resolution.
    commands.spawn(SpriteBundle {
        texture: mire_handle.clone(),
        transform: Transform::from_translation(Vec3::new(
            -(WIDTH / 2) as f32,
            (HEIGHT / 2) as f32,
            0.0,
        )),
        ..Default::default()
    });

    // Add a mire sprite in the top-right corner of our virtual resolution.
    commands.spawn(SpriteBundle {
        texture: mire_handle.clone(),
        transform: Transform::from_translation(Vec3::new(
            (WIDTH / 2) as f32,
            (HEIGHT / 2) as f32,
            0.0,
        )),
        ..Default::default()
    });
}
