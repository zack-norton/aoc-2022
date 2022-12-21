use crate::days::Problem;

use itertools::Itertools;
use serde_json::Value;
use std::cmp::Ordering;

pub struct Day13 {}

impl Problem for Day13 {
    fn part_one(&self, input: &str) -> String {
        let signals = input
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| {
                let packet_json = serde_json::from_str::<Value>(line).unwrap();
                Packet::try_from(packet_json).unwrap()
            })
            .collect::<Vec<_>>();

        let pair_sums: u32 = signals
            .iter()
            .tuples()
            .positions(|(a, b)| a.cmp(b) != Ordering::Greater)
            .map(|i| i as u32 + 1)
            .sum();

        pair_sums.to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let mut signals = input
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| {
                let packet_json = serde_json::from_str::<Value>(line).unwrap();
                Packet::try_from(packet_json).unwrap()
            })
            .collect::<Vec<_>>();

        let dividers = [
            Packet::try_from(serde_json::from_str::<Value>("[[2]]").unwrap()).unwrap(),
            Packet::try_from(serde_json::from_str::<Value>("[[6]]").unwrap()).unwrap(),
        ];

        signals.extend(dividers.iter().cloned());
        signals.sort();
        let key: u32 = signals
            .iter()
            .positions(|b| dividers.contains(b))
            .map(|i| i as u32 + 1)
            .product();

        key.to_string()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Packet {
    Integer(u32),
    List(Vec<Packet>),
}

impl TryFrom<Value> for Packet {
    type Error = String;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Number(num) => Ok(Self::Integer(num.as_u64().unwrap() as u32)),
            Value::Array(arr) => Ok(Self::List(
                arr.iter()
                    .map(|v| Packet::try_from(v.clone()).unwrap())
                    .collect(),
            )),
            _ => Err("Invalid Packet".to_owned()),
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Integer(left_packet_num), Self::Integer(right_packet_num)) => {
                left_packet_num.cmp(right_packet_num)
            }
            (Self::Integer(_), Self::List(_)) => Self::List(vec![self.clone()]).cmp(other),
            (Self::List(_), Self::Integer(_)) => self.cmp(&Self::List(vec![other.clone()])),
            (Self::List(left_packet_list), Self::List(right_packet_list)) => {
                for (left_packet_num, right_packet_num) in
                    left_packet_list.iter().zip(right_packet_list)
                {
                    if left_packet_num.cmp(right_packet_num) != Ordering::Equal {
                        return left_packet_num.cmp(right_packet_num);
                    }
                }
                left_packet_list.len().cmp(&right_packet_list.len())
            }
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    #[test]
    fn test_part_one() {
        assert_eq!(
            "13",
            Day13 {}.part_one(&util::input(13, util::InputType::Example))
        )
    }

    #[test]
    fn test_part_two() {
        assert_eq!(
            "140",
            Day13 {}.part_two(&util::input(13, util::InputType::Example))
        )
    }
}
