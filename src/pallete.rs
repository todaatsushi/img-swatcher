use std::{collections::HashMap, path::PathBuf};

use color_thief;
use image::DynamicImage;
use rgb::RGB;

use crate::{call_args, commands, exceptions::GenerationErr};

fn get_pallete(img: DynamicImage, max_colors: u8) -> Result<Vec<RGB<u8>>, GenerationErr> {
    match color_thief::get_palette(
        img.to_rgb8().into_raw().as_slice(),
        color_thief::ColorFormat::Rgb,
        1,
        max_colors,
    ) {
        Ok(res) => Ok(res),
        Err(e) => {
            println!("Error creating pallete. {:?}", e);
            Err(GenerationErr::CouldntCreatePallete)
        }
    }
}

pub fn create_palletes(images: HashMap<PathBuf, Option<DynamicImage>>, max_colors: u8) {
    let destination_dir = call_args::get_desination_path();
    match destination_dir {
        Ok(dir) => {
            for (path, img) in images {
                match img {
                    Some(i) => {
                        let img_filename = path
                            .as_path()
                            .file_name()
                            .unwrap()
                            .to_string_lossy()
                            .to_owned()
                            .to_string();
                        let filename = format!("res-{}", img_filename);
                        let destination = dir.as_path().join(filename);
                        match get_pallete(i, max_colors) {
                            Ok(palette) => commands::create_swatch(palette, destination),
                            Err(_) => (),
                        }
                    }
                    None => continue,
                }
            }
        }
        Err(e) => println!("{}", e),
    }
}
