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
//!         material: sprite_handle.clone(),
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

// Pixel-Art Camera Bundle

/// Component bundle for camera entities suitable for pixel-art sprites.
///
/// Use this for pixel-art games. Transforms are expressed using "virtual
/// pixels", which are mapped to a multiple of actual screen pixels.
#[derive(Bundle)]
pub struct PixelCameraBundle {
    pub camera: Camera,
    pub pixel_projection: PixelProjection,
    pub visible_entities: VisibleEntities,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl PixelCameraBundle {
    /// Create a component bundle for a pixel-perfect orthographic camera, where
    /// 1 world unit = `zoom` screen pixels.
    pub fn from_zoom(zoom: i32) -> Self {
        // we want 0 to be "closest" and +far to be "farthest" in 2d, so we offset
        // the camera's translation by far and use a right handed coordinate system
        let far = 1000.0;
        Self {
            camera: Camera {
                name: Some(bevy::render::render_graph::base::camera::CAMERA_2D.to_string()),
                ..Default::default()
            },
            pixel_projection: PixelProjection {
                zoom: zoom,
                ..Default::default()
            },
            visible_entities: Default::default(),
            transform: Transform::from_xyz(0.0, 0.0, far - 0.1),
            global_transform: Default::default(),
        }
    }

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

#[derive(Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct PixelProjection {
    pub left: f32,
    pub right: f32,
    pub bottom: f32,
    pub top: f32,
    pub near: f32,
    pub far: f32,
    pub desired_width: Option<i32>,
    pub desired_height: Option<i32>,
    pub zoom: i32,
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
            /// If present,
            desired_width: None,
            desired_height: None,
            zoom: 1,
            centered: true,
        }
    }
}

// Pixel-Art Sprite

use bevy::math::vec2;
use bevy::render::mesh::Indices;

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

// Tests for the projection matrix

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::math::Vec4;

    #[test]
    fn projection_test_around_240x240() {
        for width in 230..250 {
            for height in 230..250 {
                for zoom in 1..5 {
                    projection_test_for_width_height_zoom(width, height, zoom);
                }
            }
        }
    }

    #[test]
    fn projection_test_around_1280x720() {
        for width in 1275..1285 {
            for height in 715..725 {
                for zoom in 2..6 {
                    projection_test_for_width_height_zoom(width, height, zoom);
                }
            }
        }
    }

    #[test]
    fn projection_test_around_1920x1080() {
        for width in 1915..1925 {
            for height in 1075..1085 {
                for zoom in 2..6 {
                    projection_test_for_width_height_zoom(width, height, zoom);
                }
            }
        }
    }

    #[test]
    fn projection_test_around_3840x2160() {
        for width in 3835..3845 {
            for height in 2155..2165 {
                for zoom in 3..6 {
                    projection_test_for_width_height_zoom(width, height, zoom);
                }
            }
        }
    }

    const MIN_WIDTH: i32 = 240;
    const MIN_HEIGHT: i32 = 240;
    const MAX_WIDTH: i32 = 3840;
    const MAX_HEIGHT: i32 = 2160;

    #[test]
    #[ignore]
    fn expensive_projection_zoom_1() {
        expensive_projection_test_for_zoom(1);
    }

    #[test]
    #[ignore]
    fn expensive_projection_zoom_2() {
        expensive_projection_test_for_zoom(2);
    }

    #[test]
    #[ignore]
    fn expensive_projection_zoom_3() {
        expensive_projection_test_for_zoom(3);
    }

    #[test]
    #[ignore]
    fn expensive_projection_zoom_4() {
        expensive_projection_test_for_zoom(4);
    }

    #[test]
    #[ignore]
    fn expensive_projection_zoom_5() {
        expensive_projection_test_for_zoom(5);
    }

    #[test]
    #[ignore]
    fn expensive_projection_zoom_6() {
        expensive_projection_test_for_zoom(6);
    }

    #[test]
    #[ignore]
    fn expensive_projection_zoom_7() {
        expensive_projection_test_for_zoom(7);
    }

    #[test]
    #[ignore]
    fn expensive_projection_zoom_8() {
        expensive_projection_test_for_zoom(8);
    }

    fn expensive_projection_test_for_zoom(zoom: i32) {
        for width in MIN_WIDTH..MAX_WIDTH {
            let min_height = MIN_HEIGHT.max(width / 2);
            let max_height = MAX_HEIGHT.min(width);
            println!(
                "// Testing resolutions {}x[{}..{}] at zoom {}",
                width, min_height, max_height, zoom
            );
            for height in min_height..max_height {
                projection_test_for_width_height_zoom(width, height, zoom);
            }
        }
    }

    fn projection_test_for_width_height_zoom(width: i32, height: i32, zoom: i32) {
        let mut window_projection = bevy::render::camera::OrthographicProjection::default();
        let mut virtual_projection = PixelProjection {
            zoom: zoom,
            ..Default::default()
        };
        virtual_projection.update(width as f32, height as f32);
        window_projection.update(width as f32, height as f32);

        let virtual_matrix = virtual_projection.get_projection_matrix();
        let window_matrix = window_projection.get_projection_matrix();

        let virtual_width = width / zoom;
        let virtual_height = height / zoom;
        for x in -(virtual_width / 2)..(virtual_width - virtual_width / 2) {
            for y in -(virtual_height / 2)..(virtual_height - virtual_height / 2) {
                let virtual_pixel = Vec4::new(x as f32, y as f32, 0.0, 1.0);
                let expected_window_pixel = Vec4::new(
                    (virtual_pixel.x + ((virtual_width / 2) as f32)) * (zoom as f32),
                    (virtual_pixel.y + ((virtual_height / 2) as f32)) * (zoom as f32),
                    0.0,
                    1.0,
                );

                let virtual_pos = virtual_matrix * virtual_pixel;
                let _window_pos = window_matrix * expected_window_pixel;
                let computed_window_pixel = Vec4::new(
                    (virtual_pos.x + 1.0) * (width as f32) * 0.5,
                    (virtual_pos.y + 1.0) * (height as f32) * 0.5,
                    0.0,
                    1.0,
                );

                assert_eq!(
                    computed_window_pixel.round(),
                    expected_window_pixel,
                    "Resolution {}x{} Zoom {} Pixel ({}, {})",
                    width,
                    height,
                    zoom,
                    x,
                    y,
                );
            }
        }
    }
}
