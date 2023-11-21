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

    files::title("/home/neto/music/real_songs/A.mp3");

    // clear Screen and initaite input
    println!("{}",termion::clear::All);

    let stdin = std::io::stdin();

    let mut stdout = std::io::stdout().into_raw_mode().unwrap();
    let mut stdin = termion::async_stdin().keys();
    write!(stdout, "{}{}", termion::cursor::Goto(1,1), termion::cursor::Hide).unwrap();




    let mut config = display::Config::new(display::Ratio{y: 0.6, x: 1.0}, display::Ratio{y: 0.4, x: 1.0}, display::Point{x:0,y:0}, display::Point{x:0,y:1}); 

    config.frame();


    let mut playlists = files::list_songs("/home/neto/music/");

    config.array(&playlists, "playlist");

    playlists = config.highlight(0, 0, playlists, "playlist");


    config.timeline(0.8);
    config.play_pause(true);
    config.title("Paradox - Survive said the Prophet");

    stdout.flush().unwrap();




    let mut selec_state = Selection::Playlists;



    let mut current_playlist: String = String::new();

    let mut current_song = 0;

    let mut selected = 0;
    let mut previous_selected = 0;

    let mut skip_once = true;

    let mut is_running = true;

    let mut is_paused = false;


    let mut song_start = SystemTime::now();
    let mut song_length = 0;

    let mut song_secs_played = 0;


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

                    is_paused = sound::pause(&sink);

                    if is_paused {
                        song_secs_played += song_start.elapsed().expect("err").as_secs();

                        config.play_pause(false);
                    }
                    else {

                        song_start = SystemTime::now();

                        config.play_pause(true);
                    }
                },
                Key::Char('s') => {
                    if skip_once {
                        sink.stop();

                        files::log("skip");

                        skip_once = false;
                    }

                },
                Key::Char('p') => {
                    if selec_state == Selection::Songs {
                   

                        let path: String = String::from("/home/neto/music/") + &current_playlist + "/" + &playlists[selected];


                        if skip_once {
                            sink.stop();


                            skip_once = false;
                        }


                        sound::add_song(&sink, String::from("/home/neto/music/") + &current_playlist + "/" + &playlists[selected]); 
                        
                        song_secs_played = 0;
                       
                        song_length = files::duration(&path);

                        song_start = SystemTime::now();

                        current_song = selected;


                        previous_selected = selected;
                        selected = current_song;


                        config.title(&files::title(&path));

                        files::log("play song");


                        skip_once = true;

                    }


                    if selec_state == Selection::Playlists {

                        selec_state = Selection::Songs;
                        

                        current_playlist = playlists[selected].to_string();

                        let path: String = String::from("/home/neto/music/") + &playlists[selected];

                        playlists = files::list_songs(&path);

                        config.clear(display::Windows::Files);

                        config.array(&playlists, &path);

                        playlists = config.highlight(0, 0, playlists, &path);

                        selected = 0;

                        config.set_top_index(0);



                        song_secs_played = 0;
                        
                        let test = String::from("/home/neto/music/") + &current_playlist + "/" + &playlists[current_song] + ".mp3";
                        files::log(&test);

                        

                        let song = String::from("/home/neto/music/") + &current_playlist + "/" + &playlists[selected];

                        sound::add_song(&sink, song.clone());
                        
                        song_length = files::duration(&song);

                        config.title(&files::title(&song));

                        song_start = SystemTime::now();
                    } 
                    
                },
                Key::Char('r') => {

                    if selec_state == Selection::Playlists {

                        selec_state = Selection::Songs;
                        
                        current_playlist = playlists[selected].to_string();

                        let path: String = String::from("/home/neto/music/") + &playlists[selected];

                        playlists = files::list_songs(&path);
                        
                        playlists = files::shuffle_playlist(playlists);

                        
                        config.clear(display::Windows::Files);

                        config.array(&playlists, &path);

                        playlists = config.highlight(0, 0, playlists, &path);

                        selected = 0;

                        config.set_top_index(0);



                        song_secs_played = 0;


                        let song = String::from("/home/neto/music/") + &current_playlist + "/" + &playlists[selected];

                        sound::add_song(&sink, song.clone());
                        
                        song_length = files::duration(&song);

                        config.title(&files::title(&song));

                        song_start = SystemTime::now();

                    } 
                },
                Key::Char('k') => {
                    files::log("up");
                    if selected <= 0 {
                        continue;
                    }

                    let path: String = String::from("/home/neto/music/") + &current_playlist;

                    previous_selected = selected;
                    selected -= 1;
                    

                    if selected < config.top_index {

                        config.set_top_index(config.top_index - 1);

                        config.clear(display::Windows::Files);

                        config.array(&playlists, &path);

                    }


                    if selec_state == Selection::Playlists {

                        playlists = config.highlight(selected, previous_selected, playlists, "playlist");
                    }
                    else {

                        playlists = config.highlight(selected, previous_selected, playlists, &path);
                    }
                },
                Key::Char('j') => {
                    files::log("dowdownn");
                    if selected >= playlists.len() - 1 {
                        continue;
                    }

                    let path: String = String::from("/home/neto/music/") + &current_playlist;

                    previous_selected = selected;
                    selected += 1;

                    if selected - (config.top_index) > config.files_width.y - 3 {

                        config.set_top_index(config.top_index + 1);

                        config.clear(display::Windows::Files);

                        config.array(&playlists, &path);

                    }

                    if selec_state == Selection::Playlists {

                        playlists = config.highlight(selected, previous_selected, playlists, "playlist");
                    }
                    else {


                        playlists = config.highlight(selected, previous_selected, playlists, &path);
                    }                
                },
                Key::Char('b') => {
                    if selec_state == Selection::Songs {
                        selec_state = Selection::Playlists;
                        

                        playlists = files::list_songs("/home/neto/music/");

                        config.clear(display::Windows::Files);

                        config.array(&playlists, "playlist");

                        playlists = config.highlight(0, 0, playlists, "playlist");

                        selected = 0;

                        config.set_top_index(0);

                    }
                },
                
                _ => {files::log("pressed key");},
                
            }

                            
            


        }
        if selec_state == Selection::Songs { 

            if current_song + 1 < playlists.len() {

                let playlist_path =String::from("/home/neto/music/") + &current_playlist; 

                let path = String::from("/home/neto/music/") + &current_playlist + "/" + &playlists[current_song + 1];

                if sound::update(&sink, path.clone()) == 1 {
                    current_song += 1;

                    song_secs_played = 0;
                   
                    song_length = files::duration(&path);

                    song_start = SystemTime::now(); 


                    previous_selected = selected;
                    selected = current_song;

                    playlists = config.highlight(current_song, previous_selected, playlists, &playlist_path);

                    config.title(&files::title(&path));

                    skip_once = true;
                }

            }

            let time = song_start.elapsed().expect("err").as_secs() + song_secs_played;

            if time < song_length && !is_paused{
                
                config.timeline(time as f32 / song_length as f32);
            }

        }

        stdout.flush().unwrap();
    }



    write!(stdout, "{}{}{}", termion::cursor::Show, termion::clear::All, termion::cursor::Goto(1,1)).unwrap();


}


