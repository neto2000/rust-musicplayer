
use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, Sink};

let VOLUME_FACTOR: f32 = 0.1; 

pub fn add_song(sink: &rodio::Sink, path: String) {
    let file = BufReader::new(File::open(path).unwrap());
    let source = Decoder::new(file).unwrap();

    sink.append(source);

}

pub fn pause(sink: &rodio::Sink) -> bool {
    if sink.is_paused() {
        sink.play();

        return false;
    }
    else {
        sink.pause();

        return true;
    }
}

pub fn increase_volume(sink: &rodio::Sink) {

    sink.set_volume(sink.volume() + VOLUME_FACTOR);

}

pub fn decrease_volume(sink: &rodio::Sink) {

    sink.set_volume(sink.volume() - VOLUME_FACTOR);

}
pub fn update(sink: &rodio::Sink, next_song: String) -> usize {
    if sink.empty() {
        let file = BufReader::new(File::open(next_song).unwrap());
        let source = Decoder::new(file).unwrap();

        sink.append(source);

        return 1;

    } 

    return 0;
}
