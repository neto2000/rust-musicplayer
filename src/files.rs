use std::fs;
use std::io::Write;
use rand::Rng;

pub fn list_songs(path: &str) -> Vec<String> {

    let dir = fs::read_dir(path).unwrap();

    let mut playlist = Vec::new();

    for song in dir {
        playlist.push(String::from(song.unwrap().path().file_name().unwrap().to_str().unwrap())); 

    }

    return playlist;
}

pub fn shuffle_playlist(mut playlist: Vec<String>) -> Vec<String> {
    
    let mut shuffled: Vec<String> = Vec::new();


    while playlist.len() > 0 {
        let index: usize = rand::thread_rng().gen_range(0..playlist.len());

        shuffled.push(String::from(&playlist[index]));

        playlist.remove(index);
    }   

    return shuffled;
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
