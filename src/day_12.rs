pub mod part_1 {
    use rayon::prelude::*;

    use crate::util::parse_dual_column;

    pub fn solution(input: String) -> usize {
        parse_dual_column(&input)
            .par_bridge()
            .map(|(row, groups)| {
                (
                    row.chars().collect::<Vec<_>>(),
                    groups
                        .split(',')
                        .filter_map(|s| s.parse::<usize>().ok())
                        .collect::<Vec<_>>(),
                )
            })
            .map(|(row, mut groups)| find_combinations(&row, &mut groups, &State::Next))
            .sum()
    }

    enum State {
        InGroup,
        InGap,
        Next,
    }

    fn find_combinations(row: &[char], groups: &mut Vec<usize>, state: &State) -> usize {
        if (row.is_empty() && groups.is_empty())
            || (groups.is_empty() && row.iter().filter(|c| **c == '#').count() == 0)
        {
            return 1;
        } else if groups.is_empty() || row.is_empty() {
            return 0;
        } else if row[0] == '?' {
            let opt_a = &mut row.iter().cloned().collect::<Vec<_>>();
            opt_a[0] = '#';
            let opt_b = &mut row.iter().cloned().collect::<Vec<_>>();
            opt_b[0] = '.';

            return find_combinations(opt_a, groups, state)
                + find_combinations(opt_b, groups, state);
        } else if row[0] == '.' {
            return match state {
                State::InGroup => 0,
                _ => find_combinations(&row[1..], groups, &State::Next),
            };
        } else if row[0] == '#' {
            return match state {
                State::InGap => 0,
                _ => {
                    let mut new_groups = groups.clone();
                    new_groups[0] -= 1;

                    if new_groups[0] == 0 {
                        find_combinations(
                            &row[1..],
                            &mut new_groups[1..].into_iter().cloned().collect(),
                            &State::InGap,
                        )
                    } else {
                        find_combinations(&row[1..], &mut new_groups, &State::InGroup)
                    }
                }
            };
        } else {
            unreachable!()
        }
    }

    #[test]
    pub fn test() {
        crate::util::test_solution(12, solution, 7118);
    }
}

pub mod part_2 {
    use cached::proc_macro::cached;
    use rayon::prelude::*;

    use crate::util::parse_dual_column;

    pub fn solution(input: String) -> usize {
        parse_dual_column(&input)
            .par_bridge()
            .map(|(row, groups)| {
                (
                    row.chars().collect::<Vec<_>>(),
                    groups
                        .split(',')
                        .filter_map(|s| s.parse::<usize>().ok())
                        .collect::<Vec<_>>(),
                )
            })
            .map(|(row, groups)| {
                let len = groups.len() * 5;

                (
                    expand_row(row),
                    groups.into_iter().cycle().take(len).collect::<Vec<_>>(),
                )
            })
            .map(|(row, mut groups)| find_combinations(&row, &mut groups, &State::Next))
            .sum()
    }

    #[derive(Debug, Clone, Hash, PartialEq)]
    enum State {
        InGroup,
        InGap,
        Next,
    }

    impl Eq for State {}

    #[cached(
        key = "u64",
        convert = r#"{
            use std::hash::{Hash, Hasher};
            let mut h = std::collections::hash_map::DefaultHasher::new();
            row.hash(&mut h);
            groups.hash(&mut h);
            state.hash(&mut h);
            h.finish()
        }"#
    )]
    fn find_combinations(row: &[char], groups: &mut Vec<usize>, state: &State) -> usize {
        if (row.is_empty() && groups.is_empty())
            || (groups.is_empty() && row.iter().filter(|c| **c == '#').count() == 0)
        {
            return 1;
        } else if groups.is_empty() || row.is_empty() {
            return 0;
        } else if row[0] == '?' {
            let opt_a = &mut row.iter().cloned().collect::<Vec<_>>();
            opt_a[0] = '#';
            let opt_b = &mut row.iter().cloned().collect::<Vec<_>>();
            opt_b[0] = '.';

            return find_combinations(opt_a, groups, state)
                + find_combinations(opt_b, groups, state);
        } else if row[0] == '.' {
            return match state {
                State::InGroup => 0,
                _ => find_combinations(&row[1..], groups, &State::Next),
            };
        } else if row[0] == '#' {
            return match state {
                State::InGap => 0,
                _ => {
                    let mut new_groups = groups.clone();
                    new_groups[0] -= 1;

                    if new_groups[0] == 0 {
                        find_combinations(
                            &row[1..],
                            &mut new_groups[1..].into_iter().cloned().collect(),
                            &State::InGap,
                        )
                    } else {
                        find_combinations(&row[1..], &mut new_groups, &State::InGroup)
                    }
                }
            };
        } else {
            unreachable!()
        }
    }

    fn expand_row(row: Vec<char>) -> Vec<char> {
        let mut result = Vec::with_capacity(row.len() * 5);

        for _ in 0..4 {
            result.extend(&row);
            result.push('?');
        }

        result.extend(&row);
        result
    }

    #[test]
    pub fn test() {
        crate::util::test_solution(12, solution, 7118);
    }
}
