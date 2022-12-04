fn main() {
    let input = include_str!("./input.txt");
    println!("Part 1: {}", part_one(input));
    println!("Part 2: {}", part_two(input));
}

fn get_item_priority(item: char) -> u32 {
    // NOTE: Lowercase item types a through z have priorities 1 through 26.
    // Uppercase item types A through Z have priorities 27 through 52.
    match item {
        'A'..='Z' => (item as u32) - 38,
        'a'..='z' => (item as u32) - 96,
        _ => panic!("Unknown item")
    }
}

fn part_one(input: &str) -> u32 {
    input.lines().flat_map(|rucksack| {
        let compartment_size = rucksack.chars().count() / 2;
        let (first_compartment, second_compartment) = rucksack.split_at(compartment_size);

        for item in first_compartment.chars() {
            if second_compartment.contains(item) {
                return Some(get_item_priority(item))
            }
        }

        // NOTE: With valid input, we should never reach this, but just
        // in case both compartments don't have a shared item...
        None
    }).sum()
}

fn part_two(input: &str) -> u32 {
    input.lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .flat_map(|group| {
            // NOTE: We're always assuming that the group has at least three members
            let (first_rucksack, second_rucksack, third_rucksack) = (group[0], group[1], group[2]);

            for item in first_rucksack.chars() {
                if second_rucksack.contains(item) && third_rucksack.contains(item) {
                    return Some(get_item_priority(item))
                }
            }

            // NOTE: Again, we're assuming the input's always valid where
            // all three rucksacks have at least ONE shared item.
            // Ideally, we'll never reach this point.
            None
        })
        .sum()
}
