# bevy_pixel_camera

A simple camera plugin for the Bevy game engine, to help displaying
pixel-art sprites.

This crates provides a plugin to automatically configure Bevy's
`Camera2dBundle`. It sets the camera to an integer scaling factor (using
Bevy's `ScalingMode::WindowSize`), and automatically updates the zoom level
so that the specified target resolution fills as much of the sceen as
possible.

The plugin can also automatically set and resize the viewport of the camera
to match the target resolution.

## How to use

Note that Bevy uses linear sampling by default for textures, which is not
what you want for pixel art. The easiest way to change this is to configure
Bevy's `ImagePlugin`:

```rust
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        ...
```

Also note that if either the width or the height of your sprite is not
divisible by 2, you may need to change the anchor of the sprite (which is at
the center by default), if you want it to be aligned with virtual pixels.

A small example is included in the crate. Run it with:

```console
cargo run --example flappin
```

## Comparison with other methods

There is two main methods to render pixel-art games: upscale each sprite
independently, or render everything to an offscreen texture and only upscale
this texture. This crate use the first method. There is advantages and
drawbacks to both approaches.

Advantages of the "upscale each sprite independently" method:

- allows for smoother scrolling and movement of sprites, if you're willing
  to temporarily break the alignment on virtual pixels (this would be even
  more effective with a dedicated upscaling shader);
- easier to mix pixel-art and high resolution graphics (for example for
  text, particles or effects).

Advantages of the "offscreen texture" method:

- always ensure perfect alignment on virtual pixels (authentic "retro"
  look);
- probably more efficient (in most cases, the difference is probably
  negligible on modern computers).

## Example code

```rust
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_pixel_camera::{
    PixelCameraPlugin, PixelZoom, PixelViewport
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(PixelCameraPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Camera2dBundle::default(),
        PixelZoom::FitSize {
            width: 320,
            height: 180,
        },
        PixelViewport,
    ));

    commands.spawn(SpriteBundle {
        texture: asset_server.load("my-pixel-art-sprite.png"),
        sprite: Sprite {
            anchor: Anchor::BottomLeft,
            ..Default::default()
        },
        ..Default::default()
    });
}
```

## Bevy versions supported

| bevy | bevy_pixel_camera |
|------|-------------------|
| 0.11 | 0.5.2             |
| 0.10 | 0.4.1             |
| 0.9  | 0.3               |
| 0.8  | 0.2               |

### Migration guide: 0.4 to 0.5 (Bevy 0.10 to 0.11)

The `PixelBorderPlugin` has been deprecated. If you want a border around
your virtual resolution, pass `true` to the `set_viewport` argument when
creating the camera bundle (see example above).

### Migration guide: 0.5 to 0.5.2

The `PixelCameraBundle` has been deprecated. Replace it with a standard
`Camera2dBundle`, to which you add the `PixelZoom` and `PixelViewport`
components.

## License

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
  <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or
  <http://opensource.org/licenses/MIT>)

at your option.

License: MIT OR Apache-2.0
