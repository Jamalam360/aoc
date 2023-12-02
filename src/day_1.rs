pub mod part_1 {
    use crate::util::to_denary;

    pub fn solution(input: String) -> u32 {
        input.split("\n").map(line_value).sum()
    }

    fn line_value(line: &str) -> u32 {
        let line = line.as_bytes();
        let mut left_ptr = 0;
        let mut left = None;
        let mut right_ptr = line.len() - 1;
        let mut right = None;

        while left.is_none() || right.is_none() {
            let left_char = line[left_ptr] as char;
            let right_char = line[right_ptr] as char;

            if left_char.is_ascii_digit() {
                left = left.or(Some(left_char));
            }

            if right_char.is_ascii_digit() {
                right = right.or(Some(right_char))
            }

            if left_ptr < line.len() {
                left_ptr += 1;
            }

            if right_ptr > 0 {
                right_ptr -= 1;
            }
        }

        return left.and_then(to_denary).unwrap() * 10 + right.and_then(to_denary).unwrap();
    }

    #[test]
    pub fn test() {
        crate::util::test_solution(1, solution, 54450);
    }
}

pub mod part_2 {
    use crate::util::to_denary;

    pub fn solution(input: String) -> u32 {
        input.split("\n").map(line_value).sum()
    }

    fn line_value(line: &str) -> u32 {
        let line = line.as_bytes();
        let mut left_ptr = 0;
        let mut left = None;
        let mut right_ptr = line.len() - 1;
        let mut right = None;

        while left.is_none() || right.is_none() {
            left = left.or(get_digit(line, left_ptr));
            right = right.or(get_digit(line, right_ptr));

            if left_ptr < line.len() {
                left_ptr += 1;
            }

            if right_ptr > 0 {
                right_ptr -= 1;
            }
        }

        return left.unwrap() * 10 + right.unwrap();
    }

    const DIGITS: &[&[u8]] = &[
        b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine",
    ];

    fn get_digit(chars: &[u8], idx: usize) -> Option<u32> {
        let c = chars[idx] as char;

        if c.is_ascii_digit() {
            return to_denary(c);
        }

        for (digit_idx, digit) in DIGITS.iter().enumerate() {
            if idx + digit.len() <= chars.len() && &&chars[idx..idx + digit.len()] == digit {
                return Some(digit_idx as u32 + 1);
            }
        }

        None
    }

    #[test]
    pub fn test() {
        crate::util::test_solution(1, solution, 54265);
    }
}
