use std::fs;
use std::io::Write;

pub fn list_songs(path: &str) -> Vec<String> {

    let dir = fs::read_dir(path).unwrap();

    let mut playlist = Vec::new();

    for song in dir {
        playlist.push(String::from(song.unwrap().path().file_name().unwrap().to_str().unwrap())); 

    }

    return playlist;
}

pub fn log(msg: &str) {
    let mut log_file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open("/home/arne-pi/Dokumente/Scripts/Rust/rust-musicplayer/log.txt")
        .unwrap();

    
    writeln!(log_file, "{}", msg);

}

pub fn clear_log() {
    let mut log_file = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("/home/arne-pi/Dokumente/Scripts/Rust/rust-musicplayer/log.txt")
        .unwrap();

    writeln!(log_file, "-- The Log File --");
}
