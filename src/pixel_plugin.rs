use bevy::prelude::{App, CoreSet, IntoSystemConfig, Plugin};
use bevy::render::camera::{self, Camera, OrthographicProjection, ScalingMode};
use bevy::render::primitives::Aabb;
use bevy::render::view::{ComputedVisibility, Visibility, VisibleEntities};

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
            .add_system(
                camera::camera_system::<super::PixelProjection>.in_base_set(CoreSet::PostUpdate),
            );
    }
}
