//! A simple pixel-perfect camera plugin for Bevy, suitable for pixel-art.
//!
//! This crates makes it possible to correctly display low-resolution 2d
//! graphics with Bevy's default renderer and sprites.
//!
//! While it is possible to achieve pixel-perfect rendering of such sprites with
//! Bevy's own camera and `OrthographicProjection`, doing so correctly requires
//! to configure in a specific way (you can't just use
//! `OrhtographicCameraBundle`).
//!
//! This plugin offers a camera which can be configured by specifying either the
//! size of the virtual pixels, or the desired resolution.
//!
//! It also includes a quad mesh resource to replace the default one used in
//! Bevy's `SpriteBundle`. The default quad has its origin at the center of the
//! image, but if the image has an odd width or height, that origin is not
//! pixel-aligned. The resource included in this plugin puts the origin at the
//! bottom-left corner of the image.
//!
//! ## Example code
//!
//! ```no_compile
//! use bevy::prelude::*;
//! use bevy_pixel_camera::
//!
//! fn main() {
//!     App::build()
//!         .add_plugins(DefaultPlugins)
//!         .add_plugin(PixelCameraPlugin)
//!         .add_startup_system(setup.system())
//!         .run();
//! }
//!
//! fn setup(
//!     mut commands: Commands,
//!     asset_server: Res<AssetServer>,
//!     mut materials: ResMut<Assets<ColorMaterial>>,
//!     quad: Res<PixelSpriteQuad>,
//! ) {
//!     commands.spawn_bundle(PixelCameraBundle::from_zoom(3));
//!
//!     let sprite_handle = materials.add(asset_server.load("my-pixel-art-sprite.png").into());
//!     commands.spawn_bundle(SpriteBundle {
//!         material: sprite_handle,
//!         mesh: quad.clone().into(),
//!         ..Default::default()
//!     });
//! }
//! ```

mod pixel_camera;
mod pixel_plugin;
#[cfg(test)]
mod tests;

pub use pixel_camera::*;
pub use pixel_plugin::*;
