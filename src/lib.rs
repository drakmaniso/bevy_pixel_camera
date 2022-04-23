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
//! The crate also includes a separate plugin to put an opaque border
//! around the desired resolution. This way, if the window size is not an exact
//! multiple of the virtual resolution, anything out of bounds will still be
//! hidden.
//!
//! ## Comparison with other methods
//!
//! There is several possible methods to render pixel-art based games. This
//! crate simply upscale each sprite, and correctly align them on a virtual
//! pixel grid. Another option would be to render the sprites to an offscrenn
//! texture, and then upscale only this texture. There is advantages and
//! drawbacks to both approaches:
//! - the offscreen method is probably more efficient in most cases;
//! - in both cases the coordinates of non-moving sprites must be manually kept
//!   on integer coordinates;
//! - forgetting to use rounded coordinates will result in much worse results
//!   with the offscreen method; that's why this approach should probably be
//!   paired with a specialized sprite system based on integer transforms;
//! - the method in this crate allows for smoother scrolling and movement of
//!   sprites, if you're willing to temporarily break the alignment on virtual
//!   pixels (this would be even more effective with a dedicated upscaling
//!   shader).
//!
//! ## Example code
//!
//! ```no_run
//! use bevy::prelude::*;
//! use bevy::sprite::Anchor;
//! use bevy_pixel_camera::{
//!     PixelBorderPlugin, PixelCameraBundle, PixelCameraPlugin
//! };
//!
//! fn main() {
//!     App::new()
//!         .add_plugins(DefaultPlugins)
//!         .add_plugin(PixelCameraPlugin)
//!         .add_plugin(PixelBorderPlugin {
//!             color: Color::rgb(0.1, 0.1, 0.1),
//!         })
//!         .add_startup_system(setup)
//!         .run();
//! }
//!
//! fn setup(
//!     mut commands: Commands,
//!     asset_server: Res<AssetServer>,
//!     mut materials: ResMut<Assets<ColorMaterial>>,
//! ) {
//!     commands.spawn_bundle(PixelCameraBundle::from_resolution(320, 240));
//!
//!     let sprite_handle = materials.add(asset_server.load("my-pixel-art-sprite.png").into());
//!     commands.spawn_bundle(SpriteBundle {
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
//! ## License
//!
//! Licensed under either of
//!
//! * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
//! * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
//!
//! at your option.

mod pixel_border;
mod pixel_camera;
mod pixel_plugin;
#[cfg(test)]
mod tests;

pub use pixel_border::*;
pub use pixel_camera::*;
pub use pixel_plugin::*;
