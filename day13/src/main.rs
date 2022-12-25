use std::{fmt::Error, iter::Peekable, str::FromStr};

#[derive(Debug, PartialEq, Eq)]
enum Packet {
    Value(u32),
    List(Vec<Packet>),
}

impl Packet {
    pub fn parse(line: &str) -> Result<Packet, Error> {
        let mut chars = line.chars().peekable();

        // The start should always be a list
        if let Some('[') = chars.peek() {
            return Self::parse_list(&mut chars);
        }

        Err(Error)
    }

    fn parse_list<I: Iterator<Item = char>>(chars: &mut Peekable<I>) -> Result<Packet, Error> {
        let mut list: Vec<Packet> = Vec::new();

        // Consume the first '['
        if let Some('[') = chars.peek() {
            chars.next();
        }

        while let Some(c) = chars.peek() {
            match c {
                '[' => list.push(Self::parse_list(chars)?),
                ']' => {
                    chars.next();
                    break;
                }
                ',' => {
                    chars.next();
                    continue;
                }
                c => {
                    if c.is_numeric() {
                        list.push(Self::parse_number(chars)?)
                    }
                }
            }
        }

        Ok(Self::List(list))
    }

    fn parse_number<I: Iterator<Item = char>>(chars: &mut Peekable<I>) -> Result<Packet, Error> {
        let mut number_string = String::new();

        // We can safely take the first digit without checking becuase the
        // calling function verified that this is a numeric character.
        number_string.push(chars.next().unwrap());
        while let Some(c) = chars.peek() {
            if c.is_numeric() {
                number_string.push(chars.next().unwrap());
            } else {
                break;
            }
        }

        return match number_string.parse::<u32>() {
            Ok(n) => Ok(Self::Value(n)),
            Err(_) => Err(Error),
        };
    }

    fn promote_to_list(&self) -> Option<Packet> {
        match self {
            Self::Value(n) => Some(Self::List(vec![Self::Value(*n)])),
            _ => None,
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self {
            Self::Value(n) => match other {
                Self::Value(other_n) => n.partial_cmp(other_n),
                Self::List(other_list) => {
                    // If the other is a List, then we need to promote self to a List before comparing
                    if let Some(Self::List(new_list)) = self.promote_to_list() {
                        return new_list.partial_cmp(&other_list);
                    }

                    return None;
                }
            },
            Self::List(list) => match other {
                // If the other is a Value, then we need to promote other to a List before comparing
                Self::Value(_) => {
                    if let Some(Self::List(other_list)) = other.promote_to_list() {
                        return list.partial_cmp(&other_list);
                    }

                    return None;
                }
                Self::List(other_list) => list.partial_cmp(other_list),
            },
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self == other {
            return std::cmp::Ordering::Equal;
        }

        match self.partial_cmp(other) {
            Some(order) => order,
            None => panic!("Could not determine the order between two packets"),
        }
    }
}

fn main() {
    let input = include_str!("./input.txt");

    println!("Part 1: {}", part_one(input));
    println!("Part 2: {}", part_two(input));
}

fn part_one(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|packets| {
            packets
                .split("\n")
                .flat_map(|packet| Packet::parse(packet))
                .collect::<Vec<_>>()
        })
        .map(|packets| packets[0].partial_cmp(&packets[1]))
        .enumerate()
        .map(|(i, order)| {
            match order {
                // Add 1 here since the input index starts at 1
                Some(std::cmp::Ordering::Less) => i + 1,
                _ => 0,
            }
        })
        .sum()
}

fn part_two(input: &str) -> usize {
    let first_divider_packet = Packet::List(vec![Packet::List(vec![Packet::Value(2)])]);
    let second_divider_packet = Packet::List(vec![Packet::List(vec![Packet::Value(6)])]);

    // Add the divider packets to the input before parsing
    let input = match String::from_str(input) {
        Ok(mut s) => {
            s.push_str("[[2]]\n[[6]]");
            s
        }
        Err(_) => panic!("Could not add divider packets"),
    };

    let mut packets = input
        .split("\n")
        .flat_map(|packet| Packet::parse(packet))
        .collect::<Vec<_>>();

    // Sort the packets and transform it back into an iterator
    packets.sort();
    let mut sorted_packets = packets.iter();

    // Find the index of the divider packets and add 1 since the packets are indexed starting at 1
    let first_divider_packet_index = sorted_packets
        .position(|packet| *packet == first_divider_packet)
        .unwrap()
        + 1;
    let second_divider_packet_index = sorted_packets
        .position(|packet| *packet == second_divider_packet)
        .unwrap()
        + 1;

    // Since the call to 'position' modifies the underlying Iterator by returning the remaining items
    // from where it found the element, the second call to 'position' on the Iterator will start from
    // where the the previous item was found. So we need to add both indexes to get the second index.
    first_divider_packet_index * (first_divider_packet_index + second_divider_packet_index)
}

#[test]
fn compare_two_packets_correctly() {
    let packet_a = Packet::parse("[[1],[2,3,4]]").unwrap();
    let packet_b = Packet::parse("[[1],4]").unwrap();
    assert_eq!(
        packet_a.partial_cmp(&packet_b),
        Some(std::cmp::Ordering::Less)
    )
}
