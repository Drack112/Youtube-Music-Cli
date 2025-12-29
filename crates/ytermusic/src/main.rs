use std::path::Path;

use crate::consts::INTRODUCTION;

mod consts;

fn main() {
    if let Some(arg) = std::env::args().nth(1) {
        match arg.as_str() {
            "-h" | "--help" => {
                println!("{}", INTRODUCTION);
                return;
            }
            e => {
                println!("Unknown argument `{e}`");
                println!("Here are the available arguments:");
                println!(" - --files: Show the location of the ytermusic files");
                println!(" - --clear-cache: Erase all the files in cache");
                println!(" - --fix-db: Fix the database");
                return;
            }
        }
    } else {
        std::fs::write(Path::new("./log.log"), "Failed").unwrap();
    }
}
