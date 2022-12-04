fn main() {
    let total_priority = include_str!("../../input.txt")
        .lines()
        .filter_map(|bag| {
            let (c1, c2) = bag.split_at(bag.len() / 2);
            for i in c1.chars() {
                if c2.contains(i) {
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
