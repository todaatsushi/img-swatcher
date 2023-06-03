use crate::call_args::get_args;
use crate::exceptions::FileErr;
use image::DynamicImage;
use std::{
    collections::HashMap,
    ffi::OsStr,
    fmt::Display,
    fs::{self, DirEntry, ReadDir},
    path::{Path, PathBuf},
    str::FromStr,
};

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

fn is_image(file: &DirEntry) -> bool {
    let path = file.path();
    let p = Path::new(&path);
    let ext = p.extension().and_then(OsStr::to_str);
    match ext {
        Some(e) => {
            let image_ext = ImageFile::from_str(e);
            match image_ext {
                Ok(_) => true,
                Err(_) => false,
            }
        }
        None => false,
    }
}

fn build_map_from_dir(paths: ReadDir) -> HashMap<String, Option<DynamicImage>> {
    let mut store: HashMap<String, Option<DynamicImage>> = HashMap::new();
    paths.into_iter().for_each(|p| {
        let path = p.unwrap();
        match is_image(&path) {
            true => match get_img(&path.path()) {
                Some(img) => {
                    let path_str = path.path().to_string_lossy().into_owned();
                    store.insert(path_str, Some(img));
                }
                None => (),
            },
            false => (),
        }
    });
    store
}

fn build_map_from_file(
    file_path: PathBuf,
) -> Result<HashMap<String, Option<DynamicImage>>, FileErr> {
    let mut store: HashMap<String, Option<DynamicImage>> = HashMap::new();
    match get_img(&file_path) {
        Some(img) => {
            let path = file_path.to_str().unwrap().to_string();
            store.insert(path, Some(img));
        }
        None => return Err(FileErr::CouldntRead),
    };
    Ok(store)
}

pub fn open_images() -> Result<HashMap<String, Option<DynamicImage>>, FileErr> {
    match get_args() {
        Ok(args) => {
            let arg = &args[1];
            let target = PathBuf::from(arg);
            let store;

            if target.is_dir() {
                let files = fs::read_dir(target.to_str().unwrap());
                store = match files {
                    Ok(paths) => build_map_from_dir(paths),
                    Err(_e) => return Err(FileErr::CouldntRead),
                };
            } else if target.is_file() {
                store = match build_map_from_file(target) {
                    Ok(s) => s,
                    Err(e) => return Err(e),
                };
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
