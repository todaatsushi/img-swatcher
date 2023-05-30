use color_thief;

use colors::{call_args, commands};

fn main() {
    let img = match call_args::open_image() {
        Ok(i) => i,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    let max_colors = match call_args::get_num_colors() {
        Ok(i) => i,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    let destination_path = match call_args::get_desination_path() {
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
        Ok(colors) => commands::create_swatch(colors, destination_path),
        Err(_) => println!("Couldn't fetch colors from the file."),
    }
}
