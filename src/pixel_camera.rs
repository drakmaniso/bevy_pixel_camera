use bevy::prelude::{Bundle, GlobalTransform, Mat4, Reflect, ReflectComponent, Transform};
use bevy::render::camera::{Camera, CameraProjection, DepthCalculation, VisibleEntities};

/// Provides the components for the camera entity.
///
/// When using this camera, world coordinates are expressed using virtual
/// pixels, which are always mapped to a multiple of actual screen pixels.
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
