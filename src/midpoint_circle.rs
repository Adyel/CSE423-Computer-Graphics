extern crate image;
extern crate piston_window;

use std::env;
use std::ops::Neg;

use image::{ImageBuffer, Rgba};
use log::{info, LevelFilter, warn};
use piston_window::*;
use simplelog::{Config, TerminalMode, TermLogger};

static WINDOW_SIZE: u32 = 800;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Mid-Point Circle", [WINDOW_SIZE; 2])
        .exit_on_esc(true)
        .graphics_api(OpenGL::V4_5)
        .resizable(false)
        .build()
        .unwrap();

    TermLogger::init(LevelFilter::Trace, Config::default(), TerminalMode::Mixed).unwrap();

    let args: Vec<String> = env::args().collect();
    info!("Running Program from {}", args[0]);


    let radius : i16;

    if args.len() < 2 {
        warn!("Not Enough Argument. Using default values.");
        radius = 350;

    } else {
        let r = args[1].parse::<i16>().expect("Could Not Parse radius");
        if r > 350 {
            radius = 350;
        } else {
            radius = r;
        }
    }

    let mut canvas = image::ImageBuffer::new(WINDOW_SIZE, WINDOW_SIZE);

    let mut texture_context = TextureContext {
        factory: window.factory.clone(),
        encoder: window.factory.create_command_buffer().into(),
    };

    let mut texture: G2dTexture =
        Texture::from_image(&mut texture_context, &canvas, &TextureSettings::new()).unwrap();


    draw_8way_axis(&mut canvas);
    draw_flower(0,0, radius, &mut canvas);


    texture.update(&mut texture_context, &canvas).unwrap();

    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics, device| {
            // Update texture before rendering.
            texture_context.encoder.flush(device);
            image(&texture, context.transform, graphics);
        });
    }
}

fn draw_flower(x: i16, y: i16, radius: i16, canvas: &mut ImageBuffer<Rgba<u8>, Vec<u8>>) {
    draw_circle(x,y, radius, canvas);
    draw_child_circles(x, y, radius, canvas);
}

fn draw_child_circles(x: i16, y: i16, radius: i16, canvas: &mut ImageBuffer<Rgba<u8>, Vec<u8>>) {
    draw_circle(x + radius / 2, y, radius / 2, canvas);
    draw_circle((x + radius / 2).neg(), y, radius / 2, canvas);
    draw_circle(x, y + radius / 2, radius / 2, canvas);
    draw_circle(x, (y + radius / 2).neg(), radius / 2, canvas);

    let _x = ((radius - x) as f32 / 2.82) as i16;
    let _y = ((radius - y) as f32 / 2.82) as i16;

    draw_circle(_x, _y, radius / 2, canvas);
    draw_circle(_x.neg(), _y, radius / 2, canvas);
    draw_circle(_x, _y.neg(), radius / 2, canvas);
    draw_circle(_x.neg(), _y.neg(), radius / 2, canvas);
}

fn draw_8way_axis(canvas: &mut ImageBuffer<Rgba<u8>, Vec<u8>>) {
    for i in 0..WINDOW_SIZE {
        let red = Rgba([255, 0, 0, 255]);
        canvas.put_pixel(i, WINDOW_SIZE / 2, red);
        canvas.put_pixel(WINDOW_SIZE / 2, i, red);
        canvas.put_pixel(i,i, red);
        canvas.put_pixel(WINDOW_SIZE - i - 1, i, red);
    }
}

fn add_with_mirror_points(x: i16, y: i16, circle_points: &mut Vec<[i16;2]>){
    circle_points.push([x,y]);
    circle_points.push([y,x]);
    circle_points.push([y, x.neg()]);
    circle_points.push([x, y.neg()]);
    circle_points.push([x.neg(), y.neg()]);
    circle_points.push([y.neg(), x.neg()]);
    circle_points.push([y.neg(), x]);
    circle_points.push([x.neg(), y]);
}

fn draw_circle(x: i16, y: i16, radius: i16, canvas: &mut ImageBuffer<Rgba<u8>, Vec<u8>>) {

    let circle_points = calc_circle_points(x, y, radius);

    for point in &circle_points {
        draw_point(point[0],point[1] , canvas);
    }
}


fn draw_point(x: u32, y: u32, canvas: &mut ImageBuffer<Rgba<u8>, Vec<u8>>) {
    let green = Rgba([0, 255, 0, 255]);
    canvas.put_pixel(x, y, green);
}

fn calc_circle_points(x: i16, y: i16, radius: i16) -> Vec<[u32;2]>{
    let mut circle_points: Vec<[i16;2]> = vec![];

    let mut d:i16 = 1 - radius;
    let mut _x = 0;
    let mut _y = radius as i16;
    add_with_mirror_points(_x, _y, &mut circle_points);
    while _x < _y {
        if d < 0 {
            d = d + 2 * _x + 3;
            _x += 1;
        }else {
            d = d + 2 * _x - 2 * _y + 5;
            _x += 1;
            _y -= 1;
        }
        add_with_mirror_points(_x, _y, &mut circle_points);
    }

    for point in &mut circle_points {
        point[0] += x;
        point[1] += y;
    }

    conv_actual_point(circle_points)
}

fn conv_actual_point(points: Vec<[i16;2]>) -> Vec<[u32;2]>{
    let mut actual_points: Vec<[u32; 2]> = vec![];

    for point in points {
        let x = (point[0] + (WINDOW_SIZE / 2) as i16) as u32;
        let y = (point[1].neg() + (WINDOW_SIZE / 2) as i16) as u32;
        actual_points.push([x,y]);
    }

    actual_points
}