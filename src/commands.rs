use image::DynamicImage;
use std::{env, path};

use crate::exceptions::{ArgErr, FileErr};

fn get_args() -> Result<Vec<String>, ArgErr> {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        3 => Ok(args),
        _ => Err(ArgErr::InvalidArgsNum(args.len().try_into().unwrap())),
    }
}

pub fn open_image() -> Result<DynamicImage, FileErr> {
    match get_args() {
        Ok(args) => {
            let arg = &args[1];
            let path = path::Path::new(arg);
            match image::open(path) {
                Ok(img) => Ok(img),
                Err(_e) => {
                    println!("Image error. File might not exist so check that the file exists at that location. Otherwise, try another file.");
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

pub fn get_num_colors() -> Result<u8, ArgErr> {
    let num_colors = match get_args() {
        Ok(args) => args[2].clone(),
        Err(e) => return Err(e),
    };
    match num_colors.parse::<u8>() {
        Ok(n) => {
            if 2 <= n && n <= 100 {
                return Ok(n as u8);
            } else {
                return Err(ArgErr::InvalidMaxColors(n));
            }
        }
        Err(_e) => Err(ArgErr::CouldntReadValue(num_colors)),
    }
}
