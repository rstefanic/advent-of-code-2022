fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", part_one(input));
    println!("Part 2: {}", part_two(input));
}

fn get_forest(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .flat_map(|c| c.to_digit(10))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn is_visible(forest: &Vec<Vec<u32>>, i: usize, j: usize) -> bool {
    let current_tree_height = forest[i][j];

    let mut top_visible = true;
    let mut bottom_visible = true;
    for k in 0..forest.len() {
        if k == i {
            // If we reach this point and the top's visible so far,
            // then we know it's visible from at least one direction.
            if top_visible {
                return true;
            }
            continue;
        }

        let tree_height = forest[k][j];

        if tree_height >= current_tree_height {
            if k < i {
                top_visible = false;
            } else if k > i {
                bottom_visible = false;
            }
        }
    }

    if top_visible || bottom_visible {
        return true;
    }

    let mut left_visible = true;
    let mut right_visible = true;
    for k in 0..forest[i].len() {
        if k == j {
            // If we reach this point and the left's visible so far,
            // then we know it's visible from at least one direction.
            if left_visible {
                return true;
            }
            continue;
        }

        let tree_height = forest[i][k];

        if tree_height >= current_tree_height {
            if k < j {
                left_visible = false;
            } else if k > j {
                right_visible = false;
            }
        }
    }

    left_visible || right_visible
}

fn get_scenic_score(forest: &Vec<Vec<u32>>, i: usize, j: usize) -> u32 {
    let current_tree_height = forest[i][j];

    // Scenic score looking up
    let mut top_score = 0;
    for k in (0..i).rev() {
        top_score += 1;

        let tree_height = forest[k][j];
        if tree_height >= current_tree_height {
            break;
        }
    }

    // Scenic score looking down
    let mut bottom_score = 0;
    for k in (i + 1)..forest.len() {
        bottom_score += 1;

        let tree_height = forest[k][j];
        if tree_height >= current_tree_height {
            break;
        }
    }

    // Scenic Score looking left
    let mut left_score = 0;
    for k in (0..j).rev() {
        left_score += 1;

        let tree_height = forest[i][k];
        if tree_height >= current_tree_height {
            break;
        }
    }

    // Scenic Score looking right
    let mut right_score = 0;
    for k in (j + 1)..forest[i].len() {
        right_score += 1;

        let tree_height = forest[i][k];
        if tree_height >= current_tree_height {
            break;
        }
    }

    top_score * bottom_score * left_score * right_score
}

fn part_one(input: &str) -> u32 {
    let forest = get_forest(input);
    let mut visible: u32 = 0;

    for i in 0..forest.len() {
        if i == 0 || i == (forest.len() - 1) {
            visible += forest.len() as u32;
            continue;
        }

        let row = &forest[i];
        for j in 0..row.len() {
            if j == 0 || j == (row.len() - 1) {
                visible += 1;
                continue;
            }

            visible += match is_visible(&forest, i, j) {
                true => 1,
                false => 0,
            }
        }
    }

    visible
}

fn part_two(input: &str) -> u32 {
    let forest = get_forest(input);
    let mut scenic_score: u32 = 0;

    for i in 0..forest.len() {
        if i == 0 || i == (forest.len() - 1) {
            continue;
        }

        let row = &forest[i];
        for j in 0..row.len() {
            if j == 0 || j == (row.len() - 1) {
                continue;
            }

            let current_scenic_score = get_scenic_score(&forest, i, j);
            if current_scenic_score > scenic_score {
                scenic_score = current_scenic_score;
            }
        }
    }

    scenic_score
}
