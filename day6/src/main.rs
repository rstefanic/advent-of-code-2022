fn main() {
    let input = include_str!("input.txt");
    println!("{}", part_one(input));
    println!("{}", part_two(input));
}

fn has_duplicate_chars(chars: &[char]) -> bool {
    for i in 0..chars.len() {
        for j in (i + 1)..chars.len() {
            if chars[i] == chars[j] {
                return true;
            }
        }
    }

    false
}

fn find_marker_start(window_size: usize) -> Box<dyn Fn(&str) -> usize> {
    Box::new(move |datastream| {
        let mut marker_start_position = window_size;
        let chars = datastream.chars().collect::<Vec<char>>();
        let mut window = chars.windows(marker_start_position);

        while let Some(chars) = window.next() {
            if !has_duplicate_chars(chars) {
                return marker_start_position;
            }

            marker_start_position += 1;
        }

        marker_start_position
    })
}

fn part_one(input: &str) -> usize {
    input.lines().map(find_marker_start(4)).sum()
}

fn part_two(input: &str) -> usize {
    input.lines().map(find_marker_start(14)).sum()
}
