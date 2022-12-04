use itertools::Itertools;

fn main() {
    let total_score: u64 = include_str!("../../input.txt")
        .lines()
        .map(|game| game.chars().next_tuple().expect("Unexpected game"))
        .map(|(theirs, _, res)| {
            let theirs: Shape = theirs.into();
            let res: GameResult = res.into();
            game_score(theirs, our_shape(theirs, res))
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
            'A' => Shape::Rock,
            'B' => Shape::Paper,
            'C' => Shape::Scissors,
            c => panic!("Unxepected shape: {:?}", c),
        }
    }
}

enum GameResult {
    Win = 6,
    Lose = 0,
    Draw = 3,
}

impl From<char> for GameResult {
    fn from(value: char) -> Self {
        match value {
            'X' => GameResult::Lose,
            'Y' => GameResult::Draw,
            'Z' => GameResult::Win,
            c => panic!("Unxepected result: {:?}", c),
        }
    }
}

fn our_shape(theirs: Shape, res: GameResult) -> Shape {
    match (theirs, res) {
        (Shape::Rock, GameResult::Win)
        | (Shape::Paper, GameResult::Draw)
        | (Shape::Scissors, GameResult::Lose) => Shape::Paper,

        (Shape::Rock, GameResult::Lose)
        | (Shape::Paper, GameResult::Win)
        | (Shape::Scissors, GameResult::Draw) => Shape::Scissors,

        (Shape::Rock, GameResult::Draw)
        | (Shape::Paper, GameResult::Lose)
        | (Shape::Scissors, GameResult::Win) => Shape::Rock,
    }
}

fn game_score(theirs: Shape, ours: Shape) -> u64 {
    let outcome = match (ours, theirs) {
        (Shape::Rock, Shape::Rock)
        | (Shape::Paper, Shape::Paper)
        | (Shape::Scissors, Shape::Scissors) => GameResult::Draw,
        (Shape::Rock, Shape::Paper)
        | (Shape::Paper, Shape::Scissors)
        | (Shape::Scissors, Shape::Rock) => GameResult::Lose,
        (Shape::Rock, Shape::Scissors)
        | (Shape::Paper, Shape::Rock)
        | (Shape::Scissors, Shape::Paper) => GameResult::Win,
    };
    outcome as u64 + ours as u64
}
