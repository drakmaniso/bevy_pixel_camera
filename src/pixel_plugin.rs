#[allow(deprecated)]
use super::PixelProjection;

use bevy::prelude::{App, IntoSystemConfigs, Plugin, PostUpdate};
use bevy::render::camera::{
    self, Camera, OrthographicProjection, PerspectiveProjection, Projection, ScalingMode,
};
use bevy::render::primitives::Aabb;
use bevy::render::view::visibility;
use bevy::render::view::{InheritedVisibility, Visibility, VisibleEntities};
use bevy::transform::TransformSystem;

/// Provides the camera system.
pub struct PixelCameraPlugin;

#[allow(deprecated)]
impl Plugin for PixelCameraPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Camera>()
            .register_type::<Visibility>()
            .register_type::<InheritedVisibility>()
            .register_type::<OrthographicProjection>()
            .register_type::<VisibleEntities>()
            .register_type::<ScalingMode>()
            .register_type::<Aabb>()
            .add_systems(PostUpdate, super::update_pixel_camera_viewport)
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
            )
            .add_systems(
                PostUpdate,
                super::pixel_zoom_system.after(camera::camera_system::<OrthographicProjection>),
            );
    }
}
