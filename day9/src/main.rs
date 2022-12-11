use std::collections::{HashMap, LinkedList};

#[derive(Debug)]
enum Movement {
    Up(u32),
    Down(u32),
    Left(u32),
    Right(u32),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

impl Movement {
    fn from_str(line: &str) -> Movement {
        let mut split = line.split_whitespace();

        if let Some(direction) = split.next() {
            if let Some(distance) = split.next() {
                if let Ok(distance) = distance.parse::<u32>() {
                    return match direction {
                        "U" => Movement::Up(distance),
                        "D" => Movement::Down(distance),
                        "L" => Movement::Left(distance),
                        "R" => Movement::Right(distance),
                        _ => panic!("Invalid direction"),
                    };
                }
            }
        }
        panic!("Invalid movement")
    }
}

fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", part_one(input));
    println!("Part 2: {}", part_two(input));
}

fn add_one_to_visited_tail_positions(
    visisted_tail_positions: &mut HashMap<Position, u32>,
    tail_position: &Position,
) {
    if let Some(position) = visisted_tail_positions.get_mut(tail_position) {
        *position += 1;
    } else {
        visisted_tail_positions.insert(*tail_position, 1);
    }
}

fn adjust_following_knot(following_knot: &mut Position, leading_knot: &Position) -> () {
    let is_more_than_one_above = (leading_knot.y - following_knot.y) > 1;
    let is_more_than_one_below = (leading_knot.y - following_knot.y) < -1;
    let is_more_than_one_right = (leading_knot.x - following_knot.x) > 1;
    let is_more_than_one_left = (leading_knot.x - following_knot.x) < -1;

    // For part one, dx and dy can never both be 2 at the same time,
    // but this is possible in part two.
    if is_more_than_one_above && is_more_than_one_right {
        following_knot.x = leading_knot.x - 1;
        following_knot.y = leading_knot.y - 1;
    } else if is_more_than_one_above && is_more_than_one_left {
        following_knot.x = leading_knot.x + 1;
        following_knot.y = leading_knot.y - 1;
    } else if is_more_than_one_below && is_more_than_one_right {
        following_knot.x = leading_knot.x - 1;
        following_knot.y = leading_knot.y + 1;
    } else if is_more_than_one_below && is_more_than_one_left {
        following_knot.x = leading_knot.x + 1;
        following_knot.y = leading_knot.y + 1;
    } else if is_more_than_one_above {
        following_knot.x = leading_knot.x;
        following_knot.y = leading_knot.y - 1;
    } else if is_more_than_one_below {
        following_knot.x = leading_knot.x;
        following_knot.y = leading_knot.y + 1;
    } else if is_more_than_one_left {
        following_knot.x = leading_knot.x + 1;
        following_knot.y = leading_knot.y;
    } else if is_more_than_one_right {
        following_knot.x = leading_knot.x - 1;
        following_knot.y = leading_knot.y;
    }
}

fn move_rope(
    move_head_knot: fn(&mut Position) -> (),
    rope: &mut LinkedList<Position>,
    visited_tail_positions: &mut HashMap<Position, u32>,
) -> () {
    let mut previous_knot = None;

    for (i, mut knot) in rope.iter_mut().enumerate() {
        if i == 0 {
            move_head_knot(knot);
            previous_knot = Some(knot);
            continue;
        }

        adjust_following_knot(&mut knot, &previous_knot.unwrap());
        previous_knot = Some(knot);
    }

    add_one_to_visited_tail_positions(visited_tail_positions, &rope.back().unwrap());
}

fn part_one(input: &str) -> usize {
    let movements = input.lines().map(|line| Movement::from_str(line));

    let mut head_position = Position { x: 0, y: 0 };
    let mut tail_position = Position { x: 0, y: 0 };
    let mut visited_tail_positions: HashMap<Position, u32> = HashMap::new();

    // Add the starting position
    visited_tail_positions.insert(tail_position, 1);

    for movement in movements {
        match movement {
            Movement::Up(distance) => {
                for _ in 0..distance {
                    head_position.y += 1;
                    adjust_following_knot(&mut tail_position, &head_position);
                    add_one_to_visited_tail_positions(&mut visited_tail_positions, &tail_position);
                }
            }
            Movement::Down(distance) => {
                for _ in 0..distance {
                    head_position.y -= 1;
                    adjust_following_knot(&mut tail_position, &head_position);
                    add_one_to_visited_tail_positions(&mut visited_tail_positions, &tail_position);
                }
            }
            Movement::Left(distance) => {
                for _ in 0..distance {
                    head_position.x -= 1;
                    adjust_following_knot(&mut tail_position, &head_position);
                    add_one_to_visited_tail_positions(&mut visited_tail_positions, &tail_position);
                }
            }
            Movement::Right(distance) => {
                for _ in 0..distance {
                    head_position.x += 1;
                    adjust_following_knot(&mut tail_position, &head_position);
                    add_one_to_visited_tail_positions(&mut visited_tail_positions, &tail_position);
                }
            }
        }
    }

    visited_tail_positions.len()
}

fn part_two(input: &str) -> usize {
    let movements = input.lines().map(|line| Movement::from_str(line));

    let mut rope = LinkedList::from([Position { x: 0, y: 0 }; 10]);
    let mut visited_tail_positions: HashMap<Position, u32> = HashMap::new();

    // Add the starting position
    if let Some(tail) = rope.back() {
        visited_tail_positions.insert(*tail, 1);
    }

    for movement in movements {
        match movement {
            Movement::Up(distance) => {
                for _ in 0..distance {
                    move_rope(
                        |mut head| head.y += 1,
                        &mut rope,
                        &mut visited_tail_positions,
                    );
                }
            }
            Movement::Down(distance) => {
                for _ in 0..distance {
                    move_rope(
                        |mut head| head.y -= 1,
                        &mut rope,
                        &mut visited_tail_positions,
                    );
                }
            }
            Movement::Left(distance) => {
                for _ in 0..distance {
                    move_rope(
                        |mut head| head.x -= 1,
                        &mut rope,
                        &mut visited_tail_positions,
                    );
                }
            }
            Movement::Right(distance) => {
                for _ in 0..distance {
                    move_rope(
                        |mut head| head.x += 1,
                        &mut rope,
                        &mut visited_tail_positions,
                    );
                }
            }
        }
    }

    visited_tail_positions.len()
}
