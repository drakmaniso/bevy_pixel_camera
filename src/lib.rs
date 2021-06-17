//! A simple pixel-perfect camera plugin for Bevy, suitable for pixel-art.
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

use bevy::prelude::{
    AppBuilder, Assets, Bundle, Commands, CoreStage, GlobalTransform, Handle, IntoSystem, Mat4,
    Mesh, Plugin, Reflect, ReflectComponent, ResMut, StartupStage, Transform,
};
use bevy::render::camera::{self, Camera, CameraProjection, DepthCalculation, VisibleEntities};

#[cfg(test)]
mod tests;

/// Plugin for the camera system and sprite quad resource.
pub struct PixelCameraPlugin;

impl Plugin for PixelCameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system_to_stage(
            StartupStage::PreStartup,
            setup_pixel_camera_plugin.system(),
        )
        .add_system_to_stage(
            CoreStage::PostUpdate,
            camera::camera_system::<PixelProjection>.system(),
        );
    }
}

fn setup_pixel_camera_plugin(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let quad = PixelSpriteQuad(meshes.add(make_quad()));
    commands.insert_resource(quad);
}

/// Component bundle for camera entities suitable for pixel-art sprites.
///
/// Use this for pixel-art games. World coordinates are expressed using virtual
/// pixels, which are mapped to a multiple of actual screen pixels.
#[derive(Bundle)]
pub struct PixelCameraBundle {
    pub camera: Camera,
    pub pixel_projection: PixelProjection,
    pub visible_entities: VisibleEntities,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl PixelCameraBundle {
    /// Create a component bundle for a camera where the size of virtual pixels
    /// are specified with `zoom`.
    pub fn from_zoom(zoom: i32) -> Self {
        // we want 0 to be "closest" and +far to be "farthest" in 2d, so we offset
        // the camera's translation by far and use a right handed coordinate system
        let projection = PixelProjection {
            zoom: zoom,
            ..Default::default()
        };
        let far = projection.far;
        Self {
            camera: Camera {
                name: Some(bevy::render::render_graph::base::camera::CAMERA_2D.to_string()),
                ..Default::default()
            },
            pixel_projection: projection,
            visible_entities: Default::default(),
            transform: Transform::from_xyz(0.0, 0.0, far - 0.1),
            global_transform: Default::default(),
        }
    }

    /// Create a component bundle for a camera where the size of virtual pixels
    /// is automatically set to fit the specified resolution inside the window.
    pub fn from_resolution(width: i32, height: i32) -> Self {
        let far = 1000.0;
        Self {
            camera: Camera {
                name: Some(bevy::render::render_graph::base::camera::CAMERA_2D.to_string()),
                ..Default::default()
            },
            pixel_projection: PixelProjection {
                desired_width: Some(width),
                desired_height: Some(height),
                ..Default::default()
            },
            visible_entities: Default::default(),
            transform: Transform::from_xyz(0.0, 0.0, far - 0.1),
            global_transform: Default::default(),
        }
    }

    /// Create a component bundle for a camera where the size of virtual pixels
    /// is automatically set to fit the specified width inside the window.
    pub fn from_width(width: i32) -> Self {
        let far = 1000.0;
        Self {
            camera: Camera {
                name: Some(bevy::render::render_graph::base::camera::CAMERA_2D.to_string()),
                ..Default::default()
            },
            pixel_projection: PixelProjection {
                desired_width: Some(width),
                ..Default::default()
            },
            visible_entities: Default::default(),
            transform: Transform::from_xyz(0.0, 0.0, far - 0.1),
            global_transform: Default::default(),
        }
    }

    /// Create a component bundle for a camera where the size of virtual pixels
    /// is automatically set to fit the specified height inside the window.
    pub fn from_height(height: i32) -> Self {
        let far = 1000.0;
        Self {
            camera: Camera {
                name: Some(bevy::render::render_graph::base::camera::CAMERA_2D.to_string()),
                ..Default::default()
            },
            pixel_projection: PixelProjection {
                desired_height: Some(height),
                ..Default::default()
            },
            visible_entities: Default::default(),
            transform: Transform::from_xyz(0.0, 0.0, far - 0.1),
            global_transform: Default::default(),
        }
    }
}

/// Component for a pixel-perfect orthographic projection.
///
/// It is similar to Bevy's OrthographicProjection, except integral world
/// coordinates are always aligned with virtual pixels (as defined by the zoom
/// field).
#[derive(Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct PixelProjection {
    pub left: f32,
    pub right: f32,
    pub bottom: f32,
    pub top: f32,
    pub near: f32,
    pub far: f32,

    /// If present, `zoom` will be automatically updated to always fit
    /// `desired_width` in the window as best as possible.
    pub desired_width: Option<i32>,

    /// If present, `zoom` will be automatically updated to always fit
    /// `desired_height` in the window as best as possible.
    pub desired_height: Option<i32>,

    /// If neither `desired_width` nor `desired_height` are present, zoom can be
    /// manually set. The value detemines the size of the virtual pixels.
    pub zoom: i32,

    // If true, (0, 0) is the pixel closest to the center of the windoe,
    // otherwise it's at bottom left.
    pub centered: bool,
}

impl CameraProjection for PixelProjection {
    fn get_projection_matrix(&self) -> Mat4 {
        Mat4::orthographic_rh(
            self.left,
            self.right,
            self.bottom,
            self.top,
            self.near,
            self.far,
        )
    }

    fn update(&mut self, width: f32, height: f32) {
        let mut zoom_x = None;
        if let Some(desired_width) = self.desired_width {
            if desired_width > 0 {
                zoom_x = Some((width as i32) / desired_width);
            }
        }
        let mut zoom_y = None;
        if let Some(desired_height) = self.desired_height {
            if desired_height > 0 {
                zoom_y = Some((height as i32) / desired_height);
            }
        }
        match (zoom_x, zoom_y) {
            (Some(zoom_x), Some(zoom_y)) => self.zoom = zoom_x.min(zoom_y).max(1),
            (Some(zoom_x), None) => self.zoom = zoom_x.max(1),
            (None, Some(zoom_y)) => self.zoom = zoom_y.max(1),
            (None, None) => (),
        }

        let actual_width = width / (self.zoom as f32);
        let actual_height = height / (self.zoom as f32);
        if self.centered {
            self.left = -((actual_width as i32) / 2) as f32;
            self.right = self.left + actual_width;
            self.bottom = -((actual_height as i32) / 2) as f32;
            self.top = self.bottom + actual_height;
        } else {
            self.left = 0.0;
            self.right = actual_width;
            self.bottom = 0.0;
            self.top = actual_height;
        }
    }

    fn depth_calculation(&self) -> DepthCalculation {
        DepthCalculation::ZDifference
    }
}

impl Default for PixelProjection {
    fn default() -> Self {
        Self {
            left: -1.0,
            right: 1.0,
            bottom: -1.0,
            top: 1.0,
            near: 0.0,
            far: 1000.0,
            desired_width: None,
            desired_height: None,
            zoom: 1,
            centered: true,
        }
    }
}

use bevy::math::vec2;
use bevy::render::mesh::Indices;

/// Resource inserted by `PixelCameraPlugin`, to replace bevy's default mesh for
/// sprite bundles.
#[derive(Clone)]
pub struct PixelSpriteQuad(Handle<Mesh>);

impl From<Handle<Mesh>> for PixelSpriteQuad {
    fn from(handle: Handle<Mesh>) -> Self {
        Self(handle)
    }
}

impl Into<Handle<Mesh>> for PixelSpriteQuad {
    fn into(self) -> Handle<Mesh> {
        self.0
    }
}

fn make_quad() -> Mesh {
    let north_west = vec2(0.0, 1.0);
    let north_east = vec2(1.0, 1.0);
    let south_west = vec2(0.0, 0.0);
    let south_east = vec2(1.0, 0.0);
    let vertices = [
        (
            [south_east.x, south_east.y, 0.0],
            [0.0, 0.0, 1.0],
            [1.0, 1.0],
        ),
        (
            [north_east.x, north_east.y, 0.0],
            [0.0, 0.0, 1.0],
            [1.0, 0.0],
        ),
        (
            [north_west.x, north_west.y, 0.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0],
        ),
        (
            [south_west.x, south_west.y, 0.0],
            [0.0, 0.0, 1.0],
            [0.0, 1.0],
        ),
    ];

    let indices = Indices::U32(vec![0, 2, 1, 0, 3, 2]);

    let mut positions = Vec::<[f32; 3]>::new();
    let mut normals = Vec::<[f32; 3]>::new();
    let mut uvs = Vec::<[f32; 2]>::new();
    for (position, normal, uv) in vertices.iter() {
        positions.push(*position);
        normals.push(*normal);
        uvs.push(*uv);
    }

    let mut mesh = Mesh::new(bevy::render::pipeline::PrimitiveTopology::TriangleList);
    mesh.set_indices(Some(indices));
    mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh
}
