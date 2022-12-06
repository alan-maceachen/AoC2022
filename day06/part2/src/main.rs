use itertools::Itertools;

const LEN: usize = 14;

fn main() {
    let input = include_str!("../../input.txt")
        .chars()
        .collect::<Vec<_>>()
        .windows(LEN)
        .enumerate()
        .find_map(|(n, i)| {
            if i.iter().all_unique() {
                Some(n + LEN)
            } else {
                None
            }
        })
        .expect("value not found");

    println!("start-of-message marker: {input}");
}
