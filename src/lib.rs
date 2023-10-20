//! A simple pixel-perfect camera plugin for Bevy, suitable for pixel-art.
//!
//! This crates makes it possible to correctly display low-resolution 2d
//! graphics with Bevy's default renderer and sprites.
//!
//! While it is possible to achieve pixel-perfect rendering with Bevy's own
//! camera and `OrthographicProjection`, doing so correctly requires to
//! configure it in a specific way (you can't just use
//! `OrhtographicCameraBundle`).
//!
//! This plugin provides a camera which can be easily configured by specifying
//! either the size of the virtual pixels, or the desired resolution.
//!
//! Note that if either the width or the height of your sprite is not divisible
//! by 2, you need to change the anchor of the sprite (which is at the center by
//! default), or it will not be pixel aligned.
//!
//! Also note that Bevy uses linear sampling by default for textures, which is
//! not what you want for pixel art. The easiest way to change this is to set
//! the default_sampler on the ImagePlugin:
//!
//! ```ignore
//!     App::new()
//!         .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
//!         ...
//! ```
//!
//! You can also ask the plugin to automatically set and resize the viewport of
//! the camera. This way, if the window size is not an exact multiple of the
//! virtual resolution, anything out of bounds will be hidden.
//!
//! A small example is included in the crate. Run it with:
//!
//! ```console
//! cargo run --example flappin
//! ```
//!
//! # Comparison with other methods
//!
//! There is two main methods to render pixel-art games: upscale each sprite
//! independently, or render everything to an offscreen texture and only upscale
//! this texture. This crate use the first method. There is advantages and
//! drawbacks to both approaches.
//!
//! Advantages of the "upscale each sprite independently" method:
//!
//! - allows for smoother scrolling and movement of sprites, if you're willing
//!   to temporarily break the alignment on virtual pixels (this would be even
//!   more effective with a dedicated upscaling shader);
//! - easier to mix pixel-art and high resolution graphics (for example for
//!   text, particles or effects).
//!
//! Advantages of the "offscreen texture" method:
//!
//! - always ensure perfect alignment on virtual pixels (authentic "retro"
//!   look);
//! - probably more efficient (in most cases, the difference is probably
//!   negligible on modern computers).
//!
//! # Example code
//!
//! ```no_run
//! use bevy::prelude::*;
//! use bevy::sprite::Anchor;
//! use bevy_pixel_camera::{
//!     PixelCameraBundle, PixelCameraPlugin
//! };
//!
//! fn main() {
//!     App::new()
//!         .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
//!         .add_plugin(PixelCameraPlugin)
//!         .add_startup_system(setup)
//!         .run();
//! }
//!
//! fn setup(
//!     mut commands: Commands,
//!     asset_server: Res<AssetServer>,
//! ) {
//!     commands.spawn((
//!         Camera2dBundle::default(),
//!         PixelZoom::Auto {
//!             width: 320,
//!             height: 180,
//!         },
//!         PixelViewport,
//!     ));
//!
//!     commands.spawn(SpriteBundle {
//!         texture: asset_server.load("my-pixel-art-sprite.png"),
//!         sprite: Sprite {
//!             anchor: Anchor::BottomLeft,
//!             ..Default::default()
//!         },
//!         ..Default::default()
//!     });
//! }
//! ```
//!
//! # Bevy versions supported
//!
//! | bevy | bevy_pixel_camera |
//! |------|-------------------|
//! | 0.11 | 0.5               |
//! | 0.10 | 0.4.1             |
//! | 0.9  | 0.3               |
//! | 0.8  | 0.2               |
//!
//! ## Migration guide: 0.4 to 0.5 (Bevy 0.10 to 0.11)
//!
//! The `PixelBorderPlugin` has been deprecated. If you want a border around
//! your virtual resolution, pass `true` to the `set_viewport` argument when
//! creating the camera bundle (see example above).
//!
//! # License
//!
//! Licensed under either of
//!
//! - Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
//!   <http://www.apache.org/licenses/LICENSE-2.0>)
//! - MIT license ([LICENSE-MIT](LICENSE-MIT) or
//!   <http://opensource.org/licenses/MIT>)
//!
//! at your option.

mod pixel_border;
mod pixel_camera;
mod pixel_plugin;
mod pixel_zoom;

#[allow(deprecated)]
pub use pixel_border::*;
#[allow(deprecated)]
pub use pixel_camera::*;
pub use pixel_plugin::*;
pub use pixel_zoom::*;
