use std::fs::File;
use std::io::Read;

fn main() {
    let mut floor = 0;
    let mut counter = 0;
    let mut basement = false;

    let mut s = String::new();
    let mut file = File::open("input").unwrap();

    file.read_to_string(&mut s).unwrap();

    for token in s.chars() {
        match token {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => {}
        }

        counter += 1;

        if floor == -1 && basement == false {
            println!("Santa entered the basement at position {}", counter);
            basement = true;
        }
    }

    println!("Santa ended on floor {}", floor);
}
