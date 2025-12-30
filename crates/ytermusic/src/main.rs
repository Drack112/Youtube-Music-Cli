use std::{
    panic,
    path::{Path, PathBuf},
    str::FromStr,
    sync::RwLock,
};

use log::{error, info};
use once_cell::sync::Lazy;

use crate::{
    consts::{CACHE_DIR, INTRODUCTION},
    database::DATABASE,
    shutdown::shutdown,
    systems::logger::{get_log_file_path, init},
    utils::get_project_dirs,
};

mod consts;
mod database;
mod shutdown;
mod systems;
mod utils;

static COOKIES: Lazy<RwLock<Option<String>>> = Lazy::new(|| RwLock::new(None));

fn cookies(specific_browser: Option<String>) -> Option<String> {
    let loaded = match specific_browser {
        Some(browser) => match browser.as_str() {
            "firefox" => rookie::firefox,
            "chrome" => rookie::chrome,
            "edge" => rookie::edge,
            "opera" => rookie::opera,
            "brave" => rookie::brave,
            "vivaldi" => rookie::vivaldi,
            "chromium" => rookie::chromium,
            #[cfg(target_os = "macos")]
            "safari" => rookie::safari,
            "arc" => rookie::arc,
            "librewolf" => rookie::librewolf,
            "opera-gx" | "opera_gx" => rookie::opera_gx,
            #[cfg(target_os = "windows")]
            "internet_explorer" | "internet-explorer" | "ie" => rookie::internet_explorer,
            #[cfg(target_os = "windows")]
            "octo_browser" | "octo-browser" => rookie::octo_browser,
            _ => {
                println!("Unknown browser `{browser}`");
                error!("Unknown browser `{browser}`");
                return None;
            }
        },
        None => rookie::load,
    }(Some(vec!["youtube.com".to_string()]))
    .unwrap();

    let mut cookies = Vec::new();
    let current_timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    for cookie in loaded {
        if cookie.domain != ".youtube.com" && cookie.domain != "music.youtube.com" {
            continue;
        }
        if cookie
            .expires
            .map(|e| e < current_timestamp)
            .unwrap_or(false)
        {
            continue;
        }
        if cookies.iter().any(|(name, _)| name == &cookie.name) {
            continue;
        }
        cookies.push((cookie.name, cookie.value));
    }

    let cookies = cookies
        .iter()
        .map(|(name, value)| format!("{name}={value}"))
        .collect::<Vec<_>>();
    let cookies = cookies.join("; ");
    Some(cookies)
}

fn get_header_file() -> Result<(String, PathBuf), (std::io::Error, PathBuf)> {
    let fp = PathBuf::from_str("headers.txt").unwrap();
    if let Ok(e) = std::fs::read_to_string(&fp) {
        return Ok((e, fp));
    }
    let fp = get_project_dirs()
        .ok_or_else(|| {
            (
                std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "Can't find project dir. This is a `directories` crate issue",
                ),
                Path::new("./").to_owned(),
            )
        })?
        .config_dir()
        .to_owned();

    if let Err(e) = std::fs::create_dir_all(&fp) {
        println!("Can't create app directory {e} in `{}`", fp.display());
    }
    let fp = fp.join("headers.txt");
    std::fs::read_to_string(&fp).map_or_else(|e| Err((e, fp.clone())), |e| Ok((e, fp.clone())))
}

fn main() {
    if let Some(arg) = std::env::args().nth(1) {
        match arg.as_str() {
            "-h" | "--help" => {
                println!("{}", INTRODUCTION);
                return;
            }
            "--files" => {
                println!("# Location of ytermusic files");
                println!(" - Log: {}", get_log_file_path().display());
                println!(" - Headers: {}", get_header_file().unwrap().1.display());
                return;
            }
            "--fix-db" => {
                DATABASE.fix_db();
                DATABASE.write();
                println!("[INFO] Database fixed");
                return;
            }
            "--clear-cache" => {
                match std::fs::remove_dir_all(&*CACHE_DIR) {
                    Ok(_) => {
                        println!("[INFO] Cache cleared");
                    }
                    Err(e) => {
                        println!("[ERROR] Can't clear cache: {e}");
                    }
                }
                return;
            }
            "--with-auto-cookies" => {
                std::fs::write(get_log_file_path(), "# YTerMusic log file\n\n").unwrap();
                init().expect("Failed to initialize logger");

                let param = std::env::args().nth(2);
                if let Some(cookies) = cookies(param) {
                    let mut cookies_guard = COOKIES.write().unwrap();
                    info!("Cookies: {cookies}");
                    *cookies_guard = Some(cookies);
                    info!("Cookies loaded");
                } else {
                    error!("Can't load cookies");
                    error!("Maybe rookie didn't find any cookies or any browser");
                    error!("Please make sure you have cookies in your browser");
                    return;
                }
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
        std::fs::write(get_log_file_path(), "# YTerMusic log file\n\n").unwrap();
        init().expect("Failed to initialize logger");
    }

    panic::set_hook(Box::new(|e| {
        println!("{e}");
        error!("{e}");
        shutdown();
    }));
}
