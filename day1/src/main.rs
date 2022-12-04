fn main() {
    let input = include_str!("./input.txt");
    println!("Part 1: {}", part_one(input));
    println!("Part 2: {}", part_two(input));
}

fn get_individual_elf_calorie_sum(s: &str) -> u32 {
    s.lines()
        .map(|x| x.parse::<u32>().unwrap())
        .sum::<u32>()
}

fn part_one(input: &str) -> u32 {
    input.split("\n\n")
        .map(get_individual_elf_calorie_sum)
        .max()
        .unwrap()
}

fn part_two(input: &str) -> u32 {
    let mut calories_sorted = input.split("\n\n")
        .map(get_individual_elf_calorie_sum)
        .collect::<Vec<u32>>();

    // Sort calories in descending order
    calories_sorted.sort_by(|a, b| b.cmp(a));
    calories_sorted.iter().take(3).sum()
}
