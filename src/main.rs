#[macro_use]
extern crate clap;

extern crate image;

use image::ImageBuffer;
use rayon::prelude::*;
use serde_json::Value;
use std::error::Error;
use std::fs;
use std::time::SystemTime;

mod raymarch;
mod scene;
mod vector;

fn process_file(input_filename: &str) -> Result<Value, Box<dyn Error>> {
    let data = fs::read_to_string(input_filename)?;

    let v = serde_json::from_str(&data)?;

    Ok(v)
}

fn parallel_march(width: u32, height: u32, scene: &str) -> ImageBuffer<image::Rgb<u8>, Vec<u8>> {
    let pixels: Vec<u8> = (0..width * height)
        .into_par_iter()
        .map(|i| {
            let x = i % width;
            let y = i / width;

            let x_pos = ((x as f32) / (width as f32) - 0.5) * 0.8;
            let y_pos = (0.5 - (y as f32) / (height as f32)) * 0.6;

            let dir = vector::Vec3::new(-0.2 - y_pos, x_pos, 1.0).normalize();

            let (r, g, b) = raymarch::march(
                match scene {
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

    assert_eq!(width * height * 3, pixels.len() as u32);

    ImageBuffer::<image::Rgb<u8>, _>::from_vec(width, height, pixels).unwrap()
}

#[allow(dead_code)]
fn march(width: u32, height: u32, scene: &str) -> ImageBuffer<image::Rgb<u8>, Vec<u8>> {
    let mut img = ImageBuffer::new(width, height);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        // generate primary ray

        let x_pos = ((x as f32) / (width as f32) - 0.5) * 0.8;
        let y_pos = (0.5 - (y as f32) / (height as f32)) * 0.6;

        let dir = vector::Vec3::new(-0.2 - y_pos, x_pos, 1.0).normalize();

        let (r, g, b) = raymarch::march(
            match scene {
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

    img
}

fn main() {
    let matches = clap_app!(rustic =>
        (version: "1.0")
        (author: "V Wood <vwood@vwood.org>")
        (about: "Simple Raymarcher")
        (@arg INPUT: +required "Input file to use")
        (@arg OUTPUT: +required "Name of output file")
        (@arg threads: -t [count] "Sets the number of threads to use")
    )
    .get_matches();

    let input_filename = matches.value_of("INPUT").unwrap_or("default.json");
    let output_filename = matches.value_of("OUTPUT").unwrap_or("default.png");

    let thread_count = matches
        .value_of("threads")
        .unwrap_or("")
        .parse::<usize>()
        .unwrap_or(4);

    let result = process_file(input_filename);

    let value = match result {
        Ok(v) => v,
        Err(error) => {
            println!("Error: {}", error);
            return;
        }
    };

    let sdf = value["scene"].as_str().unwrap_or("");

    let width = 800;
    let height = 600;

    let start = SystemTime::now();
    let img;
    if thread_count > 1 {
        // Set number of threads
        rayon::ThreadPoolBuilder::new()
            .num_threads(thread_count)
            .build_global()
            .unwrap();
        println!("Using {} threads", thread_count);

        img = parallel_march(width, height, sdf);
    } else {
        println!("Threads disabled");
        img = march(width, height, sdf);
    }
    let end = SystemTime::now();

    let time_taken = end
        .duration_since(start)
        .expect("Negative time")
        .as_millis();
    println!("Took {}ms", time_taken);

    img.save(output_filename).unwrap();
    println!("Wrote {}", output_filename);
}
