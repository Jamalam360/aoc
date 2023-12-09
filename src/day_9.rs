pub mod part_1 {
    use crate::util::DifferenceExt;

    pub fn solution(input: String) -> i32 {
        parse(&input).iter_mut().map(find_next_term).sum()
    }

    fn find_next_term(sequence: &mut Vec<i32>) -> i32 {
        let mut differences = sequence.iter().cloned().differences().collect::<Vec<_>>();
        let mut values = Vec::new();

        loop {
            if differences.iter().all(|d| *d == 0) {
                return values
                    .iter()
                    .fold(*sequence.last().unwrap(), |acc, b| acc + b);
            }

            values.push(differences[differences.len() - 1]);

            differences = differences.iter().cloned().differences().collect();
        }
    }

    fn parse(input: &str) -> Vec<Vec<i32>> {
        let mut result = Vec::with_capacity(200);

        for input in input.lines() {
            result.push(
                input
                    .split(" ")
                    .map(|a| a.parse::<i32>().unwrap())
                    .collect::<Vec<_>>(),
            );
        }

        result
    }

    #[test]
    pub fn test() {
        crate::util::test_solution(9, solution, 1684566095);
    }
}

pub mod part_2 {
    use crate::util::DifferenceExt;

    pub fn solution(input: String) -> i32 {
        parse(&input).iter_mut().map(find_previous_term).sum()
    }

    fn find_previous_term(sequence: &mut Vec<i32>) -> i32 {
        let mut differences = sequence.iter().cloned().differences().collect::<Vec<_>>();
        let mut values = Vec::new();

        loop {
            if differences.iter().all(|d| *d == 0) {
                return sequence.first().unwrap()
                    - values
                        .iter()
                        .rev()
                        .fold(None, |acc, &x| match acc {
                            Some(val) => Some(x - val),
                            None => Some(x),
                        })
                        .unwrap();
            }

            values.push(differences[0]);

            differences = differences.iter().cloned().differences().collect();
        }
    }

    fn parse(input: &str) -> Vec<Vec<i32>> {
        let mut result = Vec::with_capacity(200);

        for input in input.lines() {
            result.push(
                input
                    .split(" ")
                    .map(|a| a.parse::<i32>().unwrap())
                    .collect::<Vec<_>>(),
            );
        }

        result
    }

    #[test]
    pub fn test() {
        crate::util::test_solution(9, solution, 1136);
    }
}
