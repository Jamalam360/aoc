pub mod part_1 {
    pub fn solution(input: String) -> usize {
        let (start_pos, grid) = parse(&input);
        let mut path = Vec::new();
        let mut pos = start_pos;

        loop {
            for direction in directions(grid[pos.0][pos.1]) {
                let new_pos = add(pos, direction, (grid[0].len(), grid.len()));

                if let Some(new_pos) = new_pos {
                    if new_pos == start_pos && path.len() > 3 {
                        pos = start_pos;
                        break;
                    } else if path.contains(&new_pos) || new_pos == start_pos {
                        continue;
                    } else if connects(pos, direction, &grid) {
                        path.push(pos);
                        pos = new_pos;
                        break;
                    }
                }
            }

            if pos == start_pos {
                break;
            }
        }

        (path.len() + 1) / 2
    }

    fn parse(input: &str) -> ((usize, usize), Vec<Vec<char>>) {
        let mut result = vec![Vec::new(); 140];
        let mut start = None;

        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == 'S' {
                    start = Some((x, y));
                }

                result[x].push(c);
            }
        }

        (start.unwrap(), result)
    }

    fn add(
        lhs: (usize, usize),
        rhs: (isize, isize),
        grid_size: (usize, usize),
    ) -> Option<(usize, usize)> {
        let new_x = lhs.0 as isize + rhs.0;
        let new_y = lhs.1 as isize + rhs.1;

        let within_bounds = |coord, max| coord >= 0 && coord < max as isize;

        if within_bounds(new_x, grid_size.0) && within_bounds(new_y, grid_size.1) {
            Some((new_x as usize, new_y as usize))
        } else {
            None
        }
    }

    fn directions(tile: char) -> Vec<(isize, isize)> {
        match tile {
            'S' => vec![(-1, 0), (1, 0), (0, -1), (0, 1)],
            'F' => vec![(0, 1), (1, 0)],
            '|' => vec![(0, -1), (0, 1)],
            'L' => vec![(0, -1), (1, 0)],
            '-' => vec![(-1, 0), (1, 0)],
            'J' => vec![(0, -1), (-1, 0)],
            '7' => vec![(0, 1), (-1, 0)],
            '.' => vec![],
            _ => unreachable!(),
        }
    }

    fn connects(pos: (usize, usize), mov: (isize, isize), grid: &Vec<Vec<char>>) -> bool {
        if grid[pos.0][pos.1] == '.' {
            return false;
        }

        if let Some(new_pos) = add(pos, mov, (grid[0].len(), grid.len())) {
            let new_tile = grid[new_pos.0][new_pos.1];

            if new_tile == 'S' {
                let current_tile = grid[pos.0][pos.1];

                return if mov.1 == -1 {
                    current_tile == 'F' || current_tile == '|' || current_tile == '7'
                } else if mov.1 == 1 {
                    current_tile == 'L' || current_tile == '|' || current_tile == 'J'
                } else if mov.0 == 1 {
                    current_tile == '7' || current_tile == '-' || current_tile == 'J'
                } else if mov.0 == -1 {
                    current_tile == 'F' || current_tile == '-' || current_tile == 'L'
                } else {
                    false
                };
            }

            match new_tile {
                'F' => mov == (0, -1) || mov == (-1, 0),
                '|' => mov == (0, 1) || mov == (0, -1),
                'L' => mov == (0, 1) || mov == (-1, 0),
                '-' => mov == (1, 0) || mov == (-1, 0),
                'J' => mov == (0, 1) || mov == (1, 0),
                '7' => mov == (0, -1) || mov == (1, 0),
                '.' => false,
                _ => unreachable!(),
            }
        } else {
            return false;
        }
    }

    #[test]
    pub fn test() {
        crate::util::test_solution(10, solution, 6815);
    }
}

pub mod part_2 {
    pub fn solution(input: String) -> usize {
        let (start_pos, grid) = parse(&input);
        let mut path = Vec::new();
        let mut pos = start_pos;

        loop {
            for direction in directions(grid[pos.0][pos.1]) {
                let new_pos = add(pos, direction, (grid[0].len(), grid.len()));

                if let Some(new_pos) = new_pos {
                    if new_pos == start_pos && path.len() > 3 {
                        path.push(pos);
                        pos = start_pos;
                        break;
                    } else if path.contains(&new_pos) || new_pos == start_pos {
                        continue;
                    } else if connects(pos, direction, &grid) {
                        path.push(pos);
                        pos = new_pos;
                        break;
                    }
                }
            }

            if pos == start_pos {
                break;
            }
        }

        let s_has_north_end = path[0].1 - path[1].1 == 1;
        let mut count = 0;
        let mut upwards_facing_pipes_to_left = 0;

        for y in 0..grid[0].len() {
            for x in 0..grid.len() {
                if path.contains(&(x, y)) {
                    let pipe = grid[x][y];
                    if pipe == '|' || pipe == 'L' || pipe == 'J' || (pipe == 'S' && s_has_north_end) {
                        upwards_facing_pipes_to_left += 1;
                    }
                } else if upwards_facing_pipes_to_left % 2 == 1 {
                    count += 1;
                }
            }

            upwards_facing_pipes_to_left = 0;
        }

        count
    }

    fn parse(input: &str) -> ((usize, usize), Vec<Vec<char>>) {
        let mut result: Vec<Vec<char>> = Vec::new();
        let mut start = None;

        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == 'S' {
                    start = Some((x, y));
                }

                if x >= result.len() {
                    result.push(Vec::new());
                }

                result[x].push(c);
            }
        }

        (start.unwrap(), result)
    }

    fn add(
        lhs: (usize, usize),
        rhs: (isize, isize),
        grid_size: (usize, usize),
    ) -> Option<(usize, usize)> {
        let new_x = lhs.0 as isize + rhs.0;
        let new_y = lhs.1 as isize + rhs.1;

        let within_bounds = |coord, max| coord >= 0 && coord < max as isize;

        if within_bounds(new_x, grid_size.0) && within_bounds(new_y, grid_size.1) {
            Some((new_x as usize, new_y as usize))
        } else {
            None
        }
    }

    fn directions(tile: char) -> Vec<(isize, isize)> {
        match tile {
            'S' => vec![(-1, 0), (1, 0), (0, -1), (0, 1)],
            'F' => vec![(0, 1), (1, 0)],
            '|' => vec![(0, -1), (0, 1)],
            'L' => vec![(0, -1), (1, 0)],
            '-' => vec![(-1, 0), (1, 0)],
            'J' => vec![(0, -1), (-1, 0)],
            '7' => vec![(0, 1), (-1, 0)],
            '.' => vec![],
            _ => unreachable!(),
        }
    }

    fn connects(pos: (usize, usize), mov: (isize, isize), grid: &Vec<Vec<char>>) -> bool {
        if grid[pos.0][pos.1] == '.' {
            return false;
        }

        if let Some(new_pos) = add(pos, mov, (grid[0].len(), grid.len())) {
            let new_tile = grid[new_pos.0][new_pos.1];

            if new_tile == 'S' {
                let current_tile = grid[pos.0][pos.1];

                return if mov.1 == -1 {
                    current_tile == 'F' || current_tile == '|' || current_tile == '7'
                } else if mov.1 == 1 {
                    current_tile == 'L' || current_tile == '|' || current_tile == 'J'
                } else if mov.0 == 1 {
                    current_tile == '7' || current_tile == '-' || current_tile == 'J'
                } else if mov.0 == -1 {
                    current_tile == 'F' || current_tile == '-' || current_tile == 'L'
                } else {
                    false
                };
            }

            match new_tile {
                'F' => mov == (0, -1) || mov == (-1, 0),
                '|' => mov == (0, 1) || mov == (0, -1),
                'L' => mov == (0, 1) || mov == (-1, 0),
                '-' => mov == (1, 0) || mov == (-1, 0),
                'J' => mov == (0, 1) || mov == (1, 0),
                '7' => mov == (0, -1) || mov == (1, 0),
                '.' => false,
                _ => unreachable!("{new_tile}"),
            }
        } else {
            return false;
        }
    }

    #[test]
    pub fn test() {
        crate::util::test_solution(10, crate::day_10::part_1::solution, 6815);
    }
}
