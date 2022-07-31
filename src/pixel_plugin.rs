use bevy::prelude::{App, CoreStage, Plugin};
use bevy::render::camera::{
    self, Camera, DepthCalculation, OrthographicProjection, ScalingMode, WindowOrigin,
};
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
            .register_type::<WindowOrigin>()
            .register_type::<ScalingMode>()
            .register_type::<DepthCalculation>()
            .register_type::<Aabb>()
            .add_system_to_stage(
                CoreStage::PostUpdate,
                camera::camera_system::<super::PixelProjection>,
            );
    }
}
