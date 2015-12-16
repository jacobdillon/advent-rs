extern crate image;

use std::fs::File;
use std::io::Read;
use std::collections::HashSet;
use image::{GenericImage, ImageBuffer, Rgb, imageops};

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    // Up one unit
    fn up(&mut self) {
        self.y += 1;
    }

    // Down one unit
    fn down(&mut self) {
        self.y -= 1;
    }

    // Left one unit
    fn left(&mut self) {
        self.x -= 1;
    }

    // Right one unit
    fn right(&mut self) {
        self.x += 1;
    }
}

fn main() {
    // Boolean to check if current move is for santa or robo
    let mut is_robo = false;

    // Locations for part1_santa, santa, and robo starting at (0, 0)
    let mut part1_santa_loc: Point = Point { x: 0, y: 0 };
    let mut part2_santa_loc: Point = Point { x: 0, y: 0 };
    let mut part2_robo_loc: Point = Point { x: 0, y: 0 };

    // New HashSets for part1_santa, santa, and robo
    let mut part1_santa = HashSet::new();
    let mut part2_santa = HashSet::new();
    let mut part2_robo = HashSet::new();

    // Put initial values in HashSets (origins)
    part1_santa.insert(part1_santa_loc);
    part2_robo.insert(part2_robo_loc);

    // String for reading input to
    let mut s = String::new();

    // Open file
    let mut file = File::open("input").unwrap();

    // Read file into string
    file.read_to_string(&mut s).unwrap();

    // Width and height for images
    let width: u32 = 250;
    let height: u32 = 250;

    // Points for images
    let mut part1_santa_loc_img: Point = Point {
        x: width as i32 / 2,
        y: height as i32 / 2,
    };
    let mut part2_santa_loc_img: Point = Point {
        x: width as i32 / 2,
        y: height as i32 / 2,
    };
    let mut part2_robo_loc_img: Point = Point {
        x: width as i32 / 2,
        y: height as i32 / 2,
    };

    // Create ImageBuffers for part 1 and 2
    let mut part1_img = ImageBuffer::<Rgb<u8>>::new(width, height);
    let mut part2_img = ImageBuffer::<Rgb<u8>>::new(width, height);

    // Put initial points into ImageBuffers
    part1_img.get_pixel_mut(part1_santa_loc_img.x as u32, part1_santa_loc_img.y as u32).data =
        [255, 0, 0];
    part2_img.get_pixel_mut(part2_robo_loc_img.x as u32, part2_robo_loc_img.y as u32).data = [0,
                                                                                              0,
                                                                                              255];

    // Loop over every char in string
    for c in s.chars() {
        match c {
            // Right
            '>' => {
                part1_santa_loc.right();
                part1_santa_loc_img.right();
                if !is_robo {
                    part2_santa_loc.right();
                    part2_santa_loc_img.right();
                } else {
                    part2_robo_loc.right();
                    part2_robo_loc_img.right();
                }
            }

            // Left
            '<' => {
                part1_santa_loc.left();
                part1_santa_loc_img.left();
                if !is_robo {
                    part2_santa_loc.left();
                    part2_santa_loc_img.left()
                } else {
                    part2_robo_loc.left();
                    part2_robo_loc_img.left();
                }
            }

            // Down
            'v' => {
                part1_santa_loc.down();
                part1_santa_loc_img.down();
                if !is_robo {
                    part2_santa_loc.down();
                    part2_santa_loc_img.down();
                } else {
                    part2_robo_loc.down();
                    part2_robo_loc_img.down();
                }
            }

            // Up
            '^' => {
                part1_santa_loc.up();
                part1_santa_loc_img.up();
                if !is_robo {
                    part2_santa_loc.up();
                    part2_santa_loc_img.up();
                } else {
                    part2_robo_loc.up();
                    part2_robo_loc_img.up();
                }
            }

            _ => (),
        }

        // Insert new part1 value into HashSet and image
        part1_santa.insert(part1_santa_loc);
        part1_img.get_pixel_mut(part1_santa_loc_img.x as u32, part1_santa_loc_img.y as u32).data =
            [255, 0, 0];

        // Decide which HashSet to insert value into and what color pixel should be
        if !is_robo {
            part2_santa.insert(part2_santa_loc);
            part2_img.get_pixel_mut(part2_santa_loc_img.x as u32, part2_santa_loc_img.y as u32)
                     .data = [255, 0, 0];
            is_robo = true;
        } else {
            part2_robo.insert(part2_robo_loc);
            part2_img.get_pixel_mut(part2_robo_loc_img.x as u32, part2_robo_loc_img.y as u32)
                     .data = [0, 0, 255];
            is_robo = false;
        }
    }

    // Print results
    println!("Houses visited by santa before robo: {}", part1_santa.len());
    println!("Houses visited by santa and robo: {}",
             part2_santa.union(&part2_robo).collect::<Vec<&Point>>().len());

    // Resize and save images
    imageops::resize(&part1_img, 2 * width, 2 * height, imageops::Nearest)
        .save("part1_image.png")
        .unwrap();
    imageops::resize(&part2_img, 2 * width, 2 * height, imageops::Nearest)
        .save("part2_image.png")
        .unwrap();
}
