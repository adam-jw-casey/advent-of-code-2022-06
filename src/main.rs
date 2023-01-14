use advent_of_code_2022_06::find_packet_marker;
use std::env;
use std::fs;

fn main() {
    let file_path = &env::args().collect::<Vec<_>>()[2];
    let contents = fs::read_to_string(file_path)
        .expect("Should be able to read the file");

    println!("Found marker after receiving {} packets!", find_packet_marker(&contents));
}
