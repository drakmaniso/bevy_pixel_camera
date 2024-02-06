use bevy::{
    prelude::*,
    render::camera::{NormalizedRenderTarget, ScalingMode, Viewport},
    utils::HashSet,
    window::{PrimaryWindow, WindowCreated, WindowResized},
};

#[derive(Component, Debug, Clone, PartialEq)]
/// Configure a `Camera2dBundle` to use integer scaling and automatically match
/// a specified resolution.
///
/// Note: when this component is present, a plugin system will automatically
/// update the `ScalingMode` of the camera bundle.
pub enum PixelZoom {
    /// Manually specify the camera zoom, i.e. the number of screen pixels
    /// (logical pixels) used to display one virtual pixel (world unit).
    Fixed(i32),
    /// Automatically set the camera zoom to fit the specified resolution inside
    /// the window.
    FitSize { width: i32, height: i32 },
    /// Automatically set the camera zoom to fit the specified width inside the
    /// window.
    FitWidth(i32),
    /// Automatically set the camera zoom to fit the specified height inside the
    /// window.
    FitHeight(i32),
    /// Set the smaller of the screen dimensions to a specific length (e.g. 160px by whatever-the-long-edge-is)
    FitSmallerDim(i32),
}

#[derive(Component, Debug, Clone, PartialEq)]
/// Configure a `Camera2dBundle` to automatically set the viewport so that only
/// pixels inside the desired resolution (as defined by the `PixelZoom`
/// component) are displayed.
pub struct PixelViewport;

pub(crate) fn pixel_zoom_system(
    mut window_resized_events: EventReader<WindowResized>,
    mut window_created_events: EventReader<WindowCreated>,
    mut image_asset_events: EventReader<AssetEvent<Image>>,
    primary_window: Query<Entity, With<PrimaryWindow>>,
    mut cameras: Query<(
        &mut Camera,
        &PixelZoom,
        Option<&PixelViewport>,
        &mut OrthographicProjection,
    )>,
) {
    // Most of the change detection code is copied from `bevy_render/src/camera`

    // TODO: maybe this can be replaced with just monitoring
    // `OrthographicProjection` for changes?

    let primary_window = primary_window.iter().next();

    let mut changed_window_ids = HashSet::new();
    changed_window_ids.extend(window_created_events.read().map(|event| event.window));
    changed_window_ids.extend(window_resized_events.read().map(|event| event.window));

    let changed_image_handles: HashSet<&AssetId<Image>> = image_asset_events
        .read()
        .filter_map(|event| {
            if let AssetEvent::Modified { id } = event {
                Some(id)
            } else {
                None
            }
        })
        .collect();

    for (mut camera, pixel_zoom, pixel_viewport, mut projection) in &mut cameras {
        if let Some(normalized_target) = camera.target.normalize(primary_window) {
            if is_changed(
                &normalized_target,
                &changed_window_ids,
                &changed_image_handles,
            ) || camera.is_added()
            {
                let logical_size = match camera.logical_target_size() {
                    Some(size) => size,
                    None => continue,
                };

                let physical_size = match camera.physical_target_size() {
                    Some(size) => size,
                    None => continue,
                };

                let zoom = auto_zoom(pixel_zoom, logical_size) as f32;
                match projection.scaling_mode {
                    ScalingMode::WindowSize(previous_zoom) => {
                        if previous_zoom != zoom {
                            projection.scaling_mode = ScalingMode::WindowSize(zoom)
                        }
                    }
                    _ => projection.scaling_mode = ScalingMode::WindowSize(zoom),
                }

                if pixel_viewport.is_some() {
                    set_viewport(&mut camera, pixel_zoom, zoom, physical_size, logical_size);
                }
            }
        }
    }
}

fn is_changed(
    render_target: &NormalizedRenderTarget,
    changed_window_ids: &HashSet<Entity>,
    changed_image_handles: &HashSet<&AssetId<Image>>,
) -> bool {
    match render_target {
        NormalizedRenderTarget::Window(window_ref) => {
            changed_window_ids.contains(&window_ref.entity())
        }
        NormalizedRenderTarget::Image(image_handle) => {
            changed_image_handles.contains(&image_handle.id())
        }
        NormalizedRenderTarget::TextureView(_) => true,
    }
}

fn auto_zoom(mode: &PixelZoom, logical_size: Vec2) -> i32 {
    match mode {
        PixelZoom::FitSize { width, height } => {
            let zoom_x = (logical_size.x as i32) / i32::max(*width, 1);
            let zoom_y = (logical_size.y as i32) / i32::max(*height, 1);
            let zoom = i32::min(zoom_x, zoom_y);
            i32::max(zoom, 1)
        }
        PixelZoom::FitWidth(width) => {
            let zoom = (logical_size.x as i32) / i32::max(*width, 1);
            i32::max(zoom, 1)
        }
        PixelZoom::FitHeight(height) => {
            let zoom = (logical_size.y as i32) / i32::max(*height, 1);
            i32::max(zoom, 1)
        }
        PixelZoom::Fixed(zoom) => *zoom,
        PixelZoom::FitSmallerDim(smaller_length) => {
            let smaller_len = if logical_size.x > logical_size.y {
                logical_size.x
            } else {
                logical_size.y
            };

            

            let zoom = (smaller_len as i32) / i32::max(*smaller_length, 1);
            i32::max(zoom, 1)
        }
    }
}

fn set_viewport(
    camera: &mut Camera,
    mode: &PixelZoom,
    zoom: f32,
    physical_size: UVec2,
    logical_size: Vec2,
) {
    let (auto_width, auto_height) = match mode {
        PixelZoom::FitSize { width, height } => (Some(*width), Some(*height)),
        PixelZoom::FitWidth(width) => (Some(*width), None),
        PixelZoom::FitHeight(height) => (None, Some(*height)),
        PixelZoom::Fixed(..) => (None, None),
        PixelZoom::FitSmallerDim(_) => (None, None),
    };

    let scale_factor = (physical_size.x as f32) / logical_size.x;

    let mut viewport_width = physical_size.x;
    let mut viewport_x = 0;
    if let Some(target_width) = auto_width {
        let logical_target_width = (target_width as f32) * zoom;
        viewport_width = (scale_factor * logical_target_width) as u32;
        viewport_x = (scale_factor * (logical_size.x - logical_target_width)) as u32 / 2;
    }

    let mut viewport_height = physical_size.y;
    let mut viewport_y = 0;
    if let Some(target_height) = auto_height {
        let logicat_target_height = (target_height as f32) * zoom;
        viewport_height = (scale_factor * logicat_target_height) as u32;
        viewport_y = (scale_factor * (logical_size.y - logicat_target_height)) as u32 / 2;
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
