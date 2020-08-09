extern crate piston_window;

use piston_window::*;
use std::env;
use std::option::Option::Some;

use log::{info, warn};
use simplelog::*;

const WINDOW_SIZE: u32 = 512;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("rectangle", [WINDOW_SIZE; 2])
        .exit_on_esc(true)
        .graphics_api(OpenGL::V4_5)
        .build()
        .unwrap();

    TermLogger::init(LevelFilter::Trace, Config::default(), TerminalMode::Mixed).unwrap();

    let args: Vec<String> = env::args().collect();
    info!("Running Program from {}", args[0]);

    let height: u32;
    let width: u32;

    if args.len() < 3 {
        warn!("Not Enough Argument. Using default values.");
        height = 200;
        width = 250;
    } else {
        height = args[1].parse::<u32>().expect("Could Not Parse height");
        width = args[2].parse::<u32>().expect("Could Not Parse width");
    }

    create_rectangle(height, width, &mut window);
}

fn create_rectangle(height: u32, width: u32, window: &mut PistonWindow) {
    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics, _device| {
            clear([1.0; 4], graphics);

            let center = (WINDOW_SIZE / 2) as f64;
            let h_dist_center = (height / 2) as f64;
            let w_dist_center = (width / 2) as f64;

            line(
                [1.0, 0.0, 0.0, 1.0],
                2.0,
                [
                    center - w_dist_center,
                    center - h_dist_center,
                    center + w_dist_center,
                    center - h_dist_center,
                ],
                context.transform,
                graphics,
            );

            line(
                [1.0, 0.0, 0.0, 1.0],
                2.0,
                [
                    center - w_dist_center,
                    center + h_dist_center,
                    center + w_dist_center,
                    center + h_dist_center,
                ],
                context.transform,
                graphics,
            );

            line(
                [1.0, 0.0, 0.0, 1.0],
                2.0,
                [
                    center - w_dist_center,
                    center - h_dist_center,
                    center - w_dist_center,
                    center + h_dist_center,
                ],
                context.transform,
                graphics,
            );

            line(
                [1.0, 0.0, 0.0, 1.0],
                2.0,
                [
                    center + w_dist_center,
                    center - h_dist_center,
                    center + w_dist_center,
                    center + h_dist_center,
                ],
                context.transform,
                graphics,
            );
        });
    }
}
