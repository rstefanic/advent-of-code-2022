use std::collections::HashMap;

#[derive(Debug)]
struct Monkey {
    number: u8,
    items: Vec<u64>,
    operation: Operation,
    test: Divisible,
    true_condition_monkey: u64,
    false_condition_monkey: u64,
}

impl Monkey {
    pub fn try_parse(s: &str) -> Option<Self> {
        let mut lines = s.lines();

        // First line (e.g. "Monkey 1:")
        let (_, monkey_id) = lines.next()?.split_once(" ")?;
        let monkey_id = monkey_id.trim_end_matches(":").parse::<u8>().ok()?;

        // Second line (e.g. "Items: 1, 2, 3")
        let (_, items) = lines.next()?.split_once(":")?;
        let items = items.split(",").flat_map(|s| s.trim().parse::<u64>().ok()).collect::<Vec<_>>();

        // Third line (e.g. "Operation: new = old * 19")
        let (_, operation) = lines.next()?.split_once("=")?;
        let operation = Operation::parse(operation.trim()).unwrap();

        // Fourth line (e.g. "Test: divisible by 3")
        let (_, test) = lines.next()?.split_once("by")?;
        let test = Divisible::parse(test);

        // Fifth line (e.g. "If true: throw to monkey 2");
        let (_, true_condition_monkey) = lines.next()?.split_once("monkey")?;
        let true_condition_monkey = true_condition_monkey.trim().parse::<u64>().ok()?;

        // Sixth line (e.g. "If false: throw to monkey 3");
        let (_, false_condition_monkey) = lines.next()?.split_once("monkey")?;
        let false_condition_monkey = false_condition_monkey.trim().parse::<u64>().ok()?;

        Some(Self {
            number: monkey_id,
            items,
            operation,
            test,
            true_condition_monkey,
            false_condition_monkey,
        })
    }
}

#[derive(Debug)]
enum Value {
    Value(u64),
    Old,
}

#[derive(Debug)]
enum Operation {
    Add(Value, Value),
    Multiply(Value, Value),
}

impl Operation {
    pub fn parse(s: &str) -> Option<Self> {
        let mut parts = s.split_whitespace();

        let first_value = parts.next()?;
        let first_value = match first_value {
            "old" => Value::Old,
            _ => Value::Value(first_value.parse::<u64>().ok()?),
        };

        let operation = parts.next()?;

        let second_value = parts.next()?;
        let second_value = match second_value {
            "old" => Value::Old,
            _ => Value::Value(second_value.parse::<u64>().ok()?),
        };

        match operation {
            "+" => Some(Operation::Add(first_value, second_value)),
            "*" => Some(Operation::Multiply(first_value, second_value)),
            _ => panic!("Unknown operation: {}", operation),
        }
    }

    pub fn apply(&self, old: u64) -> u64 {
        match self {
            Operation::Add(Value::Value(a), Value::Value(b)) => a + b,
            Operation::Add(Value::Value(a), Value::Old) => a + old,
            Operation::Add(Value::Old, Value::Value(b)) => old + b,
            Operation::Add(Value::Old, Value::Old) => old + old,
            Operation::Multiply(Value::Value(a), Value::Value(b)) => a * b,
            Operation::Multiply(Value::Value(a), Value::Old) => a * old,
            Operation::Multiply(Value::Old, Value::Value(b)) => old * b,
            Operation::Multiply(Value::Old, Value::Old) => old * old,
        }
    }
}

#[derive(Debug)]
struct Divisible(u64);

impl Divisible {
    pub fn parse(s: &str) -> Self {
        match s.trim().parse::<u64>() {
            Ok(value) => Divisible(value),
            _ => panic!("Unknown divisible amount: {}", s),
        }
    }

    pub fn is_divisible_by(&self, value: u64) -> bool {
        (value % self.0 as u64) == 0
    }
}

fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", part_one(input));
    println!("Part 2: {}", part_two(input));
}

fn get_monkey_business(monkey_inspection_count: & HashMap<u8, u64>) -> u64 {
    let mut highest_count = 0;
    let mut second_highest_count = 0;

    for (_, count) in monkey_inspection_count {
        if *count > highest_count {
            second_highest_count = highest_count;
            highest_count = *count;
        } else if *count > second_highest_count {
            second_highest_count = *count;
        }
    }

    highest_count * second_highest_count
}

fn find_lcm(a: u64, b: u64) -> u64 {
    let mut lcm = match a > b {
        true => a,
        false => b,
    };

    loop {
        if ((lcm % a) == 0) && ((lcm % b) == 0) {
            return lcm;
        }
        lcm += 1;
    }
}

fn part_one(input: &str) -> u64 {
    let monkeys = input.split("\n\n").flat_map(Monkey::try_parse).collect::<Vec<Monkey>>();

    let mut monkey_inspection_count = HashMap::new();
    for monkey in &monkeys {
        monkey_inspection_count.insert(monkey.number, 0);
    }

    let mut monkey_items = monkeys.iter().map(|m| m.items.clone()).collect::<Vec<Vec<u64>>>();

    for _ in 0..20 {
        for Monkey { number, operation, test, true_condition_monkey, false_condition_monkey, .. } in &monkeys {
            let current_monkey_items = monkey_items[*number as usize].clone();
            for worry_level in current_monkey_items {
                if let Some(count) = monkey_inspection_count.get_mut(&number) {
                    *count += 1;
                }

                let worry_level = operation.apply(worry_level);
                let worry_level = worry_level / 3;

                if test.is_divisible_by(worry_level) {
                    monkey_items[*true_condition_monkey as usize].push(worry_level);
                } else {
                    monkey_items[*false_condition_monkey as usize].push(worry_level);
                }
            }

            // Since this monkey's thrown all their items, they no longer have anything
            monkey_items[*number as usize].clear();
        }
    }

    get_monkey_business(&monkey_inspection_count)
}

fn part_two(input: &str) -> u64 {
    let monkeys = input.split("\n\n").flat_map(Monkey::try_parse).collect::<Vec<Monkey>>();

    let lcm_among_monkey_tests = monkeys
        .iter()
        .fold(1, |acc, monkey| find_lcm(acc, monkey.test.0));

    let mut monkey_inspection_count = HashMap::new();
    for monkey in &monkeys {
        monkey_inspection_count.insert(monkey.number, 0);
    }

    let mut monkey_items = monkeys.iter().map(|m| m.items.clone()).collect::<Vec<Vec<u64>>>();

    for _ in 0..10000 {
        for Monkey { number, operation, test, true_condition_monkey, false_condition_monkey, .. } in &monkeys {
            let current_monkey_items = monkey_items[*number as usize].clone();
            for worry_level in current_monkey_items {
                if let Some(count) = monkey_inspection_count.get_mut(&number) {
                    *count += 1;
                }

                let worry_level = operation.apply(worry_level);

                // Use the LCM found from the Monkey tests to keep the worry levels manageable
                // (i.e. not so large that they cause an arithmetic overflow). Using modular
                // arithmetic, we can keep the important info needed for our calculations.
                let worry_level = worry_level % lcm_among_monkey_tests;

                if test.is_divisible_by(worry_level) {
                    monkey_items[*true_condition_monkey as usize].push(worry_level);
                } else {
                    monkey_items[*false_condition_monkey as usize].push(worry_level);
                }
            }

            // Since this monkey's thrown all their items, they no longer have anything
            monkey_items[*number as usize].clear();
        }
    }

    get_monkey_business(&monkey_inspection_count)
}
