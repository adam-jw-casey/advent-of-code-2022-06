use advent_of_code_2022_06::find_packet_marker;
use std::env;
use std::fs;

fn main() {
    let file_path = &env::args().collect::<Vec<_>>()[1];
    let contents = fs::read_to_string(file_path)
        .expect("Should be able to read the file");

    println!("Found start of packet marker after receiving {} packets!", find_packet_marker(&contents, &4));
    println!("Found start of message marker after receiving {} packets!", find_packet_marker(&contents, &14));
}
