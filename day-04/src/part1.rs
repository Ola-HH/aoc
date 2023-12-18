use nom::AsBytes;
use rayon::prelude::*;
use std::{cmp, str::from_utf8};

use crate::custom_error::AocError;

struct Card {
    id: u32,
    winning_numbers: Vec<u32>,
    my_numbers: Vec<u32>,
}
impl Card {
    fn new(line: &str) -> Self {
        let mut it = line.split(":");
        let id = it
            .next()
            .expect("must exist")
            .split(" ")
            .last()
            .expect("should be a number")
            .parse::<u32>()
            .expect("Unparseable");

        let mut game_it = it.last().expect("must exist").split("|");

        let winners: Vec<u32> = game_it
            .next()
            .unwrap()
            .replace("  ", " 0")
            .trim()
            .split(" ")
            .map(|num| {
                dbg!(num);
                num.trim().parse::<u32>().unwrap()
            })
            .collect();

        let numbers: Vec<u32> = game_it
            .last()
            .unwrap()
            .replace("  ", " 0")
            .trim()
            .split(" ")
            .map(|num| {
                dbg!(num);
                num.trim().parse::<u32>().unwrap()
            })
            .collect();

        Card {
            id,
            winning_numbers: winners,
            my_numbers: numbers,
        }
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let _card: Vec<Card> = input.lines().map(|line| Card::new(line)).collect();
    // TODO: Logic
    Ok(input.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!("13", process(input)?);
        Ok(())
    }
}