extern crate piston_window;

use std::env;
use std::option::Option::Some;

use log::{info, trace, warn};
use piston_window::*;
use simplelog::*;

static WINDOW_SIZE: u32 = 512;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("DDA Algo", [WINDOW_SIZE; 2])
        .exit_on_esc(true)
        .graphics_api(OpenGL::V4_5)
        .automatic_close(true)
        .resizable(true)
        .vsync(true)
        .build()
        .unwrap();

    TermLogger::init(LevelFilter::Trace, Config::default(), TerminalMode::Mixed).unwrap();


    let args: Vec<String> = env::args().collect();
    info!("Running Program from {}", args[0]);

    let point_a: Point;
    let point_b: Point;

    if args.len() < 5 {
        warn!("Not Enough Argument. Using default values.");
        point_a = Point { x: 20, y: 20 };
        point_b = Point { x: 100, y: 100 };
    } else {
        let x1 = args[1].parse::<i32>().expect("Could Not Parse X1");
        let y1 = args[2].parse::<i32>().expect("Could Not Parse Y1");
        let x2 = args[3].parse::<i32>().expect("Could Not Parse X2");
        let y2 = args[4].parse::<i32>().expect("Could Not Parse Y2");

        point_a = Point { x: x1, y: y1 };
        point_b = Point { x: x2, y: y2 };
    }


    draw_line_dda(point_a, point_b, &mut window);
}

struct Point {
    x: i32,
    y: i32,
}

fn draw_line_dda(point_a: Point, point_b: Point, window: &mut PistonWindow) {
    let result = calculate_points(point_a, point_b);


    for r in &result {
        trace!("x:{} y:{}", r[0], r[1]);
    }


    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics, _device| {
            clear([0.0; 4], graphics);

            for r in &result {
                draw_point(r[0], r[1], context, graphics);
            }
        });
    };
}

fn calculate_points(point_a: Point, point_b: Point) -> Vec<[f64; 2]> {
    let mut points: Vec<[f64; 2]> = vec![];

    let delta_x = point_a.x - point_b.x;
    let delta_y = point_a.y - point_b.y;
    let m = delta_y as f64 / delta_x as f64;

    let mut x;
    let mut y;
    let limit;

    if m <= 1.0 && m > -1.0 {
        if point_a.x < point_b.x {
            x = point_a.x as f64;
            y = point_a.y as f64;
            limit = point_b.x + 1;
        } else {
            x = point_b.x as f64;
            y = point_b.y as f64;
            limit = point_a.x + 1;
        }

        while x < limit as f64 {
            points.push([x, y.round()]);
            x += 1.0;
            y += m;
        }
    } else {
        if point_a.y < point_b.y {
            y = point_a.y as f64;
            x = point_a.x as f64;
            limit = point_b.y + 1;
        } else {
            y = point_b.y as f64;
            x = point_b.x as f64;
            limit = point_a.y + 1;
        }

        while y < limit as f64 {
            points.push([x.round(), y]);
            y += 1.0;
            x += 1.0 / m;
        }
    }
    points
}

fn draw_point(x: f64, y: f64, context: Context, graphics: &mut G2d) {
    let red = [1.0, 0.0, 0.0, 1.0];

    rectangle(
        red,
        [x, y, 1.0, 1.0],
        context.transform,
        graphics,
    )
}
