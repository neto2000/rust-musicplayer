use std::io::Write;

use termion::{self, raw::IntoRawMode, event::Key, input::TermRead};



pub mod display;
pub mod files;

#[derive(PartialEq)]
enum Selection {
    Playlists,
    Songs,
}


fn main() {
   
    files::clear_log();

    // clear Screen and initaite input
    println!("{}",termion::clear::All);

    let stdin = std::io::stdin();

    let mut stdout = std::io::stdout().into_raw_mode().unwrap();

    write!(stdout, "{}{}", termion::cursor::Goto(1,1), termion::cursor::Hide).unwrap();





    display::frame();


    let mut playlists = files::list_songs("/home/arne-pi//Music/test");

    display::array(&playlists);

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
                }


                if selec_state == Selection::Playlists {

                    selec_state = Selection::Songs;
                    

                    let path: String = String::from("/home/arne-pi/Music/test/") + &playlists[row];

                    playlists = files::list_songs(&path);

                    display::clear();

                    display::array(&playlists);

                    playlists = display::highlight(0, 0, playlists);

                    row = 0;
                } 
                
            },
            Key::Char('r') => {

                if selec_state == Selection::Playlists {

                    selec_state = Selection::Songs;
                    

                    let path: String = String::from("/home/arne-pi/Music/test/") + &playlists[row];

                    playlists = files::list_songs(&path);
                    
                    playlists = files::shuffle_playlist(playlists);

                    display::clear();

                    display::array(&playlists);

                    playlists = display::highlight(0, 0, playlists);

                    row = 0;
                } 
            },
            Key::Char('b') => {
                if selec_state == Selection::Songs {
                    selec_state = Selection::Playlists;
                    

                    playlists = files::list_songs("/home/arne-pi/Music/test/");

                    display::clear();

                    display::array(&playlists);

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


