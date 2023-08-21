use termion::{self};

use crate::files;



pub struct Point {
    pub x: usize,
    pub y: usize,
}
pub struct Ratio {
    pub x: f32,
    pub y: f32,
}

pub struct Config {
    display_table: Vec<Vec<Ratio>>,

    files_pos: Point,
    control_pos: Point,

    term_size: Point,
}

impl Config {
    pub fn new(files_ratio: Ratio, control_ratio: Ratio, files_pos: Point, control_pos: Point) -> Self {

        
        let (columns, rows) = termion::terminal_size().unwrap();

        let term_size: Point = Point{x: columns as usize, y: rows as usize};

        let mut display_table: Vec<Vec<Ratio>> = Vec::new();

        for i in 0..3 {
            display_table.push(vec![Ratio{x:0.0,y:0.0},Ratio{x:0.0,y:0.0},Ratio{x:0.0,y:0.0}]);
        }

        display_table[files_pos.y][files_pos.x] = files_ratio;

        display_table[control_pos.y][control_pos.x] = control_ratio;
        


        Self { display_table: display_table, files_pos: files_pos, control_pos: control_pos, term_size: term_size }
    }
    fn get_width(&self, pos: &Point) -> Point {
        
        return Point {
            y: (self.term_size.y as f32 * self.display_table[pos.y][pos.x].y) as usize,
            x: (self.term_size.x as f32 * self.display_table[pos.y][pos.x].x) as usize
        }
    }

    fn get_start_point(&self, pos: &Point) -> Point {
        
        let mut origin: Point = Point{x: 0, y: 0};

        let mut counter = pos.y as usize;

        let (columns, rows) = termion::terminal_size().unwrap();

        while counter > 0 {
            
            counter -= 1;

            origin.y += (rows as f32 * self.display_table[counter][pos.x].y) as usize;

        }

        counter = pos.x as usize;

        while counter > 0 {
            
            counter -= 1;

            origin.x += (columns as f32 * self.display_table[pos.y][counter].x) as usize;

        }
        return origin
    }
}


const CIRCLE: char = '\u{26AB}';

const VERTICAL_LINE: char = '\u{2502}';
const HORIZONTAL_LINE: char = '\u{2500}';

const CORNER_TOP_LEFT: char = '\u{256D}';
const CORNER_TOP_RIGHT: char = '\u{256E}';
const CORNER_BOTTOM_LEFT: char = '\u{2570}';
const CORNER_BOTTOM_RIGHT: char = '\u{256F}';




pub fn frame(CONFIG: &Config) {
    //define later with .config file 
        
    


    let (columns, rows) = termion::terminal_size().unwrap();

    render_frame(CONFIG.get_start_point(&CONFIG.files_pos), CONFIG.get_width(&CONFIG.files_pos));

    render_frame(CONFIG.get_start_point(&CONFIG.control_pos), CONFIG.get_width(&CONFIG.control_pos));
}

fn render_frame(origin: Point, dimension: Point) {
    
    files::log(&dimension.x.to_string());

    print!("{}", termion::cursor::Goto(origin.x as u16 + 1, origin.y as u16 + 1));


    for i in origin.y..origin.y + dimension.y {
        
        for j in origin.x..origin.x + dimension.x {
    
            if i == origin.y {

                if j == origin.x {
                    print!("{}", CORNER_TOP_LEFT);

                    continue;
                }
                else if j == origin.x + dimension.x - 1 {
                    print!("{}", CORNER_TOP_RIGHT);
                    
                    print!("{}", termion::cursor::Goto(origin.x as u16 + 1, i as u16 + 2));
    
                    continue;
                }
                else {
                    print!("{}", HORIZONTAL_LINE);

                    continue;
                }
            }
            
            if i == origin.y + dimension.y - 1 {
                if j == origin.x {
                    print!("{}", CORNER_BOTTOM_LEFT);

                    continue;
                }
                else if j == origin.x + dimension.x - 1 {
                    print!("{}", CORNER_BOTTOM_RIGHT);

                    continue;
                }
                else {
                    print!("{}", HORIZONTAL_LINE);

                    continue;
                }

            }
    
            if j == origin.x {

                print!("{}", VERTICAL_LINE);

                continue;
            }
            else if j == origin.x + dimension.x - 1 {
                print!("{}", VERTICAL_LINE);

                print!("{}", termion::cursor::Goto(origin.x as u16 + 1, i as u16 + 2));

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

pub fn clear(config: &Config, pos: Point) {

    let origin: Point = config.get_start_point(&pos);
    let dimension: Point = config.get_width(&pos);

    for i in origin.y + 2..origin.y + dimension.y - 1 {
        print!("{}", termion::cursor::Goto(3, i as u16));
        
        for _j in origin.x + 2..origin.x + dimension.x - 1 {
            print!(" ");
        }
    }
}

pub fn array(config: &Config, pos: Point, array: &Vec<String>) {

    
    let origin: Point = config.get_start_point(&pos);

    let (columns, rows) = termion::terminal_size().unwrap();

    let mut counter = 0;


    for song in array {
        

        if counter > rows {
            break;
        }

        print!("{}", termion::cursor::Goto(origin.x as u16 + 3, origin.y as u16 + counter + 2));

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

pub fn timeline(config: &Config, pos: Point, percantage: f32) {
    let origin: Point = config.get_start_point(&pos);
    let dimension: Point = config.get_width(&pos);


    let row = dimension.y as f32 * 0.75 + origin.y as f32;

    let row = row as u16;

    let line_position = (dimension.x - 9) as f32 * percantage;

    
    print!("{}", termion::cursor::Goto(origin.x as u16 + 5, row));

    for i in 0..dimension.x - 9 {

        if i == line_position as usize {
    
            print!("{}", CIRCLE);
        }
        else if i < line_position as usize {
            
            print!("{}", HORIZONTAL_LINE);
        }
        else {
            print!(" ");
        }
    }


}


