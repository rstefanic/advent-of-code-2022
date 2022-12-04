#[derive(Clone, Debug, PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissor,
}

impl Shape {
    fn defeats(&self) -> Shape {
        match self {
            Shape::Rock => Shape::Scissor,
            Shape::Paper => Shape::Rock,
            Shape::Scissor => Shape:: Paper
        }
    }

    fn defeated_by(&self) -> Shape {
        match self {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissor,
            Shape::Scissor => Shape:: Rock
        }
    }
}

#[derive(Debug)]
enum RoundResult {
    Opponent,
    Me,
    Draw,
}

#[derive(Debug)]
struct Round {
    opponent_choice: Shape,
    my_choice: Shape,
}

impl Round {
    fn result(&self) -> RoundResult {
        if self.opponent_choice == self.my_choice {
            RoundResult::Draw
        } else if self.opponent_choice.defeats() == self.my_choice {
            RoundResult::Opponent
        } else {
            RoundResult::Me
        }
    }

    fn score(&self) -> u32 {
        let weapon_choice_points =  match self.my_choice {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissor => 3,
        };

        weapon_choice_points + match self.result() {
            RoundResult::Opponent => 0,
            RoundResult::Draw => 3,
            RoundResult::Me => 6,
        }
    }
}

fn main() {
    let input = include_str!("./input.txt");
    println!("Part 1: {}", part_one(input));
    println!("Part 2: {}", part_two(input));
}

fn part_one(input: &str) -> u32 {
    input.lines().map(|round| {
        let results: Vec<&str> = round.split(" ").collect();
        let opponent_choice = match results[0] {
            "A" => Shape::Rock,
            "B" => Shape::Paper,
            "C" => Shape::Scissor,
            x => panic!("Unknown opponent weapon choice: {}", x),
        };

        let my_choice = match results[1] {
            "X" => Shape::Rock,
            "Y" => Shape::Paper,
            "Z" => Shape::Scissor,
            x => panic!("Unknown my weapon choice: {}", x),
        };

        Round { opponent_choice, my_choice }.score()
    })
    .sum()
}

fn part_two(input: &str) -> u32 {
    input.lines().map(|round| {
        let results: Vec<&str> = round.split(" ").collect();
        let opponent_choice = match results[0] {
            "A" => Shape::Rock,
            "B" => Shape::Paper,
            "C" => Shape::Scissor,
            x => panic!("Unknown opponent weapon choice: {}", x),
        };

        let choice_to_make = match results[1] {
            "X" => opponent_choice.defeats(),
            "Y" => opponent_choice.clone(),
            "Z" => opponent_choice.defeated_by(),
            x => panic!("Unknown strategy: {}", x),
        };

        Round { opponent_choice, my_choice: choice_to_make }.score()
    })
    .sum()
}
