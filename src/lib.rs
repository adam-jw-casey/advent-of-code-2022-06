use itertools::Itertools;

/// Finds the first occurence of length consecutive, different characters
/// and returns the index of the last of those characters
/// # Examples
/// ```
/// use advent_of_code_2022_06::find_packet_marker;
/// assert_eq!(find_packet_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb",     &4), 7);
/// assert_eq!(find_packet_marker("bvwbjplbgvbhsrlpgdmjqwftvncz",       &4), 5);
/// assert_eq!(find_packet_marker("nppdvjthqldpwncqszvftbrmjlhg",       &4), 6);
/// assert_eq!(find_packet_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",  &4), 10);
/// assert_eq!(find_packet_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",   &4), 11);
///
/// assert_eq!(find_packet_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb",     &14), 19);
/// assert_eq!(find_packet_marker("bvwbjplbgvbhsrlpgdmjqwftvncz",       &14), 23);
/// assert_eq!(find_packet_marker("nppdvjthqldpwncqszvftbrmjlhg",       &14), 23);
/// assert_eq!(find_packet_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",  &14), 29);
/// assert_eq!(find_packet_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",   &14), 26);
/// ```
pub fn find_packet_marker(stream: &str, length: &usize) -> usize{
    stream
        .as_bytes()
        .windows(*length)
        .enumerate()
        .filter(|(_i,x)| x.iter().unique().count() == *length)
        .next()
        .expect("There should be a packet start marker found")
        .0
        +*length
}
