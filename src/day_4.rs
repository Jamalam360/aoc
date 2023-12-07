pub mod part_1 {
    use crate::util::to_denary;

    pub fn solution(input: String) -> u32 {
        input.split('\n').map(get_point_value).sum()
    }

    fn get_point_value(line: &str) -> u32 {
        let winners = parse_numbers::<10>(&line[10..39]);
        let got = parse_numbers::<25>(&line[42..]);
        let mut score = 0;

        for got in got {
            if winners.contains(&got) {
                if score == 0 {
                    score = 1;
                } else {
                    score *= 2;
                }
            }
        }

        score
    }

    fn parse_numbers<const N: usize>(input: &str) -> [u32; N] {
        let mut result = [0; N];
        let mut chars = input.chars();
        let mut i = 0;

        while let Some(c) = chars.next() {
            if c.is_ascii_digit() {
                let mut num = to_denary(c).unwrap();

                for c in chars.by_ref() {
                    if c.is_ascii_digit() {
                        num *= 10;
                        num += to_denary(c).unwrap();
                    } else {
                        break;
                    }
                }

                result[i] = num;
                i += 1;
            }
        }

        result
    }

    #[test]
    pub fn test() {
        crate::util::test_solution(4, solution, 25651);
    }
}

pub mod part_2 {
    use crate::util::to_denary;

    pub fn solution(input: String) -> u32 {
        let lines = input.split('\n').collect::<Vec<_>>();
        let mut cards = vec![1; lines.len()];

        for (idx, line) in lines.iter().enumerate() {
            let points = get_point_value(line);

            for other_idx in (idx + 1)..(idx + 1 + points as usize) {
                cards[other_idx] += cards[idx];
            }
        }

        cards.iter().sum()
    }

    fn get_point_value(line: &str) -> u32 {
        let winners = parse_numbers::<10>(&line[10..39]);
        let got = parse_numbers::<25>(&line[42..]);
        let mut score = 0;

        for got in got {
            if winners.contains(&got) {
                score += 1;
            }
        }

        score
    }

    fn parse_numbers<const N: usize>(input: &str) -> [u32; N] {
        let mut result = [0; N];
        let mut chars = input.chars();
        let mut i = 0;

        while let Some(c) = chars.next() {
            if c.is_ascii_digit() {
                let mut num = to_denary(c).unwrap();

                for c in chars.by_ref() {
                    if c.is_ascii_digit() {
                        num *= 10;
                        num += to_denary(c).unwrap();
                    } else {
                        break;
                    }
                }

                result[i] = num;
                i += 1;
            }
        }

        result
    }

    #[test]
    pub fn test() {
        crate::util::test_solution(4, solution, 19499881);
    }
}
