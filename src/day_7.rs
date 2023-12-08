pub mod part_1 {
    use std::cmp::Ordering;

    pub fn solution(input: String) -> u32 {
        let mut result = parse(&input);
        result.sort_by(|a, b| {
            if a.0.score() == b.0.score() {
                for idx in 0..a.1.len() {
                    let ordering = a.1[idx].cmp(&b.1[idx]);

                    match ordering {
                        Ordering::Equal => (),
                        _ => return ordering,
                    };
                }

                unreachable!()
            } else {
                a.0.score().cmp(&b.0.score())
            }
        });

        result
            .iter()
            .enumerate()
            .fold(0u32, |acc, (rank, r)| acc + (rank as u32 + 1) * r.2)
    }

    #[derive(Debug)]
    enum HandType {
        FiveOfAKind,
        FourOfAKind,
        FullHouse,
        ThreeOfAKind,
        TwoPair,
        OnePair,
        HighCard,
    }

    impl HandType {
        fn score(&self) -> u32 {
            match self {
                HandType::FiveOfAKind => 7,
                HandType::FourOfAKind => 6,
                HandType::FullHouse => 5,
                HandType::ThreeOfAKind => 4,
                HandType::TwoPair => 3,
                HandType::OnePair => 2,
                HandType::HighCard => 1,
            }
        }
    }

    fn parse(input: &str) -> Vec<(HandType, [u32; 5], u32)> {
        let mut result = Vec::new();

        for (hand, bid) in input.lines().filter_map(|l| l.split_once(' ')) {
            let mut cards = [0; 13];
            let mut scores = [0; 5];

            for (char_idx, card) in hand.chars().enumerate() {
                let idx = match card {
                    '2' => 0,
                    '3' => 1,
                    '4' => 2,
                    '5' => 3,
                    '6' => 4,
                    '7' => 5,
                    '8' => 6,
                    '9' => 7,
                    'T' => 8,
                    'J' => 9,
                    'Q' => 10,
                    'K' => 11,
                    'A' => 12,
                    _ => unreachable!(),
                };

                scores[char_idx] = idx as u32;
                cards[idx] = cards.get(idx).unwrap_or(&0) + 1;
            }

            let mut kind = None;

            let len = cards.iter().filter(|v| **v != 0).count();

            if len == 1 {
                kind = Some(HandType::FiveOfAKind);
            } else if len == 2 {
                if cards.iter().any(|c| *c == 4) {
                    kind = Some(HandType::FourOfAKind);
                } else {
                    kind = Some(HandType::FullHouse);
                }
            } else if len == 3 {
                if cards.iter().any(|c| *c == 2) {
                    kind = Some(HandType::TwoPair);
                } else {
                    kind = Some(HandType::ThreeOfAKind);
                }
            } else if len == 4 {
                kind = Some(HandType::OnePair);
            } else if len == 5 {
                kind = Some(HandType::HighCard);
            }

            result.push((kind.unwrap(), scores, bid.parse::<u32>().unwrap()));
        }

        result
    }

    #[test]
    pub fn test() {
        crate::util::test_solution(7, solution, 251058093);
    }
}

// prepare for most sane advent of code solution
pub mod part_2 {
    use std::cmp::Ordering;
    use crate::util;

    pub fn solution(input: String) -> u32 {
        let mut result = parse(&input);
        result.sort_by(|a, b| {
            if a.0.score() == b.0.score() {
                for idx in 0..a.1.len() {
                    let ordering = a.1[idx].cmp(&b.1[idx]);

                    match ordering {
                        Ordering::Equal => (),
                        _ => return ordering,
                    };
                }

                unreachable!()
            } else {
                a.0.score().cmp(&b.0.score())
            }
        });

        result
            .iter()
            .enumerate()
            .fold(0u32, |acc, (rank, r)| acc + (rank as u32 + 1) * r.2)
    }

    enum HandType {
        FiveOfAKind,
        FourOfAKind,
        FullHouse,
        ThreeOfAKind,
        TwoPair,
        OnePair,
        HighCard,
    }

    impl HandType {
        fn score(&self) -> u32 {
            match self {
                HandType::FiveOfAKind => 7,
                HandType::FourOfAKind => 6,
                HandType::FullHouse => 5,
                HandType::ThreeOfAKind => 4,
                HandType::TwoPair => 3,
                HandType::OnePair => 2,
                HandType::HighCard => 1,
            }
        }
    }

    fn parse(input: &str) -> Vec<(HandType, [u32; 5], u32)> {
        let mut result = Vec::new();

        for (hand, bid) in input.lines().filter_map(|l| l.split_once(' ')) {
            let mut cards = [0; 13];
            let mut scores = [0; 5];

            for (char_idx, card) in hand.chars().enumerate() {
                let idx = match card {
                    'J' => 0,
                    '2' => 1,
                    '3' => 2,
                    '4' => 3,
                    '5' => 4,
                    '6' => 5,
                    '7' => 6,
                    '8' => 7,
                    '9' => 8,
                    'T' => 9,
                    'Q' => 10,
                    'K' => 11,
                    'A' => 12,
                    _ => unreachable!(),
                };

                scores[char_idx] = idx as u32;
                cards[idx] = cards.get(idx).unwrap_or(&0) + 1;
            }

            let mut kind = None;
            cards[util::max_position(cards.iter().skip(1)).unwrap() + 1] += cards[0];
            cards[0] = 0;
            let len = cards.iter().filter(|v| **v != 0).count();

            if len < 2 {
                kind = Some(HandType::FiveOfAKind);
            } else if len == 2 {
                if cards.iter().any(|c| *c == 4) {
                    kind = Some(HandType::FourOfAKind);
                } else {
                    kind = Some(HandType::FullHouse);
                }
            } else if len == 3 {
                if cards.iter().any(|c| *c == 2) {
                    kind = Some(HandType::TwoPair);
                } else {
                    kind = Some(HandType::ThreeOfAKind);
                }
            } else if len == 4 {
                kind = Some(HandType::OnePair);
            } else if len == 5 {
                kind = Some(HandType::HighCard);
            }

            result.push((kind.unwrap(), scores, bid.parse::<u32>().unwrap()));
        }

        result
    }

    #[test]
    pub fn test() {
        util::test_solution(7, solution, 249781879);
    }
}
