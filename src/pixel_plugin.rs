use super::PixelProjection;
use bevy::prelude::{
    App, EventReader, IntoSystemConfigs, Plugin, PostUpdate, Query, UVec2, Window, With,
};
use bevy::render::camera::{
    self, Camera, OrthographicProjection, PerspectiveProjection, Projection, ScalingMode, Viewport,
};
use bevy::render::primitives::Aabb;
use bevy::render::view::visibility;
use bevy::render::view::{ComputedVisibility, Visibility, VisibleEntities};
use bevy::transform::TransformSystem;
use bevy::window::WindowResized;

/// Provides the camera system.
pub struct PixelCameraPlugin;

impl Plugin for PixelCameraPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Camera>()
            .register_type::<Visibility>()
            .register_type::<ComputedVisibility>()
            .register_type::<OrthographicProjection>()
            .register_type::<VisibleEntities>()
            .register_type::<ScalingMode>()
            .register_type::<Aabb>()
            .add_systems(PostUpdate, update_viewport)
            .add_systems(PostUpdate, camera::camera_system::<PixelProjection>)
            .add_systems(
                PostUpdate,
                visibility::update_frusta::<PixelProjection>
                    .in_set(visibility::VisibilitySystems::UpdateOrthographicFrusta)
                    .after(camera::camera_system::<PixelProjection>)
                    .after(TransformSystem::TransformPropagate)
                    .ambiguous_with(visibility::update_frusta::<PerspectiveProjection>)
                    .ambiguous_with(visibility::update_frusta::<OrthographicProjection>)
                    .ambiguous_with(visibility::update_frusta::<Projection>),
            );
    }
}

#[allow(clippy::type_complexity)]
fn update_viewport(
    mut resize_events: EventReader<WindowResized>,
    windows: Query<&Window>,
    mut cameras: Query<(&mut Camera, &PixelProjection), With<PixelProjection>>,
) {
    for event in resize_events.iter() {
        let window = windows.get(event.window).unwrap(); //TODO: better than unwrap?
        for (mut camera, projection) in cameras.iter_mut() {
            //TODO
            if projection.set_viewport {
                let zoom = projection.desired_zoom(event.width, event.height);
                let window_scale = window.resolution.scale_factor();
                let physical_width;
                let physical_height;
                let physical_x;
                let physical_y;

                if let Some(target_width) = projection.desired_width {
                    let viewport_width = (target_width * zoom) as f64;
                    physical_width = (window_scale * viewport_width) as u32;
                    physical_x =
                        (window_scale * ((event.width as f64) - viewport_width)) as u32 / 2;
                } else {
                    physical_width = window.physical_width();
                    physical_x = 0;
                }
                if let Some(target_height) = projection.desired_height {
                    let viewport_height = (target_height * zoom) as f64;
                    physical_height = (window_scale * viewport_height) as u32;
                    physical_y =
                        (window_scale * ((event.height as f64) - viewport_height)) as u32 / 2;
                } else {
                    physical_height = window.physical_height();
                    physical_y = 0;
                }
                camera.viewport = Some(Viewport {
                    physical_position: UVec2 {
                        x: physical_x,
                        y: physical_y,
                    },
                    physical_size: UVec2 {
                        x: physical_width,
                        y: physical_height,
                    },
                    ..Default::default()
                });
            }
        }
    }
}
