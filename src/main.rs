use clap::{App, Arg};
use fever_ray::*;

// const MAX_RAY_DEPTH: usize = 3;

fn main() {
    let app = App::new("fever-ray")
        .version("0.1")
        .author("Rhys Powell <rhys@rpowell.me>")
        .about("A raytracer written in Rust")
        .arg(
            Arg::with_name("image")
                .help("Sets the output image file")
                .required(true)
                .index(1),
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
        );
    let matches = app.get_matches();

    let image_path = matches.value_of("image").unwrap();
    let image_width = matches
        .value_of("width")
        .unwrap_or("640")
        .parse::<u32>()
        .unwrap();
    let image_height = matches
        .value_of("height")
        .unwrap_or("480")
        .parse::<u32>()
        .unwrap();
    let image_fov = matches
        .value_of("fov")
        .unwrap_or("90.0")
        .parse::<f64>()
        .unwrap();

    let scene = Scene {
        width: image_width,
        height: image_height,
        fov: image_fov,
        objects: vec![Sphere {
            center: Point3 {
                x: 0.0,
                y: 0.0,
                z: -5.0,
            },
            radius: 1.0,
            material: Material {
                color: Color {
                    red: 100,
                    green: 255,
                    blue: 100,
                },
            },
        }],
    };

    let image = fever_ray::render(&scene);

    image.save(image_path).unwrap();
}
