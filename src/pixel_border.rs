use bevy::{prelude::*, window::WindowResized};

use crate::{PixelProjection, PixelSpriteQuad};

pub struct PixelBorderPlugin {
    pub color: Color,
}

impl Plugin for PixelBorderPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(BorderColor(self.color))
            .add_startup_system(spawn_borders.system())
            .add_system(resize_borders.system());
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
    mut resize_events: EventReader<WindowResized>,
    cameras: Query<&PixelProjection>,
    mut borders: Query<(&mut Sprite, &mut Transform, &Border)>,
) {
    if let Some(projection) = cameras.iter().next() {
        let zoom = projection.zoom as f32;
        let z = projection.far - 0.2;
        for ev in resize_events.iter().filter(|ev| ev.id.is_primary()) {
            let win_width = ev.width / zoom;
            let win_height = ev.height / zoom;
            let virtual_width = projection
                .desired_width
                .map(|w| w as f32)
                .unwrap_or(win_width);
            let virtual_height = projection
                .desired_height
                .map(|h| h as f32)
                .unwrap_or(win_height);
            let left_width = ((win_width - virtual_width) / 2.0).round();
            let right_width = win_width - virtual_width - left_width;
            let bottom_height = ((win_height - virtual_height) / 2.0).round();
            let top_height = win_height - virtual_height - bottom_height;
            for (mut sprite, mut transform, border) in borders.iter_mut() {
                match border {
                    Border::Left => {
                        *transform =
                            Transform::from_xyz(projection.left - EXTRA, projection.bottom, z);
                        sprite.size = Vec2::new(left_width + EXTRA, win_height);
                    }
                    Border::Right => {
                        *transform = Transform::from_xyz(
                            projection.right - right_width,
                            projection.bottom,
                            z,
                        );
                        sprite.size = Vec2::new(right_width + EXTRA, win_height);
                    }
                    Border::Top => {
                        *transform =
                            Transform::from_xyz(projection.left, projection.top - top_height, z);
                        sprite.size = Vec2::new(win_width, top_height + EXTRA);
                    }
                    Border::Bottom => {
                        *transform =
                            Transform::from_xyz(projection.left, projection.bottom - EXTRA, z);
                        sprite.size = Vec2::new(win_width, top_height + EXTRA);
                    }
                }
            }
        }
    }
}

const EXTRA: f32 = 32.0;
