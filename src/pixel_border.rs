use bevy::prelude::*;

use crate::{PixelProjection, PixelSpriteQuad};

/// Provides an opaque border around the desired resolution.
pub struct PixelBorderPlugin {
    pub color: Color,
}

impl Plugin for PixelBorderPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(BorderColor(self.color))
            .add_startup_system(spawn_borders.system())
            .add_system_to_stage(CoreStage::PostUpdate, resize_borders.system());
    }
}

// Resource
#[derive(Clone, Debug)]
struct BorderColor(Color);

// Component
enum Border {
    Left,
    Right,
    Top,
    Bottom,
}

fn spawn_borders(
    mut commands: Commands,
    color: Res<BorderColor>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    quad: Res<PixelSpriteQuad>,
) {
    let material = materials.add(color.0.into());
    commands
        .spawn()
        .insert(Border::Left)
        .insert_bundle(SpriteBundle {
            material: material.clone(),
            mesh: quad.clone().into(),
            ..Default::default()
        });
    commands
        .spawn()
        .insert(Border::Right)
        .insert_bundle(SpriteBundle {
            material: material.clone(),
            mesh: quad.clone().into(),
            ..Default::default()
        });
    commands
        .spawn()
        .insert(Border::Top)
        .insert_bundle(SpriteBundle {
            material: material.clone(),
            mesh: quad.clone().into(),
            ..Default::default()
        });
    commands
        .spawn()
        .insert(Border::Bottom)
        .insert_bundle(SpriteBundle {
            material: material.clone(),
            mesh: quad.clone().into(),
            ..Default::default()
        });
}

fn resize_borders(
    cameras: Query<
        (&PixelProjection, &Transform),
        Or<(Changed<PixelProjection>, Changed<Transform>)>,
    >,
    mut borders: Query<(&mut Sprite, &mut Transform, &Border), Without<PixelProjection>>,
) {
    if let Some((projection, transform)) = cameras.iter().next() {
        let z = projection.far - 0.2;
        let width = projection.desired_width.map(|w| w as f32).unwrap_or(0.0);
        let height = projection.desired_height.map(|h| h as f32).unwrap_or(0.0);
        let left = transform.translation.x
            + if projection.centered {
                -(width / 2.0).round()
            } else {
                0.0
            };
        let right = left + width;
        let bottom = transform.translation.y
            + if projection.centered {
                (-height / 2.0).round()
            } else {
                0.0
            };
        let top = bottom + height;

        for (mut sprite, mut transform, border) in borders.iter_mut() {
            match border {
                Border::Left => {
                    *transform = Transform::from_xyz(left - width, bottom - height, z);
                    sprite.size = Vec2::new(width, 3.0 * height);
                }
                Border::Right => {
                    *transform = Transform::from_xyz(right, bottom - height, z);
                    sprite.size = Vec2::new(width, 3.0 * height);
                }
                Border::Top => {
                    *transform = Transform::from_xyz(left - width, top, z);
                    sprite.size = Vec2::new(3.0 * width, height);
                }
                Border::Bottom => {
                    *transform = Transform::from_xyz(left - width, bottom - height, z);
                    sprite.size = Vec2::new(3.0 * width, height);
                }
            }
        }
    }
}
