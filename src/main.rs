use std::io::Write;

use termion::{self, raw::IntoRawMode, event::Key, input::TermRead};

use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, Sink};
use rodio::source::Source;

use std::time::SystemTime;


pub mod display;
pub mod files;
pub mod sound;

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
    let mut stdin = termion::async_stdin().keys();
    write!(stdout, "{}{}", termion::cursor::Goto(1,1), termion::cursor::Hide).unwrap();




    let config = display::Config::new(display::Ratio{y: 0.6, x: 1.0}, display::Ratio{y: 0.4, x: 1.0}, display::Point{x:0,y:0}, display::Point{x:0,y:1}); 

    config.frame();


    let mut playlists = files::list_songs("/home/neto/music/");

    config.array(&playlists);

    playlists = config.highlight(0, 0, playlists);


    config.timeline(0.8);

    stdout.flush().unwrap();




    let mut selec_state = Selection::Playlists;



    let mut current_playlist: String = String::new();

    let mut current_song = 0;

    let mut row = 0;
    let mut previous_row = 0;

    let mut skip_once = true;

    let mut is_running = true;


    let mut song_start = SystemTime::now();
    let mut song_length = 0;


    while is_running {

        //let stdin = std::io::stdin();
        let input = stdin.next();

        //for c in stdin.keys() {
        if let Some(Ok(key)) = input {

            //match c.unwrap() {
            match key {
                Key::Char('q') => is_running = false,
                Key::Char(' ') => {
                    files::log("pause");

                    sound::pause(&sink);
                },
                Key::Char('s') => {
                    if skip_once {
                        sink.stop();


                        skip_once = false;
                    }

                },
                Key::Char('p') => {
                    if selec_state == Selection::Songs {
                        files::log("play song");

                        sound::add_song(&sink, String::from("/home/neto/music/") + &current_playlist + "/" + &playlists[row] + ".mp3"); 
                    }


                    if selec_state == Selection::Playlists {

                        selec_state = Selection::Songs;
                        

                        current_playlist = playlists[row].to_string();

                        let path: String = String::from("/home/neto/music/") + &playlists[row];

                        playlists = files::list_songs(&path);

                        config.clear(display::Windows::Files);

                        config.array(&playlists);

                        playlists = config.highlight(0, 0, playlists);

                        row = 0;
                        
                        let test = String::from("/home/neto/music/") + &current_playlist + "/" + &playlists[current_song] + ".mp3";
                        files::log(&test);

                        

                        let song = String::from("/home/neto/music/") + &current_playlist + "/" + &playlists[row] + ".mp3";

                        sound::add_song(&sink, song.clone());
                        
                        song_length = files::duration(&song);

                        song_start = SystemTime::now();
                    } 
                    
                },
                Key::Char('r') => {

                    if selec_state == Selection::Playlists {

                        selec_state = Selection::Songs;
                        
                        current_playlist = playlists[row].to_string();

                        let path: String = String::from("/home/neto/music/") + &playlists[row];

                        playlists = files::list_songs(&path);
                        
                        playlists = files::shuffle_playlist(playlists);

                        
                        config.clear(display::Windows::Files);

                        config.array(&playlists);

                        playlists = config.highlight(0, 0, playlists);

                        row = 0;


                        let song = String::from("/home/neto/music/") + &current_playlist + "/" + &playlists[row] + ".mp3";

                        sound::add_song(&sink, song.clone());
                        
                        song_length = files::duration(&song);

                        song_start = SystemTime::now();

                    } 
                },
                Key::Char('b') => {
                    if selec_state == Selection::Songs {
                        selec_state = Selection::Playlists;
                        

                        playlists = files::list_songs("/home/neto/music/");

                        config.clear(display::Windows::Files);

                        config.array(&playlists);

                        playlists = config.highlight(0, 0, playlists);

                        row = 0;

                    }
                },
                Key::Char('A') => {
                   if row <= 0 {
                        continue;
                    }

                    previous_row = row;
                    row -= 1;
                    playlists = config.highlight(row, previous_row, playlists);
                },
                Key::Char('B') => {
                    if row >= playlists.len() - 1 {
                        continue;
                    }

                    files::log("dowdownn");

                    previous_row = row;
                    row += 1;
                    playlists = config.highlight(row, previous_row, playlists);
                },
                _ => files::log("pressed key"),
                
            }

                            
            


        }
        if selec_state == Selection::Songs { 

            if current_song + 1 < playlists.len() {

                let path = String::from("/home/neto/music/") + &current_playlist + "/" + &playlists[current_song + 1] + ".mp3";

                if sound::update(&sink, path.clone()) == 1 {
                    current_song += 1;

                   
                    song_length = files::duration(&path);

                    song_start = SystemTime::now(); 


                    previous_row = row;
                    row += 1;

                    playlists = config.highlight(current_song, previous_row, playlists);
                }

            }

            if song_start.elapsed().expect("err").as_secs() < song_length {
                
                config.timeline(song_start.elapsed().expect("error").as_secs() as f32 / song_length as f32);
            }

        }

        stdout.flush().unwrap();
    }



    write!(stdout, "{}{}{}", termion::cursor::Show, termion::clear::All, termion::cursor::Goto(1,1)).unwrap();


}


