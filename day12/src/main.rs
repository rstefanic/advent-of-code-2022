use std::collections::VecDeque;

const START_MARKER: u32 = 'S' as u32;
const END_MARKER: u32 = 'E' as u32;
const HIGHEST_ELEVATION: u32 = 'z' as u32;
const LOWEST_ELEVATION: u32 = 'a' as u32;

type Point = (usize, usize);
struct Path {
    current: Point,
    steps: usize,
}

fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", part_one(input));
    println!("Part 1: {}", part_two(input));
}

fn build_elevation_map(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c as u32).collect::<Vec<u32>>())
        .collect::<Vec<_>>()
}

fn get_start_point(map: &Vec<Vec<u32>>) -> Point {
    for (y, _) in map.iter().enumerate() {
        for (x, _) in map[y].iter().enumerate() {
            if map[y][x] == START_MARKER {
                return (y, x);
            }
        }
    }

    panic!("No start point found");
}

fn get_end_point(map: &Vec<Vec<u32>>) -> Point {
    for (y, _) in map.iter().enumerate() {
        for (x, _) in map[y].iter().enumerate() {
            if map[y][x] == END_MARKER {
                return (y, x);
            }
        }
    }

    panic!("No end point found");
}

fn get_neighboring_points(point: Point, map: &Vec<Vec<u32>>) -> Vec<Point> {
    let mut neighbors = Vec::new();
    let (y, x) = point;

    // Up Neighbor
    if y > 0 {
        neighbors.push((y - 1, x));
    }

    // Down Neighbor
    if y < map.len() - 1 {
        neighbors.push((y + 1, x));
    }

    // Left Neighbor
    if x > 0 {
        neighbors.push((y, x - 1));
    }

    // Right Neighbor
    if x < (map[y].len() - 1) {
        neighbors.push((y, x + 1));
    }

    neighbors
}

fn part_one(input: &str) -> usize {
    let elevation_map = build_elevation_map(input);
    let start = get_start_point(&elevation_map);
    let end = get_end_point(&elevation_map);
    let mut visited: Vec<Point> = Vec::new();
    let mut queue: VecDeque<Path> = VecDeque::from(vec![Path {
        current: start,
        steps: 0,
    }]);

    while let Some(Path { current, steps }) = queue.pop_front() {
        if visited.contains(&current) {
            continue;
        }
        visited.push(current);

        let (y, x) = current;
        if y == end.0 && x == end.1 {
            return steps;
        }

        let mut current_elevation = elevation_map[y][x];
        // The START_MARKER is effecticely the lowest elevation
        if current_elevation == START_MARKER {
            current_elevation = LOWEST_ELEVATION;
        }

        let neighbors = get_neighboring_points((y, x), &elevation_map);
        for neighbor in neighbors {
            if visited.contains(&neighbor) {
                continue;
            }

            let neighbor_elevation = elevation_map[neighbor.0][neighbor.1];

            // neighbor_elevation could be 'E'. Since the 'E' elevation would be lower than any
            // lowercase letter, we need to ensure we're only stepping to 'E' if we're on 'z'.
            if neighbor_elevation == END_MARKER && current_elevation != HIGHEST_ELEVATION {
                continue;
            }

            if current_elevation >= (neighbor_elevation - 1) {
                queue.push_back(Path {
                    current: neighbor,
                    steps: steps + 1,
                });
            }
        }
    }

    panic!("No path found!");
}

fn part_two(input: &str) -> usize {
    let elevation_map = build_elevation_map(input);
    let end = get_end_point(&elevation_map);
    let mut visited: Vec<Point> = Vec::new();
    let mut queue: VecDeque<Path> = VecDeque::from(vec![Path {
        current: end,
        steps: 0,
    }]);

    while let Some(Path { current, steps }) = queue.pop_front() {
        if visited.contains(&current) {
            continue;
        }
        visited.push(current);

        let (y, x) = current;
        let mut current_elevation = elevation_map[y][x];
        if current_elevation == LOWEST_ELEVATION {
            return steps;
        }

        if current_elevation == END_MARKER {
            current_elevation = HIGHEST_ELEVATION;
        }

        let neighbors = get_neighboring_points((y, x), &elevation_map);
        for neighbor in neighbors {
            if visited.contains(&neighbor) {
                continue;
            }

            let neighbor_elevation = elevation_map[neighbor.0][neighbor.1];
            if neighbor_elevation == END_MARKER {
                continue;
            }

            if (current_elevation - 1) <= neighbor_elevation {
                queue.push_back(Path {
                    current: neighbor,
                    steps: steps + 1,
                });
            }
        }
    }

    panic!("No path found!");
}
