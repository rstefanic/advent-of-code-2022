use std::collections::HashSet;

const SAND_FALL_START: (u32, u32) = (500, 0);

fn main() {
    let input = include_str!("./input.txt");

    println!("Part 1: {}", part_one(input));
    println!("Part 2: {}", part_two(input));
}

fn get_coordinates(input: &str) -> impl Iterator<Item = (u32, u32)> + '_ {
    input.split("\n").flat_map(|line| {
        line.split(" -> ")
            // Parse each coordinate on this line into tuples of (x, y) points
            .flat_map(|coord| match coord.split_once(",") {
                Some((x, y)) => Some((x.parse::<u32>().unwrap(), y.parse::<u32>().unwrap())),
                None => None,
            })
            .collect::<Vec<(u32, u32)>>()
            .windows(2)
            // Find the coordinates that make up the line between each point
            .flat_map(|line| {
                let mut local_line = Vec::new();
                if let [a, b] = line {
                    if a.0 != b.0 {
                        // They differ on the x-axis which means the line runs along the x-axis.
                        // Find the all the coordinates between them that make up the line.
                        let x_max = std::cmp::max(a.0, b.0);
                        let x_min = std::cmp::min(a.0, b.0);

                        for x in x_min..x_max {
                            local_line.push((x, a.1));
                        }
                    } else {
                        // Same thing, but for the y-axis
                        let y_max = std::cmp::max(a.1, b.1);
                        let y_min = std::cmp::min(a.1, b.1);

                        for y in y_min..y_max {
                            local_line.push((a.0, y));
                        }
                    }

                    local_line.push(*a);
                    local_line.push(*b);
                }

                local_line
            })
            .collect::<Vec<(u32, u32)>>()
    })
}

fn part_one(input: &str) -> u32 {
    let mut spaces_occupied: HashSet<(u32, u32)> = HashSet::new();
    let mut abyss = 0;

    for (x, y) in get_coordinates(input) {
        spaces_occupied.insert((x, y));
        abyss = std::cmp::max(y, abyss);
    }

    let mut sand_fallen = 0;
    let mut current_sand = SAND_FALL_START;

    loop {
        let (x, y) = current_sand;
        if y >= abyss {
            break;
        }

        if let None = spaces_occupied.get(&(x, y + 1)) {
            current_sand = (x, y + 1);
            continue;
        } else if let None = spaces_occupied.get(&(x - 1, y + 1)) {
            current_sand = (x - 1, y + 1);
            continue;
        } else if let None = spaces_occupied.get(&(x + 1, y + 1)) {
            current_sand = (x + 1, y + 1);
            continue;
        }

        spaces_occupied.insert(current_sand);
        sand_fallen += 1;
        current_sand = SAND_FALL_START;
    }

    sand_fallen
}

fn part_two(input: &str) -> u32 {
    let mut spaces_occupied: HashSet<(u32, u32)> = HashSet::new();
    let mut bottom = 0;
    let mut sand_fallen = 0;

    for (x, y) in get_coordinates(input) {
        spaces_occupied.insert((x, y));
        bottom = std::cmp::max(y, bottom);
    }

    // The floor is bottom + 2; however those spaces are occupied by rock.
    // So the next available space would be bottom + 1 (i.e. floor - 1).
    let bottom = bottom + 1;

    loop {
        let (mut x, mut y) = SAND_FALL_START;

        if let Some(_) = spaces_occupied.get(&(x, y)) {
            break;
        }

        loop {
            if y == bottom {
                spaces_occupied.insert((x, y));
                sand_fallen += 1;
                break;
            }

            if let None = spaces_occupied.get(&(x, y + 1)) {
                (x, y) = (x, y + 1);
                continue;
            } else if let None = spaces_occupied.get(&(x - 1, y + 1)) {
                (x, y) = (x - 1, y + 1);
                continue;
            } else if let None = spaces_occupied.get(&(x + 1, y + 1)) {
                (x, y) = (x + 1, y + 1);
                continue;
            }

            spaces_occupied.insert((x, y));
            sand_fallen += 1;
            break;
        }
    }

    sand_fallen
}
