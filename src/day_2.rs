pub mod part_1 {
    pub fn solution(input: String) -> u32 {
        input.split('\n').filter_map(get_id_if_valid).sum()
    }

    fn get_id_if_valid(line: &str) -> Option<u32> {
        let (id, games) = line[5..].split_once(": ")?;

        for game in games.split("; ") {
            for s in game.split(", ") {
                let (count, colour) = s.split_once(' ')?;
                let count = count.parse::<u32>().ok()?;

                match colour {
                    "red" => {
                        if count > 12 {
                            return None;
                        }
                    }
                    "green" => {
                        if count > 13 {
                            return None;
                        }
                    }
                    "blue" => {
                        if count > 14 {
                            return None;
                        }
                    }
                    _ => panic!("unknown colour"),
                }
            }
        }

        id.parse::<u32>().ok()
    }

    #[test]
    pub fn test() {
        crate::util::test_solution(2, solution, 2720);
    }
}

pub mod part_2 {
    pub fn solution(input: String) -> u32 {
        input.split('\n').map(get_power).sum()
    }

    fn get_power(line: &str) -> u32 {
        let (_, games) = line.split_once(": ").unwrap();
        let mut max_values: [u32; 3] = [0, 0, 0];

        for game in games.split("; ") {
            let mut min_for_game: [u32; 3] = [0, 0, 0];

            for s in game.split(", ") {
                let (count, colour) = s.split_once(' ').unwrap();
                let count = count.parse::<u32>().unwrap();

                match colour {
                    "red" => min_for_game[0] += count,
                    "green" => min_for_game[1] += count,
                    "blue" => min_for_game[2] += count,
                    _ => panic!("unknown colour"),
                }
            }

            max_values[0] = max_values[0].max(min_for_game[0]);
            max_values[1] = max_values[1].max(min_for_game[1]);
            max_values[2] = max_values[2].max(min_for_game[2]);
        }

        max_values[0] * max_values[1] * max_values[2]
    }

    #[test]
    pub fn test() {
        crate::util::test_solution(2, solution, 71535);
    }
}
