use termion::{self};

use crate::files;


#[derive(Clone, Copy)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}
pub struct Ratio {
    pub x: f32,
    pub y: f32,
}


#[derive(PartialEq, Eq)]
pub enum Windows {
    Files,
    Control,
}

pub struct Config {
    display_table: Vec<Vec<Ratio>>,

    files_pos: Point,
    control_pos: Point,

    pub files_width: Point,
    control_width: Point,

    files_start: Point,
    control_start: Point,

    term_size: Point,

    pub top_index: usize,
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

        


        let files_width = get_width(&files_pos, &term_size, &display_table);
        
        let control_width = get_width(&control_pos, &term_size, &display_table);


        let files_start = get_start_point(&files_pos, &display_table);

        let control_start = get_start_point(&control_pos, &display_table);


        Self { 
            display_table: display_table, 
            files_pos: files_pos, 
            control_pos: control_pos, 
            term_size: term_size ,
            files_width: files_width,
            files_start: files_start,
            control_width: control_width,
            control_start: control_start,
            top_index: 0,
        }
    }

    pub fn set_top_index(&mut self, index: usize) {

        self.top_index = index;

    }
    
}

fn get_width(pos: &Point, term_size: &Point, display_table: &Vec<Vec<Ratio>>) -> Point {
        
    return Point {
        y: (term_size.y as f32 * display_table[pos.y][pos.x].y) as usize,
        x: (term_size.x as f32 * display_table[pos.y][pos.x].x) as usize
    }
}

fn get_start_point(pos: &Point, display_table: &Vec<Vec<Ratio>>) -> Point {
    
    let mut origin: Point = Point{x: 0, y: 0};

    let mut counter = pos.y as usize;

    let (columns, rows) = termion::terminal_size().unwrap();

    while counter > 0 {
        
        counter -= 1;

        origin.y += (rows as f32 * display_table[counter][pos.x].y) as usize;

    }

    counter = pos.x as usize;

    while counter > 0 {
        
        counter -= 1;

        origin.x += (columns as f32 * display_table[pos.y][counter].x) as usize;

    }
    return origin
}

const CIRCLE: char = '\u{26AB}';

const VERTICAL_LINE: char = '\u{2502}';
const HORIZONTAL_LINE: char = '\u{2500}';

const CORNER_TOP_LEFT: char = '\u{256D}';
const CORNER_TOP_RIGHT: char = '\u{256E}';
const CORNER_BOTTOM_LEFT: char = '\u{2570}';
const CORNER_BOTTOM_RIGHT: char = '\u{256F}';


const PLAY: char = '\u{25B6}';
const PAUSE: char = '\u{2016}';



impl Config {

    pub fn frame(&self) {
        //define later with .config file 
            
        


        let (columns, rows) = termion::terminal_size().unwrap();

        self.render_frame(self.files_start, self.files_width);

        self.render_frame(self.control_start, self.control_width);
    }

    fn render_frame(&self, origin: Point, dimension: Point) {
        
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

    pub fn clear(&self, window: Windows) {

        let mut origin;
        let mut dimension;

        if window == Windows::Files {
            origin = self.files_start;
            dimension = self.files_width;
        }
        else {
            origin = self.control_start;
            dimension = self.control_width;

        }

        for i in origin.y + 2..origin.y + dimension.y - 1 {
            print!("{}", termion::cursor::Goto(3, i as u16));
            
            for _j in origin.x + 2..origin.x + dimension.x - 1 {
                print!(" ");
            }
        }
    }

    pub fn array(&self, array: &Vec<String>, path: &str) {

        
        let origin: Point = self.files_start;

        let (_columns, rows) = termion::terminal_size().unwrap();

        let mut current_row: u16 = 0;


        for i in self.top_index..array.len() {
            

            if current_row > self.files_width.y as u16 {
                break;
            }

            if path == "playlist" {
                
                print!("{}", termion::cursor::Goto(origin.x as u16 + 3 , origin.y as u16 + current_row + 2));

                print!("{}", array[i]);
            }
            else {
                self.write_song_info(path, &array[i], current_row);
            }
            

            current_row += 1;
        }
    }

    pub fn highlight(&self, index: usize, previous: usize , array: Vec<String>, path: &str) -> Vec<String> {
        files::log(&array[index]);


        
        if path == "playlist" {

            print!("{}", termion::cursor::Goto(3, previous as u16 - self.top_index as u16 + 2));
            
            print!("{}", array[previous]);
        }
        else {

            self.write_song_info(path, &array[previous], previous as u16 - self.top_index as u16);
        }


        
        print!("{}{}", termion::cursor::Goto(3, index as u16 - self.top_index as u16 + 2), termion::color::Bg(termion::color::LightBlack));
        

        if path == "playlist" {
            
            print!("{}", array[index]);

        }
        else {
            self.write_song_info(path, &array[index], index as u16 - self.top_index as u16)
        }


        print!("{}", termion::color::Bg(termion::color::Reset));



        
        return array;
    }

    fn write_song_info(&self, path: &str, song: &str, counter: u16) {
       
        let origin: Point = self.files_start;

        let song_file: String = path.to_owned() + "/" + song;

        let song_info: files::SongInfo = files::song_info(&song_file); 

        let mut info_position: u16 = 0;

    
        // Title

        print!("{}", termion::cursor::Goto(origin.x as u16 + 3 + info_position, origin.y as u16 + counter + 2));
        
        if song_info.title == "" {
            print!("{}", song);
        }
        else {

            print!("{}", &song_info.title);
        }
        
        info_position += (self.files_width.x as f32 * 0.3) as u16;
        

        print!("{}", termion::color::Bg(termion::color::Reset));

        // Artist
        
        print!("{}", termion::cursor::Goto(origin.x as u16 + 3 + info_position, origin.y as u16 + counter + 2));

        print!("{}", &song_info.artist);

        info_position += (self.files_width.x as f32 * 0.2) as u16;


        // Genre

        print!("{}", termion::cursor::Goto(origin.x as u16 + 3 + info_position, origin.y as u16 + counter + 2));

        print!("{}", &song_info.genre);
    
        info_position += (self.files_width.x as f32 * 0.2) as u16;


        // Year

        print!("{}", termion::cursor::Goto(origin.x as u16 + 3 + info_position, origin.y as u16 + counter + 2));

        print!("{}", &song_info.year);

    } 

    pub fn timeline(&self, percantage: f32) {
        let origin: Point = self.control_start;
        let dimension: Point = self.control_width;


        let row = dimension.y as f32 * 0.75 + origin.y as f32;

        let mut row = row as u16;

        if row > (dimension.y - 4 + origin.y) as u16 {
            row = (dimension.y - 4 + origin.y) as u16;
        }


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

    pub fn play_pause(&self, pause: bool) {
        
        let origin: Point = self.control_start;
        let dimension: Point = self.control_width;


        let row = dimension.y as f32 * 0.75 + origin.y as f32 + 2.0;

        let mut row = row as u16;

        if row > (dimension.y - 2 + origin.y) as u16 {
            row = (dimension.y - 2 + origin.y) as u16;
        }


        let mid = (dimension.x as f32 * 0.5 + origin.x as f32) as u16;


        print!("{}", termion::cursor::Goto(mid, row));

        if pause {
            print!("{}", PAUSE);
        }
        else {

            print!("{}", PLAY);
        }
    }

    pub fn title(&self, title: &str) {
        let origin: Point = self.control_start;
        let dimension: Point = self.control_width;


        let row = dimension.y as f32 * 0.75 + origin.y as f32 - 2.0;

        let mut row = row as u16;

        //if row > (dimension.y - 2 + origin.y) as u16 {
        //    row = (dimension.y - 2 + origin.y) as u16;
        //}
        //

        for i in origin.x + 2..origin.x + dimension.x {
            
            print!("{}", termion::cursor::Goto(i as u16, row));

            print!(" ");
        }


        let mid = (dimension.x as f32 * 0.5 + origin.x as f32) as u16;


        let cursor_pos = mid - (title.len() as f32 * 0.5) as u16;

        
        print!("{}", termion::cursor::Goto(cursor_pos, row));

        print!("{}", title);

    }

}
