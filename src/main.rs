use std::usize;

use color_thief;

use colors::commands;
use image::RgbImage;

const HEIGHT: u32 = 30;
const WIDTH: u32 = 300;

fn main() {
    let img = match commands::open_image() {
        Ok(i) => i,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    let max_colors = match commands::get_num_colors() {
        Ok(i) => i,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    let destination_path = match commands::get_desination_path() {
        Ok(p) => p,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    let ct = color_thief::get_palette(
        img.to_rgb8().into_raw().as_slice(),
        color_thief::ColorFormat::Rgb,
        1,
        max_colors,
    );

    match ct {
        Ok(colors) => {
            let mut new_img = RgbImage::new(WIDTH, HEIGHT);

            let num_colors = colors.len();
            let bucket_width: u32 = WIDTH / num_colors as u32;

            for (x, _, pixel) in new_img.enumerate_pixels_mut() {
                let mut idx = (x as f32 / bucket_width as f32).floor() as usize;
                if idx >= num_colors {
                    idx -= 1;
                }

                let color = colors[idx];
                *pixel = image::Rgb([color.r, color.g, color.b]);
            }

            let res = new_img.save(destination_path.clone());
            match res {
                Ok(_img) => {
                    let msg = format!("Image saved in {}.", destination_path);
                    println!("{}", msg);
                }
                Err(e) => {
                    println!("{:?}", e);
                    println!("Couldn't save image.")
                }
            }
        }
        Err(_) => println!("Couldn't fetch colors from the file."),
    }
}
