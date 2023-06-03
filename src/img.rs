use crate::call_args::get_args;
use crate::exceptions::FileErr;
use image::DynamicImage;
use std::{collections::HashMap, fmt::Display, path::PathBuf, str::FromStr};

enum ImageFile {
    PNG,
    JPEG,
}

impl FromStr for ImageFile {
    type Err = FileErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "png" => Ok(ImageFile::PNG),
            "jpeg" => Ok(ImageFile::JPEG),
            _ => Err(FileErr::NotAnImage),
        }
    }
}

impl Display for ImageFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImageFile::JPEG => write!(f, "JPEG"),
            ImageFile::PNG => write!(f, "PNG"),
        }
    }
}

fn get_img(img_path: &PathBuf) -> Option<DynamicImage> {
    match image::open(img_path.as_path()) {
        Ok(img) => Some(img),
        Err(_e) => {
            println!("Image error. File might not exist so check that the file exists at that location. Otherwise, try another file.");
            None
        }
    }
}

pub fn open_images() -> Result<HashMap<String, Option<DynamicImage>>, FileErr> {
    match get_args() {
        Ok(args) => {
            let arg = &args[1];
            let mut store: HashMap<String, Option<DynamicImage>> = HashMap::new();
            let target = PathBuf::from(arg);

            if target.is_dir() {
                ()
            } else if target.is_file() {
                match get_img(&target) {
                    Some(img) => {
                        let path = target.to_str().unwrap().to_string();
                        store.insert(path, Some(img));
                    }
                    None => return Err(FileErr::CouldntRead),
                }
            } else {
                println!("Image error. File might not exist so check that the file exists at that location. Otherwise, try another file.");
                return Err(FileErr::CouldntRead);
            }
            Ok(store)
        }
        Err(e) => {
            println!("{}", e);
            Err(FileErr::CouldntRead)
        }
    }
}
