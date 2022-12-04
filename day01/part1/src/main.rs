fn main() {
    let highest_calories = include_str!("../../input.txt")
        .split("\n\n")
        .map(|food_list| {
            food_list
                .lines()
                .map(|cal| str::parse::<i64>(cal).unwrap_or_default())
                .sum::<i64>()
        })
        .max();

    println!("Highest total calories: {:?}", highest_calories);
}
