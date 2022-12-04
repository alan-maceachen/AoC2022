use itertools::Itertools;

fn main() {
    let total_priority = include_str!("../../input.txt")
        .lines()
        .tuples()
        .filter_map(|(bag1, bag2, bag3)| {
            for i in bag1.chars() {
                if bag2.contains(i) && bag3.contains(i) {
                    return Some(i);
                }
            }
            None
        })
        .map(priority)
        .sum::<u64>();

    println!("Total priority: {:?}", total_priority);
}

fn priority(c: char) -> u64 {
    (if c.is_uppercase() {
        c as u8 + 26 - b'A'
    } else {
        c as u8 - b'a'
    }) as u64
        + 1 // Priority starts at 1
}
