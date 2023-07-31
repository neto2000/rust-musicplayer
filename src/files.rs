use std::fs;

pub fn list_songs(path: &str) -> Vec<String> {

    let dir = fs::read_dir(path).unwrap();

    let mut playlist = Vec::new();

    for song in dir {
        playlist.push(String::from(song.unwrap().path().file_name().unwrap().to_str().unwrap())); 

    }

    return playlist;
}
