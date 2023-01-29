use std::env;
use std::hash::Hash;

use color_space::{Hsv, ToRgb};
use colored::*;
use rand::prelude::*;
use rand_pcg::Pcg64;
use rand_seeder::{Seeder, SipHasher};

fn main() {
    // implementation of the ls command with rainbow colors

    // get the current directory
    let current_dir = env::current_dir().unwrap();
    // get the content of the current directory
    let contents = current_dir.read_dir().unwrap();

    // get the current terminal color

    for content in contents {
        let content = content.unwrap();
        let path = content.path();
        let file_name = path.file_name().unwrap().to_str().unwrap();

        // get the file type
        let file_type = content.file_type().unwrap();

        // print the file name with the appropriate color

        // the color in  hsb format

        if file_type.is_dir() {
            let mut rng: Pcg64 = Seeder::from(file_name).make_rng();
            let hsb = Hsv::new(rng.gen_range(90.0..=270.0), 1.0, 1.0);
            let rgb = hsb.to_rgb();
            println!("{}", file_name.truecolor(rgb.r as u8, rgb.g as u8, rgb.b as u8));
        } else if file_type.is_file() {
            let mut rng: Pcg64 = Seeder::from(file_name).make_rng();
            let mut h = rng.gen_range(0.0..=180.0);
            if h < 90.0 {
                h += 270.0;
            } else {
                h -= 90.0;
            }
            let hsb = Hsv::new(h, 1.0, 1.0);
            let rgb = hsb.to_rgb();
            println!("{}", file_name.truecolor(rgb.r as u8, rgb.g as u8, rgb.b as u8));
        } else {
            println!("{}", file_name.cyan());
        }

        // print the file type with the appropriate color
    }
}
