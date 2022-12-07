use bevy::prelude::*;
use bevy::sprite::Anchor;

use crate::PixelProjection;

/// Provides an opaque border around the desired resolution.
pub struct PixelBorderPlugin {
    pub color: Color,
}

impl Plugin for PixelBorderPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BorderColor(self.color))
            .add_startup_system(spawn_borders)
            .add_system_to_stage(CoreStage::PostUpdate, resize_borders);
    }
}

/// Resource used to specify the color of the opaque border.
#[derive(Clone, Debug, Resource)]
pub struct BorderColor(Color);

// Component
#[derive(Component)]
enum Border {
    Left,
    Right,
    Top,
    Bottom,
}

/// System to spawn the opaque border. Automatically added by the plugin as a
/// startup system.
pub fn spawn_borders(mut commands: Commands, color: Res<BorderColor>) {
    let mut spawn_border = |name: &'static str, side: Border| -> Entity {
        commands
            .spawn((
                Name::new(name),
                side,
                SpriteBundle {
                    sprite: Sprite {
                        anchor: Anchor::BottomLeft,
                        color: color.0,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ))
            .id()
    };

    let left = spawn_border("Left", Border::Left);
    let right = spawn_border("Right", Border::Right);
    let top = spawn_border("Top", Border::Top);
    let bottom = spawn_border("Bottom", Border::Bottom);

    commands
        .spawn((SpatialBundle::default(), Name::new("Borders")))
        .push_children(&[left, right, top, bottom]);
}

fn resize_borders(
    cameras: Query<
        (&PixelProjection, &GlobalTransform),
        Or<(Changed<PixelProjection>, Changed<GlobalTransform>)>,
    >,
    mut borders: Query<(&mut Sprite, &mut Transform, &Border), Without<PixelProjection>>,
) {
    if let Some((projection, transform)) = cameras.iter().next() {
        let z = projection.far - 0.2;
        let width = projection.desired_width.map(|w| w as f32).unwrap_or(0.0);
        let height = projection.desired_height.map(|h| h as f32).unwrap_or(0.0);
        let left = transform.translation().x
            + if projection.centered {
                -(width / 2.0).round()
            } else {
                0.0
            };
        let right = left + width;
        let bottom = transform.translation().y
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
                    sprite.custom_size = Some(Vec2::new(width, 3.0 * height));
                }
                Border::Right => {
                    *transform = Transform::from_xyz(right, bottom - height, z);
                    sprite.custom_size = Some(Vec2::new(width, 3.0 * height));
                }
                Border::Top => {
                    *transform = Transform::from_xyz(left - width, top, z);
                    sprite.custom_size = Some(Vec2::new(3.0 * width, height));
                }
                Border::Bottom => {
                    *transform = Transform::from_xyz(left - width, bottom - height, z);
                    sprite.custom_size = Some(Vec2::new(3.0 * width, height));
                }
            }
        }
    }
}
