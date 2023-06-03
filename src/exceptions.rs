use std::fmt::Display;

pub enum ArgErr {
    InvalidArgsNum(u8),
    BadColorNumber(u8),
    CouldntReadValue(String),
    InvalidMaxColors(u8),
    NotADir,
}

impl Display for ArgErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArgErr::InvalidArgsNum(num) => {
                println!("Got invalid number of arguments ({})", num - 1);
                write!(
                    f,
                    "You need to give 3 arguments - the file path, the max number of colors, and the destination path (incl the filename)."
                )
            }
            ArgErr::BadColorNumber(num) => {
                println!("{}", num);
                write!(f, "Failed.")
            }
            ArgErr::CouldntReadValue(v) => {
                println!("Couldn't parse number from {}", v);
                write!(f, "Max color needs to be a number")
            }
            ArgErr::InvalidMaxColors(num) => {
                println!("Invalid number of max colors: {}", num);
                write!(f, "Must be between 2 and 100.")
            }
            ArgErr::NotADir => {
                println!("Destination directory isn't a valid directory");
                write!(f, "Please pass an existing folder to save into.")
            }
        }
    }
}

pub enum FileErr {
    CouldntRead,
    NotAnImage,
}

impl Display for FileErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileErr::CouldntRead => {
                write!(f, "Couldn't read the file. Check the logs for info.")
            }
            FileErr::NotAnImage => {
                write!(f, "Not an image file.")
            }
        }
    }
}

pub enum GenerationErr {
    CouldntCreatePallete,
}

impl Display for GenerationErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GenerationErr::CouldntCreatePallete => {
                write!(f, "Couldn't generate palette from image.")
            }
        }
    }
}
