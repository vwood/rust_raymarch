#[macro_use]
extern crate clap;
extern crate image;
extern crate serde;

use image::ImageBuffer;
use rayon::prelude::*;

use std::error::Error;
use std::fs;
use std::time::SystemTime;

mod lighting;
mod raymarch;
mod scene;
mod sdf;
mod vector;

fn process_file(input_filename: &str) -> Result<scene::SceneDescription, Box<dyn Error>> {
    let data = fs::read_to_string(input_filename)?;

    let v = serde_json::from_str(&data)?;

    Ok(v)
}

#[allow(clippy::many_single_char_names)]
fn march_pixel(x: u32, y: u32, scene: &scene::Scene) -> (u8, u8, u8) {
    let width = scene.width;
    let height = scene.height;

    let x_pos = ((x as f32) / (width as f32) - 0.5) * 0.8;
    let y_pos = (0.5 - (y as f32) / (height as f32)) * 0.6;

    let dir = (scene.direction + scene.screen_y * y_pos + scene.screen_x * x_pos).normalize();

    let (r, g, b) = raymarch::march(&scene, &dir);

    let r = (255.0 * r) as u8;
    let g = (255.0 * g) as u8;
    let b = (255.0 * b) as u8;

    (r, g, b)
}

#[allow(clippy::many_single_char_names)]
fn parallel_march(scene: &scene::Scene) -> ImageBuffer<image::Rgb<u8>, Vec<u8>> {
    let width = scene.width;
    let height = scene.height;

    let pixels: Vec<u8> = (0..width * height)
        .into_par_iter()
        .map(|i| {
            let x = i % scene.width;
            let y = i / scene.width;

            let (r, g, b) = march_pixel(x, y, scene);
            vec![r, g, b]
        })
        .flatten()
        .collect();

    assert_eq!(width * height * 3, pixels.len() as u32);

    ImageBuffer::<image::Rgb<u8>, _>::from_vec(width, height, pixels).unwrap()
}

#[allow(clippy::many_single_char_names)]
fn march(scene: &scene::Scene) -> ImageBuffer<image::Rgb<u8>, Vec<u8>> {
    let mut img = ImageBuffer::new(scene.width, scene.height);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let (r, g, b) = march_pixel(x, y, scene);
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

    let description = match result {
        Ok(v) => v,
        Err(error) => {
            eprintln!("Error: {}", error);
            return;
        }
    };

    let scene = scene::Scene::from(description);

    let start = SystemTime::now();
    let img;
    if thread_count > 1 {
        // Set number of threads
        rayon::ThreadPoolBuilder::new()
            .num_threads(thread_count)
            .build_global()
            .unwrap();
        println!("Using {} threads", thread_count);

        img = parallel_march(&scene);
    } else {
        println!("Threads disabled");
        img = march(&scene);
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
