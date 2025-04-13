use bevy::math::{uvec2, vec3};
use bevy::prelude::*;
use bevy_pixel_camera::{PixelCameraPlugin, PixelViewport, PixelZoom};

// Pixel screen dimensions
const SCREEN_DIMS: UVec2 = uvec2(320, 180);


// Main
fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.2, 0.2, 0.2)))
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource::<Msaa>(Msaa::Off)
        .add_plugins(PixelCameraPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, close_on_esc)
        .run();
}

// Setup everything
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mire_handle = asset_server.load("mire-64x64.png");

    // Add a camera that will always fit the virtual resolution WIDTH x HEIGHT
    // inside the window.
    commands.spawn((
        Camera2d,
        PixelZoom::FitSize {
            width: SCREEN_DIMS.x as i32,
            height: SCREEN_DIMS.y as i32,
        },
        PixelViewport,
    ));

    // Create Vec3 of the screen dimensions
    let v3_dims = SCREEN_DIMS.as_vec2().extend(0.0);

    // 5 sprite positions
    // 1 in the center and 4 around the edges
    let positions = [
        (0.0, 0.0),
        (-0.5, -0.5),
        (0.5, -0.5),
        (-0.5, 0.5),
        (0.5, 0.5),
    ]
    .map(|(x, y)| vec3(x, y, 0.0) * v3_dims);

    // Create a sprite entity for each position
    for pos in positions {
        commands.spawn((
            Sprite {
                image: mire_handle.clone(),
                ..Default::default()
            },
            Transform::from_translation(pos),
        ));
    }
}

// Simple system to close on ESC
// after close_on_esc was deprecated
fn close_on_esc(
    mut commands: Commands,
    focused_windows: Query<(Entity, &Window)>,
    input: Res<ButtonInput<KeyCode>>,
) {
    for (window, focus) in focused_windows.iter() {
        if focus.focused {
            if input.just_pressed(KeyCode::Escape) {
                commands.entity(window).despawn();
            }
        }
    }
}
