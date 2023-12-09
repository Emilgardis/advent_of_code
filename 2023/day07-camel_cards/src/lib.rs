//! Advent of code 2023 7
//!
//! Link: <https://adventofcode.com/2023/day/7>
//!
//! Good luck!
//!
//! ## Notes
//!
//! *

use std::collections::BTreeMap;

use aoc::{parts::*, Solver};
use eyre::Report;
use itertools::Itertools;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Hand {
    type_: Type,
    hand: [Suit; 5],
    bid: u64,
}

impl std::fmt::Debug for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Hand")
            .field("hand", &self.hand.iter().join(""))
            .field("type", &self.type_)
            .field("bid", &self.bid)
            .finish()
    }
}
impl Hand {
    fn new_p1(hand: &str, bid: &str) -> Result<Self, Report> {
        assert_eq!(hand.len(), 5);
        let hand = std::array::from_fn(|i| hand[i..i + 1].parse().unwrap());
        Ok(Self {
            type_: Type::identify_p1(&hand),
            hand,
            bid: bid.parse()?,
        })
    }

    fn new_p2(hand: &str, bid: &str) -> Result<Self, Report> {
        assert_eq!(hand.len(), 5);
        let hand = std::array::from_fn(|i| hand[i..i + 1].parse().unwrap());
        Ok(Self {
            type_: Type::identify_p2(&hand),
            hand,
            bid: bid.parse()?,
        })
    }
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Type {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Type {
    fn _identify(map: BTreeMap<&Suit, i32>) -> Self {
        let total = map.into_iter().collect_vec();
        if total.len() == 1 {
            Type::FiveOfAKind
        } else if total.len() == 2 {
            if total[0].1 == 4 || total[1].1 == 4 {
                Type::FourOfAKind
            } else {
                Type::FullHouse
            }
        } else if total.len() == 3 {
            if total.iter().any(|(_, ref c)| *c == 3) {
                Type::ThreeOfAKind
            } else {
                Type::TwoPair
            }
        } else if total.len() == 4 {
            Type::OnePair
        } else {
            Type::HighCard
        }
    }

    fn identify_p1(hand: &[Suit; 5]) -> Self {
        let mut map = BTreeMap::new();

        for suit in hand {
            map.entry(suit).and_modify(|i| *i += 1).or_insert(1);
        }
        Type::_identify(map)
    }

    fn identify_p2(hand: &[Suit; 5]) -> Self {
        let mut map = BTreeMap::new();

        for suit in hand {
            map.entry(suit).and_modify(|i| *i += 1).or_insert(1);
        }

        let Some(jokers) = map.remove(&Suit::J) else {
            return Type::_identify(map);
        };
        assert!(jokers >= 1);

        //
        match jokers {
            j @ 1..=4 => {
                *map.values_mut().max().unwrap() += j;
            }
            5 => {
                map.insert(&Suit::J, 5);
            }
            _ => unreachable!(),
        }

        Type::_identify(map)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord, Copy)]
#[repr(u8)]
pub enum Suit {
    Two = 2,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    J,
    Q,
    K,
    A,
}

impl Suit {
    fn cmp_part2(&self) -> u8 {
        if self != &Suit::J {
            (*self) as u8
        } else {
            0
        }
    }
}

impl std::fmt::Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Suit::Two => f.write_str("2"),
            Suit::Three => f.write_str("3"),
            Suit::Four => f.write_str("4"),
            Suit::Five => f.write_str("5"),
            Suit::Six => f.write_str("6"),
            Suit::Seven => f.write_str("7"),
            Suit::Eight => f.write_str("8"),
            Suit::Nine => f.write_str("9"),
            Suit::T => f.write_str("T"),
            Suit::J => f.write_str("J"),
            Suit::Q => f.write_str("Q"),
            Suit::K => f.write_str("K"),
            Suit::A => f.write_str("A"),
        }
    }
}

impl std::str::FromStr for Suit {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.as_bytes()[0] {
            b'T' => Suit::T,
            b'J' => Suit::J,
            b'Q' => Suit::Q,
            b'K' => Suit::K,
            b'A' => Suit::A,
            b @ b'2'..=b'9' => {
                let i = b - b'0';
                assert!(matches!(i, 2..=9));
                // XXX: :)
                unsafe { std::mem::transmute(i) }
            }
            _ => unreachable!(),
        })
    }
}

#[test]
fn suit_strength() {
    assert!(Suit::A > Suit::K);
}

impl Solver<Year2023, Day7, Part1> for Solution {
    type Input<'a> = Vec<Hand>;

    type Output = u64;

    fn generate_input(input: &'_ str) -> Result<Self::Input<'_>, Report> {
        let mut input: Vec<_> = input
            .lines()
            .map(|s| {
                let Some((hand, bid)) = s.split_once(' ') else {
                    return Err(eyre::eyre!("invalid hand {s}"));
                };
                Hand::new_p1(hand, bid)
            })
            .try_collect()?;
        input.sort();
        Ok(input)
    }

    fn solve(input: &Self::Input<'_>) -> Result<Self::Output, Report> {
        let mut sum = 0;
        for (i, hand) in input.iter().enumerate() {
            if hand.hand.contains(&Suit::J) {
                println!("hand: {:?}", hand);
            }
            sum += hand.bid * (i + 1) as u64;
        }
        Ok(sum)
    }
}

impl Solver<Year2023, Day7, Part2> for Solution {
    type Input<'a> = Vec<Hand>;

    type Output = <Self as Solver<2023, 7, Part1>>::Output;

    fn generate_input(input: &'_ str) -> Result<Self::Input<'_>, Report> {
        let mut input: Vec<_> = input
            .lines()
            .map(|s| {
                let Some((hand, bid)) = s.split_once(' ') else {
                    return Err(eyre::eyre!("invalid hand {s}"));
                };
                Hand::new_p2(hand, bid)
            })
            .try_collect()?;
        input.sort_by(|a, b| {
            match a.type_.cmp(&b.type_) {
                std::cmp::Ordering::Equal => {}
                ord => return ord,
            }

            let a_hand = a.hand.map(|s| s.cmp_part2());
            let b_hand = b.hand.map(|s| s.cmp_part2());
            match a_hand.cmp(&b_hand) {
                std::cmp::Ordering::Equal => {}
                ord => return ord,
            }
            a.bid.cmp(&b.bid)
        });

        Ok(input)
    }

    fn solve(input: &Self::Input<'_>) -> Result<Self::Output, Report> {
        <Self as Solver<Year2023, Day7, Part1>>::solve(input)
    }
}

pub struct Solution {}

impl Solution {}

#[test]
fn test_solution() -> Result<(), Report> {
    aoc::test_util::init();
    let input = r#"
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
    "#
    .trim();
    assert_eq!(
        aoc::solve_with_input::<Solution, Year2023, Day7, Part1>(input)?,
        6440
    );
    Ok(())
}

#[test]
fn test_solution_second() -> Result<(), Report> {
    aoc::test_util::init();
    let input = r#"
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
    "#
    .trim();
    assert_eq!(
        aoc::solve_with_input::<Solution, Year2023, Day7, Part2>(input)?,
        5905
    );
    Ok(())
}

#[test]
#[ignore]
fn solve_solution() -> Result<(), Report> {
    aoc::test_util::init();
    aoc::Aoc::solve::<Solution, Year2023, Day7, Part1>()
        .map(|s| println!(":: ⭐Solution found⭐ ::\n{s}"))
}

#[test]
#[ignore]
fn solve_solution_second() -> Result<(), Report> {
    aoc::test_util::init();
    aoc::Aoc::solve::<Solution, Year2023, Day7, Part2>()
        .map(|s| println!(":: ⭐Solution found⭐ ::\n{s}"))
}
