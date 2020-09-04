extern crate image;
extern crate piston_window;

use std::env;
use std::ops::Neg;

use image::{ImageBuffer, Rgba};
use log::{info, LevelFilter, trace, warn};
use piston_window::*;
use simplelog::{Config, TerminalMode, TermLogger};

static WINDOW_SIZE: u32 = 800;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Mid-Point Line", [WINDOW_SIZE; 2])
        .exit_on_esc(true)
        .graphics_api(OpenGL::V4_5)
        .resizable(false)
        .build()
        .unwrap();

    TermLogger::init(LevelFilter::Trace, Config::default(), TerminalMode::Mixed).unwrap();

    let args: Vec<String> = env::args().collect();
    info!("Running Program from {}", args[0]);


    let line: Line;

    if args.len() < 5 {
        warn!("Not Enough Argument. Using default values.");

        // Default
        let point_a = Point::from(-20, -70);
        let point_b = Point::from(20, 70);

        // Test Case 1
        // let point_a = Point::from(10, 10);
        // let point_b = Point::from(60, 50);

        // Test Case 2
        // let point_a = Point::from(10, -10);
        // let point_b = Point::from(60, -50);

        // Test Case 3
        // let point_a = Point::from(-30, -10);
        // let point_b = Point::from(-100, -40);

        line = Line::from(point_a, point_b);
    } else {
        let x1 = args[1].parse::<i32>().expect("Could Not Parse X1");
        let y1 = args[2].parse::<i32>().expect("Could Not Parse Y1");
        let x2 = args[3].parse::<i32>().expect("Could Not Parse X2");
        let y2 = args[4].parse::<i32>().expect("Could Not Parse Y2");

        let point_a = Point { x: x1, y: y1 };
        let point_b = Point { x: x2, y: y2 };
        line = Line::from(point_a, point_b);
    }

    let mut canvas = image::ImageBuffer::new(WINDOW_SIZE, WINDOW_SIZE);

    let mut texture_context = TextureContext {
        factory: window.factory.clone(),
        encoder: window.factory.create_command_buffer().into(),
    };

    let mut texture: G2dTexture =
        Texture::from_image(&mut texture_context, &canvas, &TextureSettings::new()).unwrap();


    draw_center_axis(&mut canvas);
    draw_line(line, &mut canvas);


    texture.update(&mut texture_context, &canvas).unwrap();

    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics, device| {
            // Update texture before rendering.
            texture_context.encoder.flush(device);


            image(&texture, context.transform, graphics);
        });
    }

    fn draw_center_axis(canvas: &mut ImageBuffer<Rgba<u8>, Vec<u8>>) {
        for i in 0..WINDOW_SIZE {
            canvas.put_pixel(i, WINDOW_SIZE / 2, Rgba([255, 0, 0, 255]));
            canvas.put_pixel(WINDOW_SIZE / 2, i, Rgba([255, 0, 0, 255]));
        }
    }

    fn draw_line(line: Line, canvas: &mut ImageBuffer<Rgba<u8>, Vec<u8>>) {
        let zero_line = line.convert_to_zone_zero();
        let mut line_points = calc_line_midpoint(zero_line);
        convert_zone(&mut line_points, &line.zone);

        for point in line_points {
            let pointer = Point::from(point[0], point[1]);
            draw_point(pointer.actual_x(), pointer.actual_y(), canvas);
        }
    }

    fn draw_point(x: u32, y: u32, canvas: &mut ImageBuffer<Rgba<u8>, Vec<u8>>) {
        canvas.put_pixel(x, y, Rgba([0, 255, 0, 255]));
    }

    fn calc_line_midpoint(zero_line: Line) -> Vec<[i32; 2]> {
        let delta_x = zero_line.end.x - zero_line.start.x;
        let delta_y = zero_line.end.y - zero_line.start.y;
        let mut d = 2 * delta_y - delta_x;
        let delta_ne = 2 * (delta_y - delta_x);
        let delta_e = 2 * delta_y;

        trace!("ΔX': {}", delta_x);
        trace!("ΔY': {}", delta_y);
        trace!("D: {}", d);
        trace!("ΔNE: {}", delta_ne);
        trace!("ΔE: {}", delta_e);

        let mut points: Vec<[i32; 2]> = vec![];

        let mut x = zero_line.start.x;
        let mut y = zero_line.start.y;
        while x <= zero_line.end.x {
            points.push([x, y]);
            x += 1;

            if d > 0 {
                y += 1;
                d += delta_ne;
            } else {
                d += delta_e;
            }
        }

        trace!("--- ZONE 0 Points ---");
        for point in &points {
            trace!("x:{} y:{}", point[0], point[1]);
        }
        points
    }

    enum Zone {
        ZERO,
        ONE,
        TWO,
        THREE,
        FOUR,
        FIVE,
        SIX,
        SEVEN,
    }

    impl Zone {
        pub fn name(&self) -> u8 {
            match self {
                Zone::ZERO => 0,
                Zone::ONE => 1,
                Zone::TWO => 2,
                Zone::THREE => 3,
                Zone::FOUR => 4,
                Zone::FIVE => 5,
                Zone::SIX => 6,
                Zone::SEVEN => 7,
            }
        }
    }

    struct Line {
        start: Point,
        end: Point,
        zone: Zone,
    }

    impl Line {
        pub fn from(starting_point: Point, ending_point: Point) -> Self {
            let line = Self {
                zone: Self::find_zone(&starting_point, &ending_point),
                start: starting_point,
                end: ending_point,
            };
            trace!("Zone: {}", line.zone.name());
            line
        }

        fn find_zone(start: &Point, end: &Point) -> Zone {
            trace!("X1: {} Y1: {}", start.x, start.y);
            trace!("X2: {} Y2: {}", end.x, end.y);

            let delta_x = end.x - start.x;
            let delta_y = end.y - start.y;

            trace!("ΔX: {}", delta_x);
            trace!("ΔY: {}", delta_y);


            if delta_x > 0 && delta_y > 0 && delta_x.abs() > delta_y.abs() {
                Zone::ZERO
            } else if delta_x > 0 && delta_y > 0 && delta_y.abs() > delta_x.abs() {
                Zone::ONE
            } else if delta_x < 0 && delta_y > 0 && delta_y.abs() > delta_x.abs() {
                Zone::TWO
            } else if delta_x < 0 && delta_y > 0 && delta_y.abs() < delta_x.abs() {
                Zone::THREE
            } else if delta_x < 0 && delta_y < 0 && delta_x.abs() > delta_y.abs() {
                Zone::FOUR
            } else if delta_x < 0 && delta_y < 0 && delta_x.abs() < delta_y.abs() {
                Zone::FIVE
            } else if delta_x > 0 && delta_y < 0 && delta_x.abs() < delta_y.abs() {
                Zone::SIX
            } else {
                Zone::SEVEN
            }
        }

        pub fn convert_to_zone_zero(&self) -> Line {
            let start: Point;
            let end: Point;

            trace!("Converting to Zone 0");
            match self.zone {
                Zone::ZERO => {
                    start = Point { x: self.start.x, y: self.start.y };
                    end = Point { x: self.end.x, y: self.end.y };
                }
                Zone::ONE => {
                    start = Point { x: self.start.y, y: self.start.x };
                    end = Point { x: self.end.y, y: self.end.x };
                }
                Zone::TWO => {
                    start = Point { x: self.start.y, y: self.start.x.neg() };
                    end = Point { x: self.end.y, y: self.end.x.neg() };
                }
                Zone::THREE => {
                    start = Point { x: self.start.x.neg(), y: self.start.y };
                    end = Point { x: self.end.x.neg(), y: self.end.y };
                }
                Zone::FOUR => {
                    start = Point { x: self.start.x.neg(), y: self.start.y.neg() };
                    end = Point { x: self.end.x.neg(), y: self.end.y.neg() };
                }
                Zone::FIVE => {
                    start = Point { x: self.start.y.neg(), y: self.start.x.neg() };
                    end = Point { x: self.end.y.neg(), y: self.end.x.neg() };
                }
                Zone::SIX => {
                    start = Point { x: self.start.y.neg(), y: self.start.x };
                    end = Point { x: self.end.y.neg(), y: self.end.x };
                }
                Zone::SEVEN => {
                    start = Point { x: self.start.x, y: self.start.y.neg() };
                    end = Point { x: self.end.x, y: self.end.y.neg() };
                }
            }

            trace!("X1': {} Y1': {}", start.x, start.y);
            trace!("X2': {} Y2': {}", end.x, end.y);

            Line { start, end, zone: Zone::ZERO }
        }
    }

    impl Point {
        pub fn from(x: i32, y: i32) -> Self {
            Self { x, y }
        }

        pub fn actual_x(&self) -> u32 {
            (self.x + (self::WINDOW_SIZE / 2) as i32) as u32
        }

        pub fn actual_y(&self) -> u32 {
            (self.y.neg() + (self::WINDOW_SIZE / 2) as i32) as u32
        }
    }

    struct Point {
        x: i32,
        y: i32,
    }

    fn convert_zone(points: &mut Vec<[i32; 2]>, zone: &Zone) {
        match zone {
            Zone::ZERO => {}
            Zone::ONE => {
                for point in points {
                    point.swap(0, 1);
                }
            }
            Zone::TWO => {
                for point in points {
                    let new_x = point[1].neg();
                    point[1] = point[0];
                    point[0] = new_x;
                }
            }
            Zone::THREE => {
                for point in points {
                    point[0] = point[0].neg();
                }
            }
            Zone::FOUR => {
                for point in points {
                    point[0] = point[0].neg();
                    point[1] = point[1].neg();
                }
            }
            Zone::FIVE => {
                for point in points {
                    let new_x = point[1].neg();
                    point[1] = point[0].neg();
                    point[0] = new_x;
                }
            }
            Zone::SIX => {
                for point in points {
                    let new_x = point[1];
                    point[1] = point[0].neg();
                    point[0] = new_x;
                }
            }
            Zone::SEVEN => {
                for point in points {
                    point[1] = point[1].neg();
                }
            }
        }
    }
}
