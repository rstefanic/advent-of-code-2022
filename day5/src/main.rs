use std::collections::HashMap;
use std::collections::LinkedList;

struct Instruction {
    amount: u32,
    source: u32,
    destination: u32,
}

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", part_one(input));
    println!("Part 2: {}", part_two(input));
}

fn build_stack(input: &str) -> HashMap<u32, LinkedList<char>> {
    let mut stacks: HashMap<u32, LinkedList<char>> = HashMap::new();

    // Get the stack diagram from the input. We want everything until the first
    // empty line which separates the stack diagram from the instructions.
    let mut stack_diagram = input
        .lines()
        .take_while(|row| !row.is_empty())
        .collect::<Vec<&str>>();

    // The last line of the diagram has the number of stacks. We'll use these
    // numbers as the keys for our HashMap to keep track of the stacks.
    if let Some(stack_numbers) = stack_diagram.pop() {
        stack_numbers
            .split_whitespace()
            .into_iter()
            .for_each(|number| {
                if let Ok(n) = number.parse::<u32>() {
                    stacks.insert(n, LinkedList::new());
                }
            });
    }

    // Iterate through the diagram from the bottom up so that the stack order is correct.
    // Starting from the bottom of the stacks, we'll add the crate label characters.
    while let Some(row) = stack_diagram.pop() {
        let mut current_stack = 1;

        // Chunk by 4 so that each chunk looks like this: "[Z] ", "[X] ", etc.
        row.as_bytes().chunks(4).for_each(|b| {
            // NOTE: Given the input, we can safely assume b[1] is valid
            let c = b[1] as char;

            // If it's a character and not a space, then we'll add it to this stack
            if c.is_alphabetic() {
                if let Some(stack) = stacks.get_mut(&current_stack) {
                    stack.push_back(c);
                }
            }

            current_stack += 1;
        });
    }

    stacks
}

fn build_instructions<'a>(input: &'a str) -> impl Iterator<Item = Instruction> + 'a {
    input
        .lines()
        // The first row with the word "move" is the start of the instructions
        .skip_while(|row| !row.contains("move"))
        .flat_map(|instruction| {
            // Since every instruction is of the format "move 1 from 8 to 9",
            // we can just focus on the number positions to get the information
            // we need. The flat_map call here is used to automatically filter
            // out the words since the call to parse will fail for the words.
            if let [amount, source, destination] = instruction
                .split_whitespace()
                .flat_map(|word| word.parse::<u32>())
                .collect::<Vec<u32>>()[..]
            {
                return Some(Instruction {
                    amount,
                    source,
                    destination,
                });
            }

            None
        })
}

fn remove_crates_from_source_stack(
    stacks: &mut HashMap<u32, LinkedList<char>>,
    instruction: &Instruction,
) -> Vec<char> {
    let mut crates_to_move = Vec::new();

    if let Some(source_stack) = stacks.get_mut(&instruction.source) {
        for _ in 0..instruction.amount {
            if let Some(c) = source_stack.pop_back() {
                crates_to_move.push(c);
            }
        }
    }

    crates_to_move
}

fn get_top_crate_stack_letters(stacks: &mut HashMap<u32, LinkedList<char>>) -> String {
    let mut result = String::new();

    // Grab the last element from each stack add it to our result
    for i in 1..=stacks.keys().len() {
        if let Some(stack) = stacks.get_mut(&(i as u32)) {
            if let Some(c) = stack.pop_back() {
                result.push(c);
            }
        }
    }

    result
}

fn part_one(input: &str) -> String {
    let mut stacks = build_stack(input);
    let instructions = build_instructions(input);

    instructions.for_each(|instruction| {
        let removed_crates = remove_crates_from_source_stack(&mut stacks, &instruction);

        if let Some(destination_stack) = stacks.get_mut(&instruction.destination) {
            // Stack the removed crates onto the destination stack in the order they were removed
            for c in removed_crates {
                destination_stack.push_back(c);
            }
        }
    });

    get_top_crate_stack_letters(&mut stacks)
}

fn part_two(input: &str) -> String {
    let mut stacks = build_stack(input);
    let instructions = build_instructions(input);

    instructions.for_each(|instruction| {
        let mut removed_crates = remove_crates_from_source_stack(&mut stacks, &instruction);

        if let Some(destination_stack) = stacks.get_mut(&instruction.destination) {
            // Move through the removed_crates backwards to stack the crates
            // while keeping the same order from the source stack.
            while let Some(c) = removed_crates.pop() {
                destination_stack.push_back(c);
            }
        }
    });

    get_top_crate_stack_letters(&mut stacks)
}
