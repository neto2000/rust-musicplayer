fn main() {
    println!("Hello, world!");

    let argu = std::env::args().nth(1).expect("No arguments given!");

    println!("argument: {}", argu);

    print!("{}[2J", 27 as char);

    println!("lol");
}
