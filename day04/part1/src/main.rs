use itertools::Itertools;

fn main() {
    let overlaps = include_str!("../../input.txt")
        .lines()
        .filter(|line| {
            let (first, second) = line
                .split(',')
                .map(|ranges| {
                    ranges
                        .split('-')
                        .map(|i| str::parse::<i64>(i).expect("Expected i64"))
                        .next_tuple::<(_, _)>()
                        .map(|(l, u)| l..=u)
                        .expect("Expected range shape {i64}-{i64}")
                })
                .next_tuple::<(_, _)>()
                .expect("Expected row shape {i64}-{i64},{i64}-{i64}");
            first.clone().into_iter().all(|i| (&second).contains(&i))
                || second.into_iter().all(|i| (&first).contains(&i))
        })
        .count();

    println!("Amount whole of overlaps: {:?}", overlaps);
}
