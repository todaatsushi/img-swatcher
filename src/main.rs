use colors::{call_args, img, pallete};

fn main() {
    let images = match img::open_images() {
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

    pallete::create_palletes(images, max_colors);
}
