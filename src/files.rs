use std::fs;
use std::io::Write;
use rand::Rng;

use lofty::{Accessor, AudioFile, Probe, TaggedFileExt};
use std::path::Path;


pub struct SongInfo {
    pub title: String,
    pub artist: String,
    pub year: String,
    pub genre: String,
}

pub fn list_songs(path: &str) -> Vec<String> {

    let dir = fs::read_dir(path).unwrap();

    let mut playlist = Vec::new();

    for song in dir {
        let path = song.unwrap().path();
        let str_song: &str = path.file_name().unwrap().to_str().unwrap();

        let song_vec: Vec<&str> = str_song.split(".").collect();

        if song_vec.len() == 1 {
            playlist.push(String::from(song_vec[0])); 
            continue;
        }

        if song_vec[1] != "mp3" {
            continue;
        }


        playlist.push(String::from(song_vec[0]) + "." + song_vec[1]); 

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
        .open("/home/neto/documents/scripts/rust/rust-musicplayer/log.txt")
        .unwrap();

    
    writeln!(log_file, "{}", msg);

}

pub fn clear_log() {
    let mut log_file = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("/home/neto/documents/scripts/rust/rust-musicplayer/log.txt")
        .unwrap();

    writeln!(log_file, "-- The Log File --");
}



pub fn duration(path: &str) -> u64 {

    let tag = Probe::open(Path::new(path)).expect("error").read().expect("error");

    return tag.properties().duration().as_secs();
    
}

pub fn title(path: &str) -> String {
    
    let tag = Probe::open(Path::new(path)).expect("error").read().expect("error");

    let prim_tag = match tag.primary_tag() {
        Some(p_tag) => p_tag,

        None => return path.to_owned(),
    };

    let title: String = match prim_tag.title() {
        Some(t) => t.to_string(),
        None => path.to_owned(),
    };


    let artist: String = match prim_tag.artist() {
        Some(t) => t.to_string(),
        None => String::new(),
    };

    return String::new() + &title + " - " + &artist;
}

pub fn song_info(path: &str) -> SongInfo {
    let tag = Probe::open(Path::new(path)).expect("error").read().expect("error");

    let prim_tag = match tag.primary_tag() {
        Some(p_tag) => p_tag,

        None => return SongInfo { title: path.to_owned(), artist: "".to_owned(), year: "".to_owned(), genre: "".to_owned() },
    };

    let title: String = match prim_tag.title() {
        Some(t) => t.to_string(),
        None => path.to_owned(),
    };

    //let title: String = prim_tag.title().expect("title err").to_string();
    

   // let artist: String = prim_tag.artist().expect("artist err").to_string();
    let artist: String = match prim_tag.artist() {
        Some(t) => t.to_string(),
        None => String::new(),
    };

    //let year: u32 = prim_tag.year().expect("year err");
    let year: String = match prim_tag.year() {
        Some(t) => t. to_string(),
        None => String::new(),
    };

    //let genre: String = prim_tag.genre().expect("genre err").to_string();
    let genre: String = match prim_tag.genre() {
        Some(t) => t.to_string(),
        None => String::new(),
    };


    return SongInfo { title: title, artist: artist, year: year, genre: genre }
}


