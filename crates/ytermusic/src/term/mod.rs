use ytapi2::types::YoutubeMusicVideoRef;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Screens {
    MusicPlayer = 0x0,
    Playlist = 0x1,
    Search = 0x2,
    DeviceLost = 0x3,
    PlaylistViewer = 0x4,
}

#[derive(Debug, Clone)]
pub enum ManagerMessage {
    Error(String, Box<Option<ManagerMessage>>),
    PassTo(Screens, Box<ManagerMessage>),
    Inspect(String, Screens, Vec<YoutubeMusicVideoRef>),
    ChangeState(Screens),
    SearchFrom(Screens),
    PlayerFrom(Screens),
    PlaylistFrom(Screens),
    RestartPlayer,
    Quit,
    AddElementToChooser((String, Vec<YoutubeMusicVideoRef>)),
}
