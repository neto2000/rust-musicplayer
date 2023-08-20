
use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, Sink};


pub fn add_song(sink: &rodio::Sink, path: String) {
    let file = BufReader::new(File::open(path).unwrap());
    let source = Decoder::new(file).unwrap();

    sink.append(source);

}

pub fn pause(sink: &rodio::Sink) {
    if sink.is_paused() {
        sink.play();
    }
    else {
        sink.pause();
    }
}
