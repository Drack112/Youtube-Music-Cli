use directories::ProjectDirs;

pub fn get_project_dirs() -> Option<ProjectDirs> {
    ProjectDirs::from("com", "drack", "ytermusic")
}
