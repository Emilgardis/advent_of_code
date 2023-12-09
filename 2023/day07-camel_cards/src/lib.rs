//! Advent of code 2023 7
//!
//! Link: <https://adventofcode.com/2023/day/7>
//!
//! Good luck!
//!
//! ## Description
//! <!---STARTOFDESCRIPTION--->
//!Your all-expenses-paid trip turns out to be a one-way, five-minute ride in an [airship](https://en.wikipedia.org/wiki/Airship). (At least it's a *cool* airship!) It drops you off at the edge of a vast desert and descends back to Island Island.
//!
//! "Did you bring the parts?"
//!
//! You turn around to see an Elf completely covered in white clothing, wearing goggles, and riding a large [camel](https://en.wikipedia.org/wiki/Dromedary).
//!
//! "Did you bring the parts?" she asks again, louder this time. You aren't sure
//! what parts she's looking for; you're here to figure out why the sand
//! stopped.
//!
//! "The parts! For the sand, yes! Come with me; I will show you." She beckons
//! you onto the camel.
//!
//! After riding a bit across the sands of Desert Island, you can see what look
//! like very large rocks covering half of the horizon. The Elf explains that
//! the rocks are all along the part of Desert Island that is directly above
//! Island Island, making it hard to even get there. Normally, they use big
//! machines to move the rocks and filter the sand, but the machines have broken
//! down because Desert Island recently stopped receiving the *parts* they need
//! to fix the machines.
//!
//! You've already assumed it'll be your job to figure out why the parts stopped
//! when she asks if you can help. You agree automatically.
//!
//! Because the journey will take a few days, she offers to teach you the game of *Camel Cards*. Camel Cards is sort of similar to [poker](https://en.wikipedia.org/wiki/List_of_poker_hands) except it's designed to be easier to play while riding a camel.
//!
//! In Camel Cards, you get a list of *hands*, and your goal is to order them
//! based on the *strength* of each hand. A hand consists of *five cards*
//! labeled one of `A`, `K`, `Q`, `J`, `T`, `9`, `8`, `7`, `6`, `5`, `4`, `3`,
//! or `2`. The relative strength of each card follows this order, where `A` is
//! the highest and `2` is the lowest.
//!
//! Every hand is exactly one *type*. From strongest to weakest, they are:
//!
//! * *Five of a kind*, where all five cards have the same label: `AAAAA`
//! * *Four of a kind*, where four cards have the same label and one card has a
//!   different label: `AA8AA`
//! * *Full house*, where three cards have the same label, and the remaining two
//!   cards share a different label: `23332`
//! * *Three of a kind*, where three cards have the same label, and the
//!   remaining two cards are each different from any other card in the hand:
//!   `TTT98`
//! * *Two pair*, where two cards share one label, two other cards share a
//!   second label, and the remaining card has a third label: `23432`
//! * *One pair*, where two cards share one label, and the other three cards
//!   have a different label from the pair and each other: `A23A4`
//! * *High card*, where all cards' labels are distinct: `23456`
//!
//! Hands are primarily ordered based on type; for example, every *full house*
//! is stronger than any *three of a kind*.
//!
//! If two hands have the same type, a second ordering rule takes effect. Start
//! by comparing the *first card in each hand*. If these cards are different,
//! the hand with the stronger first card is considered stronger. If the first
//! card in each hand have the *same label*, however, then move on to
//! considering the *second card in each hand*. If they differ, the hand with
//! the higher second card wins; otherwise, continue with the third card in each
//! hand, then the fourth, then the fifth.
//!
//! So, `33332` and `2AAAA` are both *four of a kind* hands, but `33332` is
//! stronger because its first card is stronger. Similarly, `77888` and `77788`
//! are both a *full house*, but `77888` is stronger because its third card is
//! stronger (and both hands have the same first and second card).
//!
//! To play Camel Cards, you are given a list of hands and their corresponding
//! *bid* (your puzzle input). For example:
//!
//! ```
//! 32T3K 765
//! T55J5 684
//! KK677 28
//! KTJJT 220
//! QQQJA 483
//! ```
//!
//! This example shows five hands; each hand is followed by its *bid* amount.
//! Each hand wins an amount equal to its bid multiplied by its *rank*, where
//! the weakest hand gets rank 1, the second-weakest hand gets rank 2, and so on
//! up to the strongest hand. Because there are five hands in this example, the
//! strongest hand will have rank 5 and its bid will be multiplied by 5.
//!
//! So, the first step is to put the hands in order of strength:
//!
//! * `32T3K` is the only *one pair* and the other hands are all a stronger
//!   type, so it gets rank *1*.
//! * `KK677` and `KTJJT` are both *two pair*. Their first cards both have the
//!   same label, but the second card of `KK677` is stronger (`K` vs `T`), so
//!   `KTJJT` gets rank *2* and `KK677` gets rank *3*.
//! * `T55J5` and `QQQJA` are both *three of a kind*. `QQQJA` has a stronger
//!   first card, so it gets rank *5* and `T55J5` gets rank *4*.
//!
//! Now, you can determine the total winnings of this set of hands by adding up
//! the result of multiplying each hand's bid with its rank (`765` \* 1 + `220`
//! \* 2 + `28` \* 3 + `684` \* 4 + `483` \* 5). So the *total winnings* in this
//! example are `*6440*`.
//!
//! Find the rank of every hand in your set. *What are the total winnings?*
//!
//! Your puzzle answer was `246912307`.
//!
//! The first half of this puzzle is complete! It provides one gold star: \*
//!
//! \--- Part Two ---
//! ----------
//!
//! To make things a little more interesting, the Elf introduces one additional
//! rule. Now, `J` cards are [jokers](https://en.wikipedia.org/wiki/Joker_(playing_card)) - wildcards that can act like whatever card would make the hand the strongest type possible.
//!
//! To balance this, *`J` cards are now the weakest* individual cards, weaker
//! even than `2`. The other cards stay in the same order: `A`, `K`, `Q`, `T`,
//! `9`, `8`, `7`, `6`, `5`, `4`, `3`, `2`, `J`.
//!
//! `J` cards can pretend to be whatever card is best for the purpose of
//! determining hand type; for example, `QJJQ2` is now considered *four of a
//! kind*. However, for the purpose of breaking ties between two hands of the
//! same type, `J` is always treated as `J`, not the card it's pretending to be:
//! `JKKK2` is weaker than `QQQQ2` because `J` is weaker than `Q`.
//!
//! Now, the above example goes very differently:
//!
//! ```
//! 32T3K 765
//! T55J5 684
//! KK677 28
//! KTJJT 220
//! QQQJA 483
//! ```
//!
//! * `32T3K` is still the only *one pair*; it doesn't contain any jokers, so
//!   its strength doesn't increase.
//! * `KK677` is now the only *two pair*, making it the second-weakest hand.
//! * `T55J5`, `KTJJT`, and `QQQJA` are now all *four of a kind*! `T55J5` gets
//!   rank 3, `QQQJA` gets rank 4, and `KTJJT` gets rank 5.
//!
//! With the new joker rule, the total winnings in this example are `*5905*`.
//!
//! Using the new joker rule, find the rank of every hand in your set. *What are
//! the new total winnings?*
//!
//! Answer:
//!
//! Although it hasn't changed, you can still [get your puzzle input](7/input).
//! <!---ENDOFDESCRIPTION--->
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
