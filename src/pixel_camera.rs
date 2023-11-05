#![deprecated(since = "0.5.1", note = "please use the `PixelZoom` component instead")]
#![allow(deprecated)]

use bevy::math::Vec3A;
use bevy::prelude::{
    Bundle, Camera2d, Component, EventReader, GlobalTransform, Mat4, Query, Reflect,
    ReflectComponent, Transform, UVec2, With,
};
use bevy::render::camera::{Camera, CameraProjection, CameraRenderGraph, Viewport};
use bevy::render::primitives::Frustum;
use bevy::render::view::VisibleEntities;
use bevy::window::{Window, WindowResized};

/// Provides the components for the camera entity.
///
/// When using this camera, world coordinates are expressed using virtual
/// pixels, which are always mapped to a multiple of actual screen pixels.
#[derive(Bundle)]
pub struct PixelCameraBundle {
    pub camera: Camera,
    pub camera_render_graph: CameraRenderGraph,
    pub pixel_projection: PixelProjection,
    pub visible_entities: VisibleEntities,
    pub frustum: Frustum,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub camera_2d: Camera2d,
}

impl PixelCameraBundle {
    /// Create a component bundle for a camera with the specified projection.
    pub fn new(pixel_projection: PixelProjection) -> Self {
        let transform = Transform::from_xyz(0.0, 0.0, 0.0);
        let view_projection =
            pixel_projection.get_projection_matrix() * transform.compute_matrix().inverse();
        let frustum = Frustum::from_view_projection_custom_far(
            &view_projection,
            &transform.translation,
            &transform.back(),
            pixel_projection.far(),
        );
        Self {
            camera_render_graph: CameraRenderGraph::new(bevy::core_pipeline::core_2d::graph::NAME),
            pixel_projection,
            visible_entities: Default::default(),
            frustum,
            transform,
            global_transform: Default::default(),
            camera: Camera::default(),
            camera_2d: Camera2d::default(),
        }
    }

    /// Create a component bundle for a camera where the size of virtual pixels
    /// are specified with `zoom`.
    pub fn from_zoom(zoom: i32) -> Self {
        Self::new(PixelProjection {
            zoom,
            ..Default::default()
        })
    }

    /// Create a component bundle for a camera where the size of virtual pixels
    /// is automatically set to fit the specified resolution inside the window.
    ///
    /// If `set_viewport` is true, pixels outside of the desired resolution will
    /// not be displayed. This will automatically set the viewport of the
    /// camera, and resize it when necessary.
    pub fn from_resolution(width: i32, height: i32, set_viewport: bool) -> Self {
        Self::new(PixelProjection {
            desired_width: Some(width),
            desired_height: Some(height),
            set_viewport,
            ..Default::default()
        })
    }

    /// Create a component bundle for a camera where the size of virtual pixels
    /// is automatically set to fit the specified width inside the window.
    ///
    /// If `set_viewport` is true, pixels outside of the desired width will
    /// not be displayed. This will automatically set the viewport of the
    /// camera, and resize it when necessary.
    pub fn from_width(width: i32, set_viewport: bool) -> Self {
        Self::new(PixelProjection {
            desired_width: Some(width),
            set_viewport,
            ..Default::default()
        })
    }

    /// Create a component bundle for a camera where the size of virtual pixels
    /// is automatically set to fit the specified height inside the window.
    ///
    /// If `set_viewport` is true, pixels outside of the desired height will
    /// not be displayed. This will automatically set the viewport of the
    /// camera, and resize it when necessary.
    pub fn from_height(height: i32, set_viewport: bool) -> Self {
        Self::new(PixelProjection {
            desired_height: Some(height),
            set_viewport,
            ..Default::default()
        })
    }
}

/// Component for a pixel-perfect orthographic projection.
///
/// It is similar to Bevy's OrthographicProjection, except integral world
/// coordinates are always aligned with virtual pixels (as defined by the zoom
/// field).
#[derive(Debug, Clone, Reflect, Component)]
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

    /// If true, (0, 0) is the pixel closest to the center of the window,
    /// otherwise it's at bottom left.
    pub centered: bool,

    /// If true, pixels outside of the desired resolution will not be displayed.
    /// This will automatically set the viewport of the camera, and resize it
    /// when necessary.
    pub set_viewport: bool,
}

impl CameraProjection for PixelProjection {
    fn get_projection_matrix(&self) -> Mat4 {
        Mat4::orthographic_rh(
            self.left,
            self.right,
            self.bottom,
            self.top,
            // NOTE: near and far are swapped to invert the depth range from [0,1] to [1,0]
            // This is for interoperability with pipelines using infinite reverse perspective projections.
            self.far,
            self.near,
        )
    }

    fn update(&mut self, width: f32, height: f32) {
        self.zoom = self.desired_zoom(width, height);

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

    fn far(&self) -> f32 {
        self.far
    }

    fn get_frustum_corners(&self, z_near: f32, z_far: f32) -> [Vec3A; 8] {
        // NOTE: These vertices are in the specific order required by [`calculate_cascade`].
        [
            Vec3A::new(self.right, self.bottom, z_near), // bottom right
            Vec3A::new(self.right, self.top, z_near),    // top right
            Vec3A::new(self.left, self.top, z_near),     // top left
            Vec3A::new(self.left, self.bottom, z_near),  // bottom left
            Vec3A::new(self.right, self.bottom, z_far),  // bottom right
            Vec3A::new(self.right, self.top, z_far),     // top right
            Vec3A::new(self.left, self.top, z_far),      // top left
            Vec3A::new(self.left, self.bottom, z_far),   // bottom left
        ]
    }
}

impl PixelProjection {
    pub fn desired_zoom(&self, width: f32, height: f32) -> i32 {
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
            (Some(zoom_x), Some(zoom_y)) => zoom_x.min(zoom_y).max(1),
            (Some(zoom_x), None) => zoom_x.max(1),
            (None, Some(zoom_y)) => zoom_y.max(1),
            (None, None) => self.zoom,
        }
    }
}

impl Default for PixelProjection {
    fn default() -> Self {
        Self {
            left: -1.0,
            right: 1.0,
            bottom: -1.0,
            top: 1.0,
            near: -1000.0,
            far: 1000.0,
            desired_width: None,
            desired_height: None,
            zoom: 1,
            centered: true,
            set_viewport: false,
        }
    }
}

#[allow(clippy::type_complexity)]
pub(crate) fn update_pixel_camera_viewport(
    mut resize_events: EventReader<WindowResized>,
    windows: Query<&Window>,
    mut cameras: Query<(&mut Camera, &PixelProjection), With<PixelProjection>>,
) {
    for event in resize_events.iter() {
        let window = windows.get(event.window).unwrap(); // TODO: better than unwrap?
        for (mut camera, projection) in cameras.iter_mut() {
            //TODO
            if projection.set_viewport {
                let zoom = projection.desired_zoom(event.width, event.height);
                let scale_factor = window.resolution.scale_factor();
                let viewport_width;
                let viewport_height;
                let viewport_x;
                let viewport_y;

                if let Some(target_width) = projection.desired_width {
                    let logical_target_width = (target_width * zoom) as f64;
                    viewport_width = (scale_factor * logical_target_width) as u32;
                    viewport_x =
                        (scale_factor * ((event.width as f64) - logical_target_width)) as u32 / 2;
                } else {
                    viewport_width = window.physical_width();
                    viewport_x = 0;
                }
                if let Some(target_height) = projection.desired_height {
                    let logicat_target_height = (target_height * zoom) as f64;
                    viewport_height = (scale_factor * logicat_target_height) as u32;
                    viewport_y =
                        (scale_factor * ((event.height as f64) - logicat_target_height)) as u32 / 2;
                } else {
                    viewport_height = window.physical_height();
                    viewport_y = 0;
                }
                camera.viewport = Some(Viewport {
                    physical_position: UVec2 {
                        x: viewport_x,
                        y: viewport_y,
                    },
                    physical_size: UVec2 {
                        x: viewport_width,
                        y: viewport_height,
                    },
                    ..Default::default()
                });
            }
        }
    }
}
