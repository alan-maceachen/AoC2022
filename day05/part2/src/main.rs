use std::num::ParseIntError;

use itertools::Itertools;

fn main() {
    let input = include_str!("../../input.txt");

    let starting_stacks: [Vec<char>; 9] = input
        .lines()
        .take_while(|line| line.contains('['))
        .flat_map(|line| {
            line.chars()
                .skip(1)
                .step_by(4)
                .enumerate()
                .collect::<Vec<_>>()
        })
        .filter(|&(_, c)| c != ' ')
        .fold(Default::default(), |mut stacks, (stack, c)| {
            stacks[stack].push(c);
            stacks
        });

    let msg: String = input
        .split("\n\n")
        .skip(1)
        .flat_map(|input| {
            input.lines().flat_map(|line| {
                line.split(' ')
                    .skip(1)
                    .step_by(2)
                    .tuples()
                    .flat_map(|(n, s1, s2)| {
                        Ok::<(usize, usize, usize), ParseIntError>((
                            n.parse()?,
                            s1.parse()?,
                            s2.parse()?,
                        ))
                    })
            })
        })
        .try_fold(starting_stacks, |mut stacks, (n_crates, s1, s2)| {
            let mut moving_crates: Vec<_> = stacks
                .get_mut(s1 - 1)
                .ok_or(())?
                .drain(..n_crates)
                .collect();

            moving_crates.append(stacks.get_mut(s2 - 1).ok_or(())?);
            stacks[s2 - 1] = moving_crates;
            Ok::<_, ()>(stacks)
        })
        .expect("Failed to find stack")
        .into_iter()
        .map(|list| *list.first().expect("No crate found after moves"))
        .collect();

    println!("Final crates: {:?}", msg);
}
