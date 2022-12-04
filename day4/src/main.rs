use std::ops::Range;

pub trait RangeComparison<T> {
    fn contains_range(&self, other: &Self) -> bool;
    fn overlaps_with(&self, other: &Self) -> bool;
}

impl<T: PartialOrd> RangeComparison<T> for Range<T> {
    fn contains_range(&self, other: &Range<T>) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn overlaps_with(&self, other: &Range<T>) -> bool {
        self.start <= other.end && self.end >= other.end
            || self.end <= other.start && self.end >= other.start
    }
}

fn main() {
    let input = include_str!("./input.txt");
    println!("Part 1: {}", part_one(input));
    println!("Part 2: {}", part_two(input));
}

fn get_assignment_pairs(pair: &str) -> Option<(&str, &str)> {
    let mut pair = pair.split(",");
    Some((pair.next()?, pair.next()?))
}

fn get_assignment_range(assignment: &str) -> Option<Range<u32>> {
    let mut assignment = assignment.split("-");
    let start = match assignment.next()?.parse::<u32>() {
        Ok(n) => n,
        _ => return None,
    };
    let end = match assignment.next()?.parse::<u32>() {
        Ok(n) => n,
        _ => return None,
    };
    Some(Range { start, end })
}

fn part_one(input: &str) -> u32 {
    input
        .lines()
        .flat_map(|pair| {
            let (assignment_one, assignment_two) = get_assignment_pairs(pair)?;
            let assignment_one_range = get_assignment_range(assignment_one)?;
            let assignment_two_range = get_assignment_range(assignment_two)?;
            if assignment_one_range.contains_range(&assignment_two_range)
                || assignment_two_range.contains_range(&assignment_one_range)
            {
                return Some(1);
            }
            None
        })
        .sum()
}

fn part_two(input: &str) -> u32 {
    input
        .lines()
        .flat_map(|pair| {
            let (assignment_one, assignment_two) = get_assignment_pairs(pair)?;
            let assignment_one_range = get_assignment_range(assignment_one)?;
            let assignment_two_range = get_assignment_range(assignment_two)?;
            if assignment_one_range.overlaps_with(&assignment_two_range)
                || assignment_two_range.overlaps_with(&assignment_one_range)
            {
                return Some(1);
            }
            None
        })
        .sum()
}
