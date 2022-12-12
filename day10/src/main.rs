use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", part_one(input));
    println!("Part 2:\n{}", part_two(input));
}

fn part_one(input: &str) -> i32 {
    const STARTING_SIGNAL_STENGTH: i32 = 20;
    const SIGNAL_STENGTH_INCREASE: i32 = 40;

    // It's easier to have the cycle start as '1' instead of '0' here
    // to check the signal strength and do multiplication with it.
    let mut cycle: u32 = 1;
    let mut x_register: i32 = 1;
    let mut processing: HashMap<u32, i32> = HashMap::new();

    let mut signal_strength_sum: i32 = 0;
    let mut signal_strength = STARTING_SIGNAL_STENGTH;

    let mut process_cycle_tick = |cycle: &mut u32, processing: &mut HashMap<u32, i32>| -> () {
        *cycle += 1;

        // Finish executing instructions started from previous cycles
        if let Some(instruction_value) = processing.get(&cycle) {
            x_register += instruction_value;
            processing.remove(&cycle);
        }

        // Report Signal Strength
        if *cycle == signal_strength.try_into().unwrap() {
            signal_strength_sum += x_register * signal_strength;
            signal_strength += SIGNAL_STENGTH_INCREASE;
        }
    };

    for instruction in input.lines() {
        if instruction == "noop" {
            process_cycle_tick(&mut cycle, &mut processing);
            continue;
        }

        let mut instruction = instruction.split_whitespace();
        instruction.next(); // Consume the "addx" instruction
        if let Some(amount) = instruction.next() {
            let amount = amount.parse::<i32>().unwrap();
            processing.insert(cycle + 2, amount);
            process_cycle_tick(&mut cycle, &mut processing);
            process_cycle_tick(&mut cycle, &mut processing);
        }
    }

    signal_strength_sum
}

fn part_two(input: &str) -> String {
    const LINE_LENGTH: u32 = 40;

    // Start the cycle with 0 in part_two since the pixel positions correspond to the cycle
    let mut cycle: u32 = 0;
    let mut x_register: i32 = 1;
    let mut processing: HashMap<u32, i32> = HashMap::new();

    let mut crt_output = String::new();
    let mut current_line = String::new();

    let mut process_cycle_tick = |cycle: &mut u32, processing: &mut HashMap<u32, i32>| -> () {
        // Determine the pixel output from this current cycle
        let (left_sprite_pos, middle_spite_pos, right_sprite_pos) =
            (x_register - 1, x_register, x_register + 1);
        let pixel_position: i32 = (*cycle % LINE_LENGTH).try_into().unwrap();
        if pixel_position == left_sprite_pos
            || pixel_position == middle_spite_pos
            || pixel_position == right_sprite_pos
        {
            current_line.push_str("#");
        } else {
            current_line.push_str(".");
        }

        *cycle += 1;

        // Check if we need to start a new line
        if (*cycle % LINE_LENGTH) == 0 {
            current_line.push_str("\n");
            crt_output.push_str(&current_line);
            current_line = String::new();
        }

        // Finish executing instructions started from previous cycles
        if let Some(instruction_value) = processing.get(&cycle) {
            x_register += instruction_value;
            processing.remove(&cycle);
        }
    };

    for instruction in input.lines() {
        if instruction == "noop" {
            process_cycle_tick(&mut cycle, &mut processing);
            continue;
        }

        let mut instruction = instruction.split_whitespace();
        instruction.next(); // Consume the "addx" instruction
        if let Some(amount) = instruction.next() {
            let amount = amount.parse::<i32>().unwrap();
            processing.insert(cycle + 2, amount);
            process_cycle_tick(&mut cycle, &mut processing);
            process_cycle_tick(&mut cycle, &mut processing);
        }
    }

    crt_output
}
