use std::fs::File;
use std::io::Read;

fn main() {
    // Initial floor #
    let mut floor = 0;

    // Count position
    let mut counter = 0;

    // True when Santa has visited basement
    let mut basement = false;

    // Create new string which will contain instructions
    let mut s = String::new();

    // Open input file
    let mut file = File::open("input").unwrap();

    // Read file to string
    file.read_to_string(&mut s).unwrap();

    // Loop through every character in s
    for c in s.chars() {
        match c {
            // Up
            '(' => floor += 1,

            // Down
            ')' => floor -= 1,

            _ => (),
        }

        // Increment counter
        counter += 1;

        // Check if it's the first time entering basement
        if floor == -1 && basement == false {
            // Print the position and set basement bool to true
            println!("Santa entered the basement at position {}", counter);
            basement = true;
        }
    }

    // Print result
    println!("Santa ended on floor {}", floor);
}
