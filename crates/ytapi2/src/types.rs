use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{fmt::Display, string::FromUtf8Error};

pub type Result<T> = std::result::Result<T, YoutubeMusicError>;

#[derive(Debug)]
pub enum YoutubeMusicError {
    RequestError(reqwest::Error),
    Other(String),
    NoCookieAttribute,
    NoSapsidInCookie,
    InvalidCookie(FromUtf8Error),
    NeedToLogin,
    CantFindInnerTubeApiKey(String),
    CantFindInnerTubeClientVersion(String),
    CantFindVisitorData(String),
    SerdeJson(serde_json::Error),
    IoError(std::io::Error),
    YoutubeMusicError(Value),
    InvalidHeaders,
}

#[derive(Debug, Clone, PartialOrd, Eq, Ord, PartialEq, Hash, Serialize, Deserialize)]
pub struct YoutubeMusicVideoRef {
    pub title: String,
    pub author: String,
    pub album: String,
    pub video_id: String,
    pub duration: String,
}

impl Display for YoutubeMusicVideoRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} | {}", self.author, self.title)
    }
}

#[derive(Debug, Clone, PartialOrd, Eq, Ord, PartialEq, Hash, Serialize, Deserialize)]
pub struct YoutubeMusicPlaylistRef {
    pub name: String,
    pub subtitle: String,
    pub browse_id: String,
}

#[derive(Debug, Clone, PartialOrd, Eq, Ord, PartialEq, Hash)]
pub struct SearchResults {
    pub videos: Vec<YoutubeMusicVideoRef>,
    pub playlists: Vec<YoutubeMusicPlaylistRef>,
}

#[derive(Debug, Clone, PartialOrd, Eq, Ord, PartialEq, Hash, Serialize, Deserialize)]
pub struct Continuation {
    pub(crate) continuation: String,
    pub(crate) click_tracking_params: String,
}
