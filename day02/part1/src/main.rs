use itertools::Itertools;

fn main() {
    let total_score: u64 = include_str!("../../input.txt")
        .lines()
        .map(|game| game.chars().next_tuple().expect("Unexpected game"))
        .map(|(t, _, o)| {
            let theirs: Shape = t.into();
            let ours: Shape = o.into();
            game_score(theirs, ours)
        })
        .sum();

    println!("Total Score: {:?}", total_score);
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl From<char> for Shape {
    fn from(value: char) -> Self {
        match value {
            'A' | 'X' => Shape::Rock,
            'B' | 'Y' => Shape::Paper,
            'C' | 'Z' => Shape::Scissors,
            c => panic!("Unxepected shape: {:?}", c),
        }
    }
}
fn game_score(theirs: Shape, ours: Shape) -> u64 {
    let outcome = match (ours, theirs) {
        (Shape::Rock, Shape::Rock)
        | (Shape::Paper, Shape::Paper)
        | (Shape::Scissors, Shape::Scissors) => 3,
        (Shape::Rock, Shape::Paper)
        | (Shape::Paper, Shape::Scissors)
        | (Shape::Scissors, Shape::Rock) => 0,
        (Shape::Rock, Shape::Scissors)
        | (Shape::Paper, Shape::Rock)
        | (Shape::Scissors, Shape::Paper) => 6,
    };
    outcome + ours as u64
}
