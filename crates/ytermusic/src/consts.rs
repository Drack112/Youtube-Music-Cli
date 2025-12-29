pub const INTRODUCTION: &str = r#"Usage: ytermusic [options]

YTerMusic is a TUI based Youtube Music Player that aims to be as fast and simple as possible.
In order to get your music, create a file "headers.txt" in the config folder, and copy the Cookie and User-Agent from request header of the music.youtube.com html document "/" page.
More info at: https://github.com/ccgauche/ytermusic

Options:
        -h or --help        Show this menu
        --files             Show the location of the ytermusic files
        --fix-db            Fix the database in cache
        --clear-cache       Erase all the files in cache

Shortcuts:
        Use your mouse to click in lists if your terminal has mouse support
        Space                     play/pause
        Enter                     select a playlist or a music
        f                         search
        s                         shuffle
        r                         remove a music from the main playlist
        Arrow Right or >          skip 5 seconds
        Arrow Left or <           go back 5 seconds
        CTRL + Arrow Right (>)    go to the next song
        CTRL + Arrow Left  (<)    go to the previous song
        +                         volume up
        -                         volume down
        Arrow down                scroll down
        Arrow up                  scroll up
        ESC                       exit the current menu
        CTRL + C or CTRL + D      quit
"#;
