pub mod part_1 {
    use crate::util::Grid;
    use itertools::Itertools;

    pub fn solution(input: String) -> usize {
        let grid = Grid::parse(&input);
        let mut distance_sum = 0;

        let column_has_galaxy = grid
            .iter_columns()
            .enumerate()
            .map(|(_, c)| c.iter().any(|c| *c == '#'))
            .collect::<Vec<_>>();
        let row_has_galaxy = grid
            .iter_rows()
            .enumerate()
            .map(|(_, c)| c.iter().any(|c| *c == '#'))
            .collect::<Vec<_>>();

        for pair in grid.find('#').combinations(2) {
            let g1 = pair[0];
            let g2 = pair[1];
            let x_min = g1.0.min(g2.0);
            let x_max = g1.0.max(g2.0);
            let y_min = g1.1.min(g2.1);
            let y_max = g1.1.max(g2.1);

            let mut empty_count = 0;

            for x in x_min..x_max {
                if !column_has_galaxy[x] {
                    empty_count += 1;
                }
            }

            for y in y_min..y_max {
                if !row_has_galaxy[y] {
                    empty_count += 1;
                }
            }

            distance_sum += (x_max - x_min) + (y_max - y_min) + empty_count;
        }

        distance_sum
    }

    #[test]
    pub fn test() {
        crate::util::test_solution(11, solution, 9957702);
    }
}

pub mod part_2 {
    use crate::util::Grid;
    use itertools::Itertools;

    pub fn solution(input: String) -> usize {
        let grid = Grid::parse(&input);
        let mut distance_sum = 0;

        let column_has_galaxy = grid
            .iter_columns()
            .enumerate()
            .map(|(_, c)| c.iter().any(|c| *c == '#'))
            .collect::<Vec<_>>();
        let row_has_galaxy = grid
            .iter_rows()
            .enumerate()
            .map(|(_, c)| c.iter().any(|c| *c == '#'))
            .collect::<Vec<_>>();

        for pair in grid.find('#').combinations(2) {
            let g1 = pair[0];
            let g2 = pair[1];
            let x_min = g1.0.min(g2.0);
            let x_max = g1.0.max(g2.0);
            let y_min = g1.1.min(g2.1);
            let y_max = g1.1.max(g2.1);

            let mut empty_count = 0;

            for x in x_min..x_max {
                if !column_has_galaxy[x] {
                    empty_count += 1;
                }
            }

            for y in y_min..y_max {
                if !row_has_galaxy[y] {
                    empty_count += 1;
                }
            }

            distance_sum += (x_max - x_min) + (y_max - y_min) + empty_count * 999999;
        }

        distance_sum
    }

    #[test]
    pub fn test() {
        crate::util::test_solution(11, solution, 512240933238);
    }
}
