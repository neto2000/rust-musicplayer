use std::io::Write;

use termion::{self, raw::IntoRawMode, event::Key, input::TermRead};

use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, Sink};
use rodio::source::Source;


pub mod display;
pub mod files;

#[derive(PartialEq)]
enum Selection {
    Playlists,
    Songs,
}




fn main() {
   


    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    



    files::clear_log();

    // clear Screen and initaite input
    println!("{}",termion::clear::All);

    let stdin = std::io::stdin();

    let mut stdout = std::io::stdout().into_raw_mode().unwrap();

    write!(stdout, "{}{}", termion::cursor::Goto(1,1), termion::cursor::Hide).unwrap();




    let config = display::Config::new(display::Ratio{y: 0.6, x: 1.0}, display::Ratio{y: 0.4, x: 1.0}, display::Point{x:0,y:0}, display::Point{x:0,y:1}); 

    display::frame(&config);


    let mut playlists = files::list_songs("/home/neto/music/");

    display::array(&config, display::Point{x:0,y:0}, &playlists);

    playlists = display::highlight(0, 0, playlists);


    stdout.flush().unwrap();




    let mut selec_state = Selection::Playlists;


    let mut row = 0;
    let mut previous_row = 0;

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('q') => break,
            Key::Char('p') => {
                if selec_state == Selection::Songs {
                    files::log("play song");

                    let file = BufReader::new(File::open("/home/neto/music/test_playlist/real_song.mp3").unwrap());
                    let source = Decoder::new(file).unwrap();

                    sink.append(source);

                }


                if selec_state == Selection::Playlists {

                    selec_state = Selection::Songs;
                    

                    let path: String = String::from("/home/neto/music/") + &playlists[row];

                    playlists = files::list_songs(&path);

                    display::clear(&config, display::Point{x:0,y:0});

                    display::array(&config, display::Point{x:0,y:0}, &playlists);

                    playlists = display::highlight(0, 0, playlists);

                    row = 0;
                } 
                
            },
            Key::Char('r') => {

                if selec_state == Selection::Playlists {

                    selec_state = Selection::Songs;
                    

                    let path: String = String::from("/home/neto/music/") + &playlists[row];

                    playlists = files::list_songs(&path);
                    
                    playlists = files::shuffle_playlist(playlists);

                    display::clear(&config, display::Point{x:0,y:0});

                    display::array(&config, display::Point{x:0,y:0}, &playlists);

                    playlists = display::highlight(0, 0, playlists);

                    row = 0;
                } 
            },
            Key::Char('b') => {
                if selec_state == Selection::Songs {
                    selec_state = Selection::Playlists;
                    

                    playlists = files::list_songs("/home/neto/music/");

                    display::clear(&config, display::Point{x:0,y:0});

                    display::array(&config, display::Point{x:0,y:0}, &playlists);

                    playlists = display::highlight(0, 0, playlists);

                    row = 0;

                }
            },
            Key::Up => {
               if row <= 0 {
                    continue;
                }

                previous_row = row;
                row -= 1;
                playlists = display::highlight(row, previous_row, playlists);
            },
            Key::Down => {
                if row >= playlists.len() - 1 {
                    continue;
                }

                previous_row = row;
                row += 1;
                playlists = display::highlight(row, previous_row, playlists);
            },
            _ => files::log("pressed key"),
            
        }

        stdout.flush().unwrap();
    }


    write!(stdout, "{}{}{}", termion::cursor::Show, termion::clear::All, termion::cursor::Goto(1,1)).unwrap();


}


