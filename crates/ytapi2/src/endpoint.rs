#[derive(Debug, Clone, PartialOrd, Eq, Ord, PartialEq, Hash)]
pub enum Endpoint {
    MusicLikedPlaylists,
    MusicHome,
    MusicLibraryLanding,
    Playlist(String),
    Search(String),
}

impl Endpoint {
    pub fn get_key(&self) -> String {
        match self {
            Endpoint::MusicLikedPlaylists => "browseId".to_owned(),
            Endpoint::MusicLibraryLanding => "browseId".to_owned(),
            Endpoint::Playlist(_) => "browseId".to_owned(),
            Endpoint::MusicHome => "browseId".to_owned(),
            Endpoint::Search(_) => "query".to_owned(),
        }
    }
    pub fn get_param(&self) -> String {
        match self {
            Endpoint::MusicLikedPlaylists => "FEmusic_liked_playlists".to_owned(),
            Endpoint::MusicLibraryLanding => "FEmusic_library_landing".to_owned(),
            Endpoint::Playlist(id) => id.to_owned(),
            Endpoint::Search(query) => query.to_owned(),
            Endpoint::MusicHome => "FEmusic_home".to_owned(),
        }
    }
    pub fn get_route(&self) -> String {
        match self {
            Endpoint::MusicLikedPlaylists => "browse".to_owned(),
            Endpoint::MusicLibraryLanding => "browse".to_owned(),
            Endpoint::Playlist(_) => "browse".to_owned(),
            Endpoint::Search(_) => "search".to_owned(),
            Endpoint::MusicHome => "browse".to_owned(),
        }
    }
}
