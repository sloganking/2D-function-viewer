use std::sync::mpsc;

use macroquad::prelude::*;
use noise::{NoiseFn, OpenSimplex, Perlin, PerlinSurflet, SuperSimplex, Value};

const LOD_FUZZYNESS: f64 = 1.0;

// fn mandelbrot(x: f64, y: f64) -> f64 {
//     let mut zx = 0.0;
//     let mut zy = 0.0;
//     let mut iteration = 0;
//     let max_iterations = 100;

//     while zx * zx + zy * zy < 4.0 && iteration < max_iterations {
//         let tmp = zx * zx - zy * zy + x;
//         zy = 2.0 * zx * zy + y;
//         zx = tmp;
//         iteration += 1;
//     }

//     if iteration < max_iterations {
//         iteration as f64
//     } else {
//         0.0
//     }
// }

// fn value_to_color(value: f64) -> Color {
//     let clamped = value.clamp(0.0, 1.0);
//     let h = clamped * 360.0; // Hue goes from 0 to 360
//     let s = 1.0;
//     let v = 1.0;

//     let c = v * s;
//     let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
//     let m = v - c;

//     let (r, g, b) = match h as u32 {
//         0..=60 => (c, x, 0.0),
//         61..=120 => (x, c, 0.0),
//         121..=180 => (0.0, c, x),
//         181..=240 => (0.0, x, c),
//         241..=300 => (x, 0.0, c),
//         _ => (c, 0.0, x),
//     };

//     // (r + m, g + m, b + m)

//     Color {
//         r: r as f32 + m as f32,
//         g: g as f32 + m as f32,
//         b: b as f32 + m as f32,
//         a: 1.0,
//     }
// }

fn value_to_color(value: f64) -> Color {
    let clamped = value.clamp(0.0, 1.0);
    let h = clamped * 360.0; // Hue goes from 0 to 360
    let s = 1.0;
    let v = 1.0;

    let c = v * s;
    let h_prime = h / 60.0;
    let x = c * (1.0 - ((h_prime % 2.0) - 1.0).abs());
    let m = v - c;

    let (r1, g1, b1) = if h_prime >= 0.0 && h_prime < 1.0 {
        (c, x, 0.0)
    } else if h_prime >= 1.0 && h_prime < 2.0 {
        (x, c, 0.0)
    } else if h_prime >= 2.0 && h_prime < 3.0 {
        (0.0, c, x)
    } else if h_prime >= 3.0 && h_prime < 4.0 {
        (0.0, x, c)
    } else if h_prime >= 4.0 && h_prime < 5.0 {
        (x, 0.0, c)
    } else if h_prime >= 5.0 && h_prime < 6.0 {
        (c, 0.0, x)
    } else {
        (0.0, 0.0, 0.0)
    };

    Color {
        r: (r1 + m) as f32,
        g: (g1 + m) as f32,
        b: (b1 + m) as f32,
        a: 1.0,
    }
}

fn mandelbrot(x: f64, y: f64) -> f64 {
    let mut zx = 0.0;
    let mut zy = 0.0;
    let mut iteration = 0;
    let max_iterations = 100;

    while zx * zx + zy * zy < 4.0 && iteration < max_iterations {
        let temp = zx * zx - zy * zy + x;
        zy = 2.0 * zx * zy + y;
        zx = temp;
        iteration += 1;
    }

    if iteration == max_iterations {
        0.0 // Point is in the Mandelbrot set
    } else {
        iteration as f64 / max_iterations as f64 // Normalize the iteration count
    }
}

fn burning_ship(x: f64, y: f64) -> f64 {
    let max_iterations = 1000;
    let mut zx = 0.0;
    let mut zy = 0.0;
    let mut iteration = 0;

    while zx * zx + zy * zy < 4.0 && iteration < max_iterations {
        let temp_zx = zx * zx - zy * zy + x;
        zy = 2.0 * zx.abs() * zy.abs() + y;
        zx = temp_zx.abs();
        iteration += 1;
    }

    // Normalize the result to a range between 0 and 1
    iteration as f64 / max_iterations as f64
}

fn julia(cx: f64, cy: f64, x: f64, y: f64) -> f64 {
    let mut zx = x;
    let mut zy = y;
    let max_iterations = 100;
    let mut iteration = 0;

    while zx * zx + zy * zy < 4.0 && iteration < max_iterations {
        let tmp = zx * zx - zy * zy + cx;
        zy = 2.0 * zx * zy + cy;
        zx = tmp;
        iteration += 1;
    }

    if iteration < max_iterations {
        return iteration as f64;
    } else {
        return 0.0;
    }
}

fn perlin(x: f64, y: f64) -> f64 {
    let perlin = OpenSimplex::new(1000000);

    let scale_factor = 2.0;

    let amplitude_factor = 0.5; // The factor by which to reduce the amplitude of each successive octave

    let mut scale = 1.0;
    let mut amplitude = 1.0;

    let mut combined_noise = 0.0;
    for _ in 0..4 {
        combined_noise += perlin.get([x * scale, y * scale]) * amplitude;
        scale *= scale_factor;
        amplitude *= amplitude_factor;
    }

    // println!("combined_noise: {}", combined_noise);

    // squish -1 - 1 into 0-1;
    combined_noise += 1.;
    combined_noise /= 2.;

    combined_noise
}

fn _wave_pattern(x: f64, y: f64) -> f64 {
    (x.sin() + y.sin()) / 2.0
}

fn _circular_pattern(x: f64, y: f64) -> f64 {
    ((x * x + y * y).sqrt()).sin()
}

// fn world_pos_to_screen_pos(x: f32, y: f32, camera: &CameraSettings) -> (f32, f32) {
//     let out_x = screen_width() / 2. + ((x - camera.x_offset) * camera.zoom_multiplier);
//     let out_y = screen_height() / 2. + ((y - camera.y_offset) * camera.zoom_multiplier);
//     (out_x, out_y)
// }

fn screen_pos_to_world_pos(x: f64, y: f64, camera: &CameraSettings) -> (f64, f64) {
    let x_out = camera.x_offset + (x - screen_width() as f64 / 2.) / camera.zoom_multiplier;
    let y_out = camera.y_offset + (y - screen_height() as f64 / 2.) / camera.zoom_multiplier;
    (x_out, y_out)
}

fn median(numbers: &mut [i32]) -> i32 {
    numbers.sort();
    let mid = numbers.len() / 2;
    numbers[mid]
}

async fn infer_target_fps() -> i32 {
    let fps_test_start_time = get_time();
    let mut fps_records: Vec<i32> = Vec::new();
    while get_time() - fps_test_start_time < 0.5 {
        fps_records.push(get_fps());
        next_frame().await;
    }
    median(&mut fps_records)
}

struct CameraSettings {
    x_offset: f64,
    y_offset: f64,
    zoom_multiplier: f64,
}

fn julia_normalized(cx: f64, cy: f64, x: f64, y: f64) -> f64 {
    let mut zx = x;
    let mut zy = y;
    let max_iterations = 100;
    let mut iteration = 0;

    while zx * zx + zy * zy < 4.0 && iteration < max_iterations {
        let tmp = zx * zx - zy * zy + cx;
        zy = 2.0 * zx * zy + cy;
        zx = tmp;
        iteration += 1;
    }

    if iteration < max_iterations {
        return iteration as f64 / max_iterations as f64;
    } else {
        return 0.0;
    }
}

#[macroquad::main("TileView")]
async fn main() {
    // let args = options::Args::parse();

    let max_lod = 0;

    let mut cx = -0.123;
    let mut cy = 0.745;

    let cx_change = 0.0001;

    let two: f64 = 2.0;
    let default_zoom = 1.0 / two.powf(max_lod as f64 - 1.0);

    let mut camera = CameraSettings {
        x_offset: 0.,
        y_offset: 0.,
        zoom_multiplier: default_zoom,
    };

    let mut mouse_clicked_in_position: Option<(f32, f32)> = None;
    let mut clicked_in_x_offset: f64 = 0.0;
    let mut clicked_in_y_offset: f64 = 0.0;

    let target_fps = infer_target_fps().await;
    // let frame_time_limit = 1. / target_fps as f64;

    loop {
        // let frame_start_time = get_time();

        // react to key presses
        {
            let fps_speed_multiplier = 144. / target_fps as f64;
            let speed = if is_key_down(KeyCode::LeftShift) {
                20. / camera.zoom_multiplier * fps_speed_multiplier
            } else {
                5. / camera.zoom_multiplier * fps_speed_multiplier
            };

            if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
                camera.x_offset += speed;
            }
            if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
                camera.x_offset -= speed;
            }
            if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
                camera.y_offset -= speed;
            }
            if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
                camera.y_offset += speed;
            }

            let zoom_speed = if is_key_down(KeyCode::LeftShift) {
                camera.zoom_multiplier / 100. * 4. * fps_speed_multiplier
            } else {
                camera.zoom_multiplier / 100. * fps_speed_multiplier
            };

            // zoom via buttons
            if is_key_down(KeyCode::E) {
                camera.zoom_multiplier += zoom_speed;
            }
            if is_key_down(KeyCode::Q) {
                camera.zoom_multiplier -= zoom_speed;
            }

            let min_zoom = LOD_FUZZYNESS / two.powf(max_lod as f64 + 1.0);
            // let max_zoom = 20.0;
            let max_zoom = f64::MAX;

            // limit the zoom
            camera.zoom_multiplier = camera.zoom_multiplier.clamp(min_zoom, max_zoom);

            // zoom via scroll wheel
            let (_, mouse_scroll) = mouse_wheel();
            if mouse_scroll == 1.0 && camera.zoom_multiplier < max_zoom {
                // record mouse positions
                let mouse_screen_pos = mouse_position();
                let mouse_world_pos = screen_pos_to_world_pos(
                    mouse_screen_pos.0.into(),
                    mouse_screen_pos.1.into(),
                    &camera,
                );

                // zoom in
                camera.zoom_multiplier += zoom_speed * 10.;

                // limit the zoom
                camera.zoom_multiplier = camera.zoom_multiplier.clamp(min_zoom, max_zoom);

                // center camera on where mouse was in world
                camera.x_offset = mouse_world_pos.0;
                camera.y_offset = mouse_world_pos.1;

                let screen_x_to_change: f64 =
                    mouse_screen_pos.0 as f64 - screen_width() as f64 / 2.;
                let screen_y_to_change: f64 =
                    mouse_screen_pos.1 as f64 - screen_height() as f64 / 2.;

                // move camera by screen_x_to_change
                camera.x_offset -= screen_x_to_change / camera.zoom_multiplier;
                camera.y_offset -= screen_y_to_change / camera.zoom_multiplier;
            } else if mouse_scroll == -1.0 && camera.zoom_multiplier > min_zoom {
                // record mouse positions
                let mouse_screen_pos = mouse_position();
                let mouse_world_pos = screen_pos_to_world_pos(
                    mouse_screen_pos.0.into(),
                    mouse_screen_pos.1.into(),
                    &camera,
                );

                // zoom out
                camera.zoom_multiplier -= zoom_speed * 10.;

                // limit the zoom
                camera.zoom_multiplier = camera
                    .zoom_multiplier
                    .clamp(min_zoom.into(), max_zoom.into());

                // center camera on where mouse was in world
                camera.x_offset = mouse_world_pos.0;
                camera.y_offset = mouse_world_pos.1;

                let screen_x_to_change: f64 =
                    mouse_screen_pos.0 as f64 - screen_width() as f64 / 2.;
                let screen_y_to_change: f64 =
                    mouse_screen_pos.1 as f64 - screen_height() as f64 / 2.;

                // move camera by screen_x_to_change
                camera.x_offset -= screen_x_to_change / camera.zoom_multiplier;
                camera.y_offset -= screen_y_to_change / camera.zoom_multiplier;
            }

            // mouse drag screen
            if is_mouse_button_down(MouseButton::Left) {
                match mouse_clicked_in_position {
                    None => {
                        mouse_clicked_in_position = Some(mouse_position());
                        clicked_in_x_offset = -camera.x_offset;
                        clicked_in_y_offset = -camera.y_offset;
                    }
                    Some(x) => {
                        let cur_mouse_pos = mouse_position();

                        // calc new x_offset
                        let mouse_x_diff: f64 = cur_mouse_pos.0 as f64 - x.0 as f64;
                        camera.x_offset =
                            -(clicked_in_x_offset + mouse_x_diff / camera.zoom_multiplier);

                        // calc new y_offset
                        let mouse_y_diff: f64 = cur_mouse_pos.1 as f64 - x.1 as f64;
                        camera.y_offset =
                            -(clicked_in_y_offset + mouse_y_diff / camera.zoom_multiplier);
                    }
                };
            } else {
                mouse_clicked_in_position = None;
            }
        }

        fn screen_pos_to_world_pos_thread_safe(
            x: f64,
            y: f64,
            camera: &CameraSettings,
            screen_width: f64,
            screen_height: f64,
        ) -> (f64, f64) {
            let x_out = camera.x_offset + (x - screen_width / 2.) / camera.zoom_multiplier;
            let y_out = camera.y_offset + (y - screen_height / 2.) / camera.zoom_multiplier;
            (x_out, y_out)
        }

        use rayon::prelude::*;
        // render
        {
            let mut image = Image::empty();
            use rayon::prelude::*;

            // On the main thread
            image.width = screen_width() as u16;
            image.height = screen_height() as u16;
            image.bytes = vec![0; image.width as usize * image.height as usize * 4];

            let width = image.width as u32;
            let height = image.height as u32;
            let num_pixels = (width * height) as usize;
            // println!("num_pixels: {}", num_pixels);

            let screen_width = screen_width() as f64;
            let screen_height = screen_height() as f64;

            // Create a buffer to hold the computed colors
            let mut pixels = vec![Color::default(); num_pixels];

            // Compute pixel colors in parallel
            pixels.par_iter_mut().enumerate().for_each(|(i, pixel)| {
                let screen_x = (i as u32) % width;
                let screen_y = (i as u32) / width;

                // Use the refactored function without Macroquad dependencies
                let (world_x, world_y) = screen_pos_to_world_pos_thread_safe(
                    screen_x as f64,
                    screen_y as f64,
                    &camera,
                    screen_width,
                    screen_height,
                );

                let brightness_value = burning_ship(world_x, world_y);

                let color = value_to_color(brightness_value);

                *pixel = color;
            });

            // After the parallel loop, update the image on the main thread
            for (i, pixel) in pixels.into_iter().enumerate() {
                let screen_x = (i as u32) % width;
                let screen_y = (i as u32) / width;
                // println!("screen_x: {}, screen_y: {}", screen_x, screen_y);
                image.set_pixel(screen_x, screen_y, pixel);
            }
            // clear_background(LIGHTGRAY);
            let texture = Texture2D::from_image(&image);
            draw_texture(&texture, 0., 0., WHITE);
        }

        // cy += cx_change;
        // cx += cx_change;

        // draw text in top left corner
        {
            // let lod = lod_from_zoom(camera.zoom_multiplier, max_lod);
            draw_text(
                &("fps: ".to_owned() + &get_fps().to_string()),
                20.0,
                20.0,
                30.0,
                WHITE,
            );

            draw_text(
                &("zoom_multiplier: ".to_owned() + &camera.zoom_multiplier.to_string()),
                20.0,
                20.0 * 2.,
                30.0,
                WHITE,
            );

            let mouse = mouse_position();
            let mouse_coord = screen_pos_to_world_pos(mouse.0.into(), mouse.1.into(), &camera);
            draw_text(
                &("mouse.x: ".to_owned() + &mouse_coord.0.to_string()),
                20.0,
                20. * 3.,
                30.0,
                WHITE,
            );

            draw_text(
                &("mouse.y: ".to_owned() + &mouse_coord.1.to_string()),
                20.0,
                20. * 4.,
                30.0,
                WHITE,
            );
        }

        next_frame().await
    }
}
