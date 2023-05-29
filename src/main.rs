use std::{env, fmt::Display, path};

use image::DynamicImage;
use {color_thief, image};

enum ArgErr {
    NotEnoughArgs,
}

impl Display for ArgErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArgErr::NotEnoughArgs => {
                write!(f, "Not enough arguments provided to the call. You need to give 2 arguments - the file path and the max number of colors.")
            }
        }
    }
}

enum FileErr {
    CouldntRead,
}

fn get_args() -> Result<Vec<String>, ArgErr> {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => Ok(args),
        _ => Err(ArgErr::NotEnoughArgs),
    }
}

fn open_image() -> Result<DynamicImage, FileErr> {
    match get_args() {
        Ok(args) => {
            let arg = &args[1];
            let path = path::Path::new(arg);
            match image::open(path) {
                Ok(img) => Ok(img),
                Err(_e) => {
                    println!("Image error. File might not exist so check the path. Otherwise, try another file.");
                    Err(FileErr::CouldntRead)
                }
            }
        }
        Err(e) => {
            println!("{}", e);
            Err(FileErr::CouldntRead)
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = path::Path::new(&args[1]);

    let img = image::open(file_path).expect("Couldn't open image.");
    let ct = color_thief::get_palette(
        img.to_rgb8().into_raw().as_slice(),
        color_thief::ColorFormat::Rgb,
        1,
        10,
    );

    match ct {
        Ok(c) => {
            for (i, color) in c.iter().enumerate() {
                let hex = format!("#{:x?}{:x?}{:x?}", color.r, color.g, color.b);
                println!("{}. {}", i, hex);
            }
        }
        Err(_) => println!("Couldn't fetch colors from the file."),
    }
}
