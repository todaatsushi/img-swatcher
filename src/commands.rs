use std::path::PathBuf;

use image::{Rgb, RgbImage};
use rgb::RGB;

const HEIGHT: u32 = 50;
const WIDTH: u32 = 500;

pub fn create_swatch(colors: Vec<RGB<u8>>, destination_path: PathBuf) {
    let mut new_img = RgbImage::new(WIDTH, HEIGHT);

    let num_colors = colors.len();
    let bucket_width: u32 = WIDTH / num_colors as u32;

    for (x, _, pixel) in new_img.enumerate_pixels_mut() {
        let mut idx = (x as f32 / bucket_width as f32).floor() as usize;
        if idx >= num_colors {
            idx -= 1;
        }

        let color = colors[idx];
        *pixel = Rgb([color.r, color.g, color.b]);
    }

    let res = new_img.save(destination_path.clone().to_owned());
    match res {
        Ok(_img) => {
            let msg = format!(
                "Image saved in {}.",
                destination_path.to_string_lossy().to_owned()
            );
            println!("{}", msg);
        }
        Err(e) => {
            println!("{:?}", e);
            println!("Couldn't save image.")
        }
    }
}
