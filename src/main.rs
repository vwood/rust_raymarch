extern crate image;

use image::ImageBuffer;
use rayon::prelude::*;
use serde_json::Value;
use std::env;
use std::error::Error;
use std::fs;

mod raymarch;
mod scene;
mod vector;

fn process_file(input_filename: &str) -> Result<Value, Box<dyn Error>> {
    let data = fs::read_to_string(input_filename)?;

    let v = serde_json::from_str(&data)?;

    Ok(v)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut input_filename = "default.json";
    let mut output_filename = "default.png";

    if args.len() > 1 {
        input_filename = &args[1];
        if args.len() > 2 {
            output_filename = &args[2];
        }
    }

    println!("Input: {}", input_filename);

    let result = process_file(input_filename);

    let value = match result {
        Ok(v) => v,
        Err(error) => {
            println!("Error: {}", error);
            return;
        }
    };

    let sdf = value["scene"].as_str().unwrap_or("");

    println!("Ouput: {}", output_filename);

    let width = 800;
    let height = 600;

    let pixels: Vec<u8> = (0..width * height)
        .into_par_iter()
        .map(|i| {
            let x = i % width;
            let y = i / width;

            let x_pos = ((x as f32) / (width as f32) - 0.5) * 0.8;
            let y_pos = (0.5 - (y as f32) / (height as f32)) * 0.6;

            let dir = vector::Vec3::new(-0.2 - y_pos, x_pos, 1.0).normalize();

            let (r, g, b) = raymarch::march(
                match sdf {
                    "torus" => &scene::torus_scene_sdf,
                    "mandlebulb" => &scene::mandlebulb_scene_sdf,
                    "gyroid" => &scene::gyroid_scene_sdf,
                    "example" => &scene::example_scene_sdf,
                    _ => &scene::example_scene_sdf,
                },
                vector::Vec3::new(0.5, 0.5, -2.0),
                dir,
                100,
                255.0,
                0.001,
            );

            let r = (255.0 * r) as u8;
            let g = (255.0 * g) as u8;
            let b = (255.0 * b) as u8;
            vec![r, g, b]
        })
        .flatten()
        .collect();

    println!("vec_length: {}", width * height * 3);
    println!("vec_length: {}", pixels.len());

    let img = ImageBuffer::<image::Rgb<u8>, _>::from_vec(width, height, pixels).unwrap();

    /*
        let mut img = ImageBuffer::new(width, height);

        for (x, y, pixel) in img.enumerate_pixels_mut() {
            // generate primary ray

            let x_pos = ((x as f32) / (width as f32) - 0.5) * 0.8;
            let y_pos = (0.5 - (y as f32) / (height as f32)) * 0.6;

            let dir = vector::Vec3::new(-0.2 - y_pos, x_pos, 1.0).normalize();

            let (r, g, b) = raymarch::march(
                match sdf {
                    "torus" => &scene::torus_scene_sdf,
                    "mandlebulb" => &scene::mandlebulb_scene_sdf,
                    "gyroid" => &scene::gyroid_scene_sdf,
                    "example" => &scene::example_scene_sdf,
                    _ => &scene::example_scene_sdf,
                },
                vector::Vec3::new(0.5, 0.5, -2.0),
                dir,
                100,
                255.0,
                0.001,
            );

            let r = (255.0 * r) as u8;
            let g = (255.0 * g) as u8;
            let b = (255.0 * b) as u8;
            *pixel = image::Rgb([r, g, b]);
        }
    */

    img.save(output_filename).unwrap();
}
