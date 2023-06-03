use std::{env, path::PathBuf};

use crate::exceptions::ArgErr;

pub fn get_args() -> Result<Vec<String>, ArgErr> {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        4 => Ok(args),
        _ => Err(ArgErr::InvalidArgsNum(args.len().try_into().unwrap())),
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

pub fn get_desination_path() -> Result<PathBuf, ArgErr> {
    let dest = match get_args() {
        Ok(args) => {
            let dest = args[3].clone();
            let target = PathBuf::from(&dest);
            if target.is_file() || !target.is_dir() {
                return Err(ArgErr::NotADir);
            }
            target
        }
        Err(e) => return Err(e),
    };
    Ok(dest)
}
