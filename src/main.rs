// use futures::executor::LocalPool;
// use futures::task::LocalSpawnExt;
use macroquad::prelude::*;
use noise::{NoiseFn, OpenSimplex, Perlin, PerlinSurflet, SuperSimplex, Value};
// use std::collections::VecDeque;
// use std::fs;
// use std::path::Path;
// use std::{
//     collections::HashMap,
//     path::PathBuf,
//     sync::mpsc::{self, Sender},
// };

// use tempdir::TempDir;
// use tileproc::args::GenTilesArgs;
// use tileproc::tiler::{gen_tiles_to_dir, generate_lods};
// mod options;
// use clap::Parser;

const LOD_FUZZYNESS: f32 = 1.0;

fn mandelbrot(x: f64, y: f64) -> f64 {
    let mut zx = 0.0;
    let mut zy = 0.0;
    let mut iteration = 0;
    let max_iterations = 100;

    while zx * zx + zy * zy < 4.0 && iteration < max_iterations {
        let tmp = zx * zx - zy * zy + x;
        zy = 2.0 * zx * zy + y;
        zx = tmp;
        iteration += 1;
    }

    if iteration < max_iterations {
        iteration as f64
    } else {
        0.0
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

fn screen_pos_to_world_pos(x: f32, y: f32, camera: &CameraSettings) -> (f32, f32) {
    let x_out = camera.x_offset + (x - screen_width() / 2.) / camera.zoom_multiplier;
    let y_out = camera.y_offset + (y - screen_height() / 2.) / camera.zoom_multiplier;
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
    x_offset: f32,
    y_offset: f32,
    zoom_multiplier: f32,
}

#[macroquad::main("TileView")]
async fn main() {
    // let args = options::Args::parse();

    let max_lod = 0;

    let two: f32 = 2.0;
    let default_zoom = 1.0 / two.powf(max_lod as f32 - 1.0);

    let mut camera = CameraSettings {
        x_offset: 0.,
        y_offset: 0.,
        zoom_multiplier: default_zoom,
    };

    let mut mouse_clicked_in_position: Option<(f32, f32)> = None;
    let mut clicked_in_x_offset: f32 = 0.0;
    let mut clicked_in_y_offset: f32 = 0.0;

    let target_fps = infer_target_fps().await;
    // let frame_time_limit = 1. / target_fps as f64;

    loop {
        // let frame_start_time = get_time();

        // react to key presses
        {
            let fps_speed_multiplier = 144. / target_fps as f32;
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

            let min_zoom = LOD_FUZZYNESS / two.powf(max_lod as f32 + 1.0);
            // let max_zoom = 20.0;
            let max_zoom = f32::MAX;

            // limit the zoom
            camera.zoom_multiplier = camera.zoom_multiplier.clamp(min_zoom, max_zoom);

            // zoom via scroll wheel
            let (_, mouse_scroll) = mouse_wheel();
            if mouse_scroll == 1.0 && camera.zoom_multiplier < max_zoom {
                // record mouse positions
                let mouse_screen_pos = mouse_position();
                let mouse_world_pos =
                    screen_pos_to_world_pos(mouse_screen_pos.0, mouse_screen_pos.1, &camera);

                // zoom in
                camera.zoom_multiplier += zoom_speed * 10.;

                // limit the zoom
                camera.zoom_multiplier = camera.zoom_multiplier.clamp(min_zoom, max_zoom);

                // center camera on where mouse was in world
                camera.x_offset = mouse_world_pos.0;
                camera.y_offset = mouse_world_pos.1;

                let screen_x_to_change = mouse_screen_pos.0 - screen_width() / 2.;
                let screen_y_to_change = mouse_screen_pos.1 - screen_height() / 2.;

                // move camera by screen_x_to_change
                camera.x_offset -= screen_x_to_change / camera.zoom_multiplier;
                camera.y_offset -= screen_y_to_change / camera.zoom_multiplier;
            } else if mouse_scroll == -1.0 && camera.zoom_multiplier > min_zoom {
                // record mouse positions
                let mouse_screen_pos = mouse_position();
                let mouse_world_pos =
                    screen_pos_to_world_pos(mouse_screen_pos.0, mouse_screen_pos.1, &camera);

                // zoom out
                camera.zoom_multiplier -= zoom_speed * 10.;

                // limit the zoom
                camera.zoom_multiplier = camera.zoom_multiplier.clamp(min_zoom, 20.);

                // center camera on where mouse was in world
                camera.x_offset = mouse_world_pos.0;
                camera.y_offset = mouse_world_pos.1;

                let screen_x_to_change = mouse_screen_pos.0 - screen_width() / 2.;
                let screen_y_to_change = mouse_screen_pos.1 - screen_height() / 2.;

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
                        let mouse_x_diff = cur_mouse_pos.0 - x.0;
                        camera.x_offset =
                            -(clicked_in_x_offset + mouse_x_diff / camera.zoom_multiplier);

                        // calc new y_offset
                        let mouse_y_diff = cur_mouse_pos.1 - x.1;
                        camera.y_offset =
                            -(clicked_in_y_offset + mouse_y_diff / camera.zoom_multiplier);
                    }
                };
            } else {
                mouse_clicked_in_position = None;
            }
        }
        // render
        {
            let mut image = Image::empty();
            image.width = screen_width() as u16;
            image.height = screen_height() as u16;
            image.bytes = vec![0; image.width as usize * image.height as usize * 4];

            // fill image with white
            for screen_x in 0..image.width as u32 {
                for screen_y in 0..image.height as u32 {
                    let (world_x, world_y) =
                        screen_pos_to_world_pos(screen_x as f32, screen_y as f32, &camera);

                    let color = perlin(world_x.into(), world_y.into());

                    image.set_pixel(
                        screen_x,
                        screen_y,
                        Color {
                            r: color as f32,
                            g: 0.,
                            b: 0.,
                            a: 1.0,
                        },
                    );
                }
            }
            // clear_background(LIGHTGRAY);
            let texture = Texture2D::from_image(&image);
            draw_texture(&texture, 0., 0., WHITE);
        }

        // draw text in top left corner
        {
            // let lod = lod_from_zoom(camera.zoom_multiplier, max_lod);
            // draw_text(
            //     &("fps: ".to_owned() + &get_fps().to_string()),
            //     20.0,
            //     20.0,
            //     30.0,
            //     WHITE,
            // );

            draw_text(
                &("zoom_multiplier: ".to_owned() + &camera.zoom_multiplier.to_string()),
                20.0,
                40.0,
                30.0,
                WHITE,
            );

            // draw_text(
            //     &("LOD: ".to_owned() + &lod.to_string()),
            //     20.0,
            //     60.0,
            //     30.0,
            //     WHITE,
            // );

            // draw_text(
            //     &("rendered_tiles: ".to_owned() + &num_rendered_tiles.to_string()),
            //     20.0,
            //     80.0,
            //     30.0,
            //     WHITE,
            // );

            let mouse = mouse_position();
            let mouse_coord = screen_pos_to_world_pos(mouse.0, mouse.1, &camera);
            draw_text(
                &("mouse.x: ".to_owned() + &mouse_coord.0.to_string()),
                20.0,
                100.0,
                30.0,
                WHITE,
            );

            draw_text(
                &("mouse.y: ".to_owned() + &mouse_coord.1.to_string()),
                20.0,
                120.0,
                30.0,
                WHITE,
            );
        }

        next_frame().await
    }
}
