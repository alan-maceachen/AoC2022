use itertools::Itertools;

fn main() {
    let top_highest_calories_total: i64 = include_str!("../../input.txt")
        .split("\n\n")
        .map(|x| {
            x.lines()
                .map(str::parse::<i64>)
                .map(|i| i.unwrap_or_default())
                .sum::<i64>()
        })
        .sorted()
        .rev()
        .take(3)
        .sum();

    println!(
        "Top Three Elves are carrying {:?} calories",
        top_highest_calories_total
    );
}
