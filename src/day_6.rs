pub mod part_1 {
    use crate::util;

    pub fn solution(input: String) -> u32 {
        let result = parse::<4>(&input);
        let mut product = 1u32;

        for idx in 0..result[0].len() {
            let (sol_1, sol_2) = util::solve_quadratic(-1, result[0][idx], -result[1][idx]);
            product *= (sol_2.ceil() - sol_1.floor() - 1f64) as u32;
        }

        product
    }

    fn parse<const N: usize>(input: &str) -> [[i32; N]; 2] {
        let (mut time, mut distance) = input.split_once('\n').unwrap();
        (time, distance) = (&time[10..], &distance[10..]);
        let mut result = [[0i32; N]; 2];

        for (idx, time) in time
            .split_ascii_whitespace()
            .map(|n| n.parse::<i32>())
            .enumerate()
        {
            result[0][idx] = time.unwrap();
        }

        for (idx, distance) in distance
            .split_ascii_whitespace()
            .map(|n| n.parse::<i32>())
            .enumerate()
        {
            result[1][idx] = distance.unwrap();
        }

        result
    }

    #[test]
    pub fn test() {
        util::test_solution(6, solution, 316800);
    }
}

pub mod part_2 {
    use crate::util;

    pub fn solution(input: String) -> u32 {
        let (time, distance) = parse::<4>(&input);
        let (sol_1, sol_2) = util::solve_quadratic(-1f64, time as f64, -distance as f64);
        (sol_2.floor() as u64 - sol_1.floor() as u64) as u32
    }

    fn parse<const N: usize>(input: &str) -> (i64, i64) {
        let (mut time, mut distance) = input.split_once('\n').unwrap();
        (time, distance) = (&time[10..], &distance[10..]);
        let mut final_time = String::with_capacity(N);
        let mut final_distance = String::with_capacity(N);

        for time in time.split_ascii_whitespace() {
            final_time.push_str(time);
        }

        for distance in distance.split_ascii_whitespace() {
            final_distance.push_str(distance);
        }

        (final_time.parse().unwrap(), final_distance.parse().unwrap())
    }

    #[test]
    pub fn test() {
        util::test_solution(6, solution, 45647654);
    }
}
