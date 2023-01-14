use itertools::Itertools;

/// Finds the first occurence of 4 consecutive, different characters
/// and returns the index of the last of those characters
/// # Examples
/// ```
/// use advent_of_code_2022_06::find_packet_marker;
/// assert_eq!(find_packet_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
/// assert_eq!(find_packet_marker("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
/// assert_eq!(find_packet_marker("nppdvjthqldpwncqszvftbrmjlhg"), 6);
/// assert_eq!(find_packet_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
/// assert_eq!(find_packet_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
/// ```
pub fn find_packet_marker(stream: &str) -> usize{
    let length = 4;

    stream
        .as_bytes()
        .windows(length)
        .enumerate()
        .filter(|(_i,x)| x.iter().unique().count() == length)
        .next()
        .expect("There should be a packet start marker found")
        .0
        +length
}
