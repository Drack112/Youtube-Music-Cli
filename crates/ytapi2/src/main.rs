use std::path::Path;

use log::error;
use tokio::runtime::Runtime;

use crate::{endpoint::Endpoint, instance::YoutubeMusicInstance};

mod endpoint;
mod instance;
mod json_extractor;
mod types;
mod utils;

fn main() {
    Runtime::new().unwrap().block_on(async {
        match YoutubeMusicInstance::from_header_file(Path::new("headers.txt")).await {
            Ok(instance) => {
                let library = instance
                    .get_library(&Endpoint::MusicLibraryLanding, 0)
                    .await;
                println!("{library:?}")
            }
            Err(e) => {
                error!("{e:?}")
            }
        }
    });
}
