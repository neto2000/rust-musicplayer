use std::io::Write;

use termion::{self, raw::IntoRawMode, event::Key, input::TermRead};


const VERTICAL_LINE: char = '\u{2502}';
const HORIZONTAL_LINE: char = '\u{2500}';

const CORNER_TOP_LEFT: char = '\u{256D}';
const CORNER_TOP_RIGHT: char = '\u{256E}';
const CORNER_BOTTOM_LEFT: char = '\u{2570}';
const CORNER_BOTTOM_RIGHT: char = '\u{256F}';



fn main() {
    println!("hello");
    println!("servus");


    println!("{}",termion::clear::All);

    let stdin = std::io::stdin();

    let mut stdout = std::io::stdout().into_raw_mode().unwrap();

    write!(stdout, "{}{}", termion::cursor::Goto(1,1), termion::cursor::Hide).unwrap();

    stdout.flush().unwrap();


    //println!("{}{}", '\u{256D}', '\u{2500}');
    //write!(stdout, "{}", termion::cursor::Goto(1,2)).unwrap();
    //println!("{}", '\u{2502}');

    print_frame();


    stdout.flush().unwrap();

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('q') => break,
            Key::Char(c) => println!("{}", c),
            _ => println!("other"),
        }

        stdout.flush().unwrap();
    }

    write!(stdout, "{}{}{}", termion::cursor::Show, termion::clear::All, termion::cursor::Goto(1,1)).unwrap();

}

fn print_frame() {

    let (columns, rows) = termion::terminal_size().unwrap();

    for i in 0..rows {
        
        for j in 0..columns {
    
            if i == 0 {

                if j == 0 {
                    print!("{}", CORNER_TOP_LEFT);

                    continue;
                }
                else if j == columns - 1 {
                    print!("{}", CORNER_TOP_RIGHT);

                    continue;
                }
                else {
                    print!("{}", HORIZONTAL_LINE);

                    continue;
                }
            }
            
            if i == rows - 1 {
                if j == 0 {
                    print!("{}", CORNER_BOTTOM_LEFT);

                    continue;
                }
                else if j == columns - 1 {
                    print!("{}", CORNER_BOTTOM_RIGHT);

                    continue;
                }
                else {
                    print!("{}", HORIZONTAL_LINE);

                    continue;
                }

            }
    
            if j == 0 || j == columns - 1 {
                print!("{}", VERTICAL_LINE);

                continue;
            }
            else {
                print!(" ");

                continue;
            }

            print!("{}", termion::cursor::Goto(j + 1, i + 1));

        }

    }

}



