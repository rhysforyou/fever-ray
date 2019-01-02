use clap::{App, Arg};
use fever_ray::*;
use serde_yaml;
use std::fs::File;

// const MAX_RAY_DEPTH: usize = 3;

fn main() {
    let app = App::new("fever-ray")
        .version("0.1")
        .author("Rhys Powell <rhys@rpowell.me>")
        .about("A raytracer written in Rust")
        .arg(
            Arg::with_name("scene")
                .help("Scene description input")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("image")
                .help("Sets the output image file")
                .required(true)
                .index(2),
        )
        .arg(
            Arg::with_name("width")
                .long("width")
                .short("w")
                .help("The width of the image")
                .default_value("640")
                .required(false),
        )
        .arg(
            Arg::with_name("height")
                .long("height")
                .short("h")
                .help("The height of the image")
                .default_value("480")
                .required(false),
        )
        .arg(
            Arg::with_name("fov")
                .long("fov")
                .short("f")
                .help("The field of view of the image")
                .default_value("90.0")
                .required(false),
        )
        .arg(
            Arg::with_name("shadow_bias")
                .long("shadow_bias")
                .short("s")
                .default_value("0.0001")
                .required(false),
        );
    let matches = app.get_matches();

    let scene_path = matches.value_of("scene").unwrap();
    let scene_file = File::open(scene_path).expect("File not found");

    let image_path = matches.value_of("image").unwrap();
    let width = matches
        .value_of("width")
        .unwrap_or("640")
        .parse::<u32>()
        .unwrap();
    let height = matches
        .value_of("height")
        .unwrap_or("480")
        .parse::<u32>()
        .unwrap();
    let fov = matches
        .value_of("fov")
        .unwrap_or("90.0")
        .parse::<f64>()
        .unwrap();
    let shadow_bias = matches
        .value_of("shadow_bias")
        .unwrap_or("0.0001")
        .parse::<f64>()
        .unwrap();

    let scene: Scene = serde_yaml::from_reader(scene_file).unwrap();

    let config = Config {
        width: width,
        height: height,
        fov,
        shadow_bias,
        scene,
    };

    let image = fever_ray::render(&config);

    image.save(image_path).unwrap();
}
