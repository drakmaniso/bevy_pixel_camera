use super::*;
use bevy::math::Vec4;

#[test]
fn projection_test_around_240x240() {
    for width in 230..250 {
        for height in 230..250 {
            for zoom in 1..5 {
                projection_test_for_width_height_zoom(width, height, zoom);
            }
        }
    }
}

#[test]
fn projection_test_around_1280x720() {
    for width in 1275..1285 {
        for height in 715..725 {
            for zoom in 2..6 {
                projection_test_for_width_height_zoom(width, height, zoom);
            }
        }
    }
}

#[test]
fn projection_test_around_1920x1080() {
    for width in 1915..1925 {
        for height in 1075..1085 {
            for zoom in 2..6 {
                projection_test_for_width_height_zoom(width, height, zoom);
            }
        }
    }
}

#[test]
fn projection_test_around_3840x2160() {
    for width in 3836..3844 {
        for height in 2156..2164 {
            for zoom in 3..6 {
                projection_test_for_width_height_zoom(width, height, zoom);
            }
        }
    }
}

const MIN_WIDTH: i32 = 240;
const MIN_HEIGHT: i32 = 240;
const MAX_WIDTH: i32 = 3840;
const MAX_HEIGHT: i32 = 2160;

#[test]
#[ignore]
fn expensive_projection_test_for_zoom_1() {
    expensive_projection_test_for_zoom(1);
}

#[test]
#[ignore]
fn expensive_projection_test_for_zoom_2() {
    expensive_projection_test_for_zoom(2);
}

#[test]
#[ignore]
fn expensive_projection_test_for_zoom_3() {
    expensive_projection_test_for_zoom(3);
}

#[test]
#[ignore]
fn expensive_projection_test_for_zoom_4() {
    expensive_projection_test_for_zoom(4);
}

#[test]
#[ignore]
fn expensive_projection_test_for_zoom_5() {
    expensive_projection_test_for_zoom(5);
}

#[test]
#[ignore]
fn expensive_projection_test_for_zoom_6() {
    expensive_projection_test_for_zoom(6);
}

#[test]
#[ignore]
fn expensive_projection_test_for_zoom_7() {
    expensive_projection_test_for_zoom(7);
}

#[test]
#[ignore]
fn expensive_projection_test_for_zoom_8() {
    expensive_projection_test_for_zoom(8);
}

fn expensive_projection_test_for_zoom(zoom: i32) {
    for width in MIN_WIDTH..MAX_WIDTH {
        let min_height = MIN_HEIGHT.max(width / 2);
        let max_height = MAX_HEIGHT.min(width);
        println!(
            "// Testing resolutions {}x[{}..{}] at zoom {}",
            width, min_height, max_height, zoom
        );
        for height in min_height..max_height {
            projection_test_for_width_height_zoom(width, height, zoom);
        }
    }
}

fn projection_test_for_width_height_zoom(width: i32, height: i32, zoom: i32) {
    let mut window_projection = bevy::render::camera::OrthographicProjection::default();
    let mut virtual_projection = PixelProjection {
        zoom: zoom,
        ..Default::default()
    };
    virtual_projection.update(width as f32, height as f32);
    window_projection.update(width as f32, height as f32);

    let virtual_matrix = virtual_projection.get_projection_matrix();
    let window_matrix = window_projection.get_projection_matrix();

    let virtual_width = width / zoom;
    let virtual_height = height / zoom;
    for x in -(virtual_width / 2)..(virtual_width - virtual_width / 2) {
        for y in -(virtual_height / 2)..(virtual_height - virtual_height / 2) {
            let virtual_pixel = Vec4::new(x as f32, y as f32, 0.0, 1.0);
            let expected_window_pixel = Vec4::new(
                (virtual_pixel.x + ((virtual_width / 2) as f32)) * (zoom as f32),
                (virtual_pixel.y + ((virtual_height / 2) as f32)) * (zoom as f32),
                0.0,
                1.0,
            );

            let virtual_pos = virtual_matrix * virtual_pixel;
            let _window_pos = window_matrix * expected_window_pixel;
            let computed_window_pixel = Vec4::new(
                (virtual_pos.x + 1.0) * (width as f32) * 0.5,
                (virtual_pos.y + 1.0) * (height as f32) * 0.5,
                0.0,
                1.0,
            );

            assert_eq!(
                computed_window_pixel.round(),
                expected_window_pixel,
                "Resolution {}x{} Zoom {} Pixel ({}, {})",
                width,
                height,
                zoom,
                x,
                y,
            );
        }
    }
}
