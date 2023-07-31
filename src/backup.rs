use termsize;

fn main() {

    let argu = std::env::args().nth(1).expect("No arguments given!");



    print!("{e}[2J{e}[1;1H", e = 27 as char);



    let dim = termsize::get().unwrap();

    //println!("X: {}", dim.rows);

    

    //println!("arguments: {argu}");

    //println!("{}",'\u{2502}');

    for i in 0..dim.rows {

        println!("{}",'\u{2502}');
    }

    
    print!("\u{001B}[?251");

}
