# bevy_pixel_camera

A simple pixel-perfect camera plugin for Bevy, suitable for pixel-art.

```no_compile
use bevy::prelude::*;
use bevy_pixel_camera::

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(PixelCameraPlugin)
        .add_startup_system(setup.system())
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    quad: Res<PixelSpriteQuad>,
) {
    commands.spawn_bundle(PixelCameraBundle::from_zoom(3));

    let sprite_handle = materials.add(asset_server.load("my-pixel-art-sprite.png").into());
    commands.spawn_bundle(SpriteBundle {
        material: sprite_handle,
        mesh: quad.clone().into(),
        ..Default::default()
    });
}
```
