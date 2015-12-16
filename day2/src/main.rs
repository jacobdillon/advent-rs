use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::cmp::min;

fn main() {
    // Paper and ribbon variables
    let mut paper = 0;
    let mut ribbon = 0;

    // Open file and read it into buffer
    let f = File::open("input").unwrap();
    let file = BufReader::new(&f);

    // Loop through buffer
    for line in file.lines() {
        // Unwrap and split line at 'x's
        let ln = line.unwrap();
        let splitln = ln.split('x');

        // Read split values into vector
        let vec: Vec<&str> = splitln.collect();

        // Parse and read vector values into l, w, and h
        let l = vec[0].parse::<i32>().unwrap();
        let w = vec[1].parse::<i32>().unwrap();
        let h = vec[2].parse::<i32>().unwrap();

        // Get the smallest side
        let smallest_side = min(l * w, min(w * h, h * l));

        // Calculate the surface area of the present
        paper += (2 * l * w) + (2 * w * h) + (2 * h * l);

        // Add the extra paper needed
        paper += smallest_side;

        // Calculate the volume of the present
        ribbon += l * w * h;

        // Add the extra ribbon needed
        if smallest_side == l * w {
            ribbon += l + l + w + w
        } else if smallest_side == w * h {
            ribbon += w + w + h + h
        } else if smallest_side == h * l {
            ribbon += h + h + l + l
        }
    }

    // Print result
    println!("The elves need {} square feet of paper", paper);
    println!("The elves need {} feet or ribbon", ribbon);
}
