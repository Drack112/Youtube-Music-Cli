mod endpoint;
mod instance;
mod json_extractor;
mod types;
mod utils;

use std::path::Path;

use tokio::runtime::Runtime;

use crate::{
    endpoint::Endpoint, instance::YoutubeMusicInstance, types::YoutubeMusicError,
    utils::StringUtils,
};

fn main() {
    Runtime::new().unwrap().block_on(async {
        println!(
            "{:?}",
            YoutubeMusicError::Other("Error in cookie".to_string())
        );

        println!(
            "{:?}",
            "Error in cookie".to_string().between("Error", "cookie")
        );

        println!("{:?}", Endpoint::MusicHome.get_param());

        println!(
            "{:?}",
            YoutubeMusicInstance::from_header_file(Path::new("./headers.txt")).await
        );
    })
}
