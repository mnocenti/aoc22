use std::str::FromStr;

use std::cmp::Ordering;

use itertools::Itertools;

aoc22::main!(day13, "../inputs/input13.txt");

aoc22::test_with_example!(day13, "../inputs/example13.txt", 13, 140);

pub fn day13(input: &str) -> aoc22::MyResult<(usize, usize)> {
    let mut distress_signal = parse_packets(input)?;

    let part1 = distress_signal
        .iter()
        .tuples()
        .map(|(left, right)| left < right)
        .enumerate()
        .filter_map(|(index, is_ordered)| is_ordered.then_some(index + 1))
        .sum();

    let first_divider = Packet::from_str("[[2]]")?;
    let second_divider = Packet::from_str("[[6]]")?;
    distress_signal.push(first_divider.clone());
    distress_signal.push(second_divider.clone());
    distress_signal.sort();

    let divider_position =
        |divider| distress_signal.iter().position(|p| *p == divider).unwrap() + 1;

    let part2 = divider_position(first_divider) * divider_position(second_divider);

    Ok((part1, part2))
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Packet {
    List(Vec<Packet>),
    Int(u32),
}

impl Packet {
    /// Consumes the first part of the string until a packet has been parsed, and returns the rest of the string
    fn parse_and_consume(mut s: &str) -> aoc22::MyResult<(Self, &str)> {
        if s.starts_with('[') {
            // list
            // consume opening bracket
            s = &s[1..];
            let mut list = Vec::new();
            while !s.starts_with(']') && !s.is_empty() {
                // parse each subpacket
                let (subpacket, consumed_s) = Self::parse_and_consume(s)?;
                list.push(subpacket);
                // consume the comma if any
                s = consumed_s.trim_start_matches(',');
            }
            // consume closing bracket
            if s.starts_with(']') {
                s = &s[1..];
            }
            Ok((Self::List(list), s))
        } else {
            // integer
            let end_index = s.find([',', ']']).unwrap_or(s.len() - 1);
            Ok((Self::Int(s[..end_index].parse()?), &s[end_index..]))
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        use Packet::*;
        match (self, other) {
            (Int(l), Int(r)) => l.cmp(r),
            (Int(l), List(_)) => List(vec![Int(*l)]).cmp(other),
            (List(_), Int(r)) => self.cmp(&List(vec![Int(*r)])),
            (List(l), List(r)) => l.cmp(r),
        }
    }
}

impl FromStr for Packet {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Packet::parse_and_consume(s)?.0)
    }
}

fn parse_packets(input: &str) -> aoc22::MyResult<Vec<Packet>> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(Packet::from_str)
        .collect()
}
