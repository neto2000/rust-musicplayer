
use termion::{self};

use crate::files;



const VERTICAL_LINE: char = '\u{2502}';
const HORIZONTAL_LINE: char = '\u{2500}';

const CORNER_TOP_LEFT: char = '\u{256D}';
const CORNER_TOP_RIGHT: char = '\u{256E}';
const CORNER_BOTTOM_LEFT: char = '\u{2570}';
const CORNER_BOTTOM_RIGHT: char = '\u{256F}';


pub fn frame() {
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

            //print!("{}", termion::cursor::Goto(j + 1, i + 1));

        }

    }

}

pub fn clear() {

    let (columns, rows) = termion::terminal_size().unwrap();

    for i in 2..rows-1 {
        print!("{}", termion::cursor::Goto(3, i));

        print!("                        ");
    }
}

pub fn array(array: &Vec<String>) {

    
    let (columns, rows) = termion::terminal_size().unwrap();

    let mut counter = 0;


    for song in array {
        

        if counter > rows {
            break;
        }

        print!("{}", termion::cursor::Goto(3, counter + 2));

        print!("{}", song);


        counter += 1;
    }
}

pub fn highlight(index: usize, previous: usize , array: Vec<String>) -> Vec<String> {
    files::log(&array[index]);


    print!("{}", termion::cursor::Goto(3, previous as u16 + 2));

    print!("{}", array[previous]);

    
    print!("{}{}", termion::cursor::Goto(3, index as u16 + 2), termion::color::Bg(termion::color::LightBlack));

    print!("{}", array[index]);

    print!("{}", termion::color::Bg(termion::color::Reset));



    
    return array;
}
