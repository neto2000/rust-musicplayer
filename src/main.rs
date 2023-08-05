use std::io::Write;

use termion::{self, raw::IntoRawMode, event::Key, input::TermRead};

use crate::display::array;


pub mod display;
pub mod files;


fn main() {
   
    files::clear_log();

    // clear Screen and initaite input
    println!("{}",termion::clear::All);

    let stdin = std::io::stdin();

    let mut stdout = std::io::stdout().into_raw_mode().unwrap();

    write!(stdout, "{}{}", termion::cursor::Goto(1,1), termion::cursor::Hide).unwrap();



    display::frame();

    let test_playlist = vec![String::from("Hello"), String::from("LEvels"), String::from("The Nights")];

    let mut playlist = files::list_songs("/home/arne-pi//Music/test");

    display::array(&playlist);

    stdout.flush().unwrap();


    let mut row = 0;
    let mut previous_row = 0;

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('q') => break,
            Key::Up => {
               if row <= 0 {
                    return;
                }

                previous_row = row;
                row -= 1;
                playlist = display::highlight(row, previous_row, playlist);
            },
            Key::Down => {
                if row >= playlist.len() {
                    return;
                }

                previous_row = row;
                row += 1;
                playlist = display::highlight(row, previous_row, playlist);
            },
            Key::Char(c) => files::log("pressed key"),
            _ => println!("other"),
        }

        stdout.flush().unwrap();
    }

    write!(stdout, "{}{}{}", termion::cursor::Show, termion::clear::All, termion::cursor::Goto(1,1)).unwrap();


}


