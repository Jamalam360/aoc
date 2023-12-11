// Looking back, this solution is incredibly stupid. Don't read it.
pub mod part_1 {
    use crate::util::number_length;

    #[derive(Debug)]
    struct PotentialPartNumber {
        number: usize,
        line: usize,
        column: usize,
        has_symbol: bool,
    }

    pub fn solution(input: String) -> usize {
        let mut potentials: Vec<PotentialPartNumber> = Vec::new();
        let mut previous_line_symbol_columns: Vec<usize> = Vec::new();

        for (line_num, line) in input.split('\n').enumerate() {
            potentials.retain(|p| p.line >= line_num - 1 || p.has_symbol);

            let mut previous_was_symbol = false;
            let mut current_number: Option<PotentialPartNumber> = None;
            let mut this_line_symbol_columns: Vec<usize> = Vec::new();

            for (column, c) in line.chars().enumerate() {
                if c.is_ascii_digit() {
                    match current_number {
                        Some(ref mut p) => {
                            p.number = p.number * 10 + c.to_digit(10).unwrap() as usize
                        }
                        None => {
                            current_number = Some(PotentialPartNumber {
                                number: c.to_digit(10).unwrap() as usize,
                                line: line_num,
                                column,
                                has_symbol: previous_was_symbol,
                            })
                        }
                    };
                }

                if !c.is_ascii_digit() || column == line.len() - 1 {
                    if let Some(mut p) = current_number {
                        p.has_symbol |= previous_line_symbol_columns.iter().any(|c| {
                            is_adjacent_to(
                                number_length(p.number),
                                line_num,
                                column - number_length(p.number),
                                line_num - 1,
                                *c,
                            )
                        });
                        potentials.push(p);
                        current_number = None;
                    }
                }

                if is_symbol(c) {
                    for p in potentials.iter_mut() {
                        p.has_symbol |= is_adjacent_to(
                            number_length(p.number),
                            p.line,
                            p.column,
                            line_num,
                            column,
                        );
                    }

                    previous_was_symbol = true;
                    this_line_symbol_columns.push(column);
                } else {
                    previous_was_symbol = false;
                }
            }

            previous_line_symbol_columns = this_line_symbol_columns;
        }

        potentials
            .iter()
            .filter(|p| p.has_symbol)
            .map(|p| p.number)
            .sum()
    }

    fn is_symbol(c: char) -> bool {
        !c.is_ascii_digit() && c != '.'
    }

    fn is_adjacent_to(
        length: usize,
        line_one: usize,
        column_one: usize,
        line_two: usize,
        column_two: usize,
    ) -> bool {
        if (line_two > 0 && line_one < line_two - 1) || line_one > line_two + 1 {
            return false;
        }

        if line_one == line_two {
            for i in 0..length {
                if (column_two > 0 && column_one + i == column_two - 1)
                    || column_one + i == column_two + 1
                {
                    return true;
                }
            }
        } else if (line_two > 0 && line_one == line_two - 1) || line_one == line_two + 1 {
            for i in 0..length {
                if column_one + i == column_two {
                    return true;
                }

                if (column_two > 0 && column_one + i == column_two - 1)
                    || column_one + i == column_two + 1
                {
                    return true;
                }
            }
        }

        false
    }

    #[test]
    pub fn test() {
        crate::util::test_solution(3, solution, 526404);
    }
}

pub mod part_2 {
    use crate::util::number_length;

    #[derive(Debug)]
    struct PotentialPartNumber {
        number: usize,
        line: usize,
        column: usize,
        symbols: Vec<(usize, usize)>,
    }

    pub fn solution(input: String) -> usize {
        let mut potentials: Vec<PotentialPartNumber> = Vec::new();
        let mut previous_line_symbol_columns: Vec<usize> = Vec::new();

        for (line_num, line) in input.split('\n').enumerate() {
            potentials.retain(|p| !(p.line < line_num - 1 && p.symbols.is_empty()));

            let mut previous_was_symbol = false;
            let mut current_number: Option<PotentialPartNumber> = None;
            let mut this_line_symbol_columns: Vec<usize> = Vec::new();

            for (column, c) in line.chars().enumerate() {
                if c.is_ascii_digit() {
                    match current_number {
                        Some(ref mut p) => {
                            p.number = p.number * 10 + c.to_digit(10).unwrap() as usize
                        }
                        None => {
                            current_number = Some(PotentialPartNumber {
                                number: c.to_digit(10).unwrap() as usize,
                                line: line_num,
                                column,
                                symbols: if previous_was_symbol {
                                    vec![(line_num, column - 1)]
                                } else {
                                    Vec::new()
                                },
                            })
                        }
                    };
                }

                if !c.is_ascii_digit() || column == line.len() - 1 {
                    if let Some(mut p) = current_number {
                        previous_line_symbol_columns.iter().for_each(|c| {
                            if is_adjacent_to(
                                number_length(p.number),
                                line_num,
                                p.column,
                                line_num - 1,
                                *c,
                            ) {
                                p.symbols.push((line_num - 1, *c));
                            }
                        });

                        potentials.push(p);
                        current_number = None;
                    }
                }

                if is_symbol(c) {
                    for p in potentials.iter_mut() {
                        if is_adjacent_to(
                            number_length(p.number),
                            p.line,
                            p.column,
                            line_num,
                            column,
                        ) {
                            p.symbols.push((line_num, column));
                        }
                    }

                    previous_was_symbol = true;
                    this_line_symbol_columns.push(column);
                } else {
                    previous_was_symbol = false;
                }
            }

            previous_line_symbol_columns = this_line_symbol_columns;
        }

        let lines = input.split('\n').collect::<Vec<_>>();

        let iter = potentials
            .iter()
            .filter(|p| !p.symbols.is_empty())
            .filter(|p| {
                p.symbols
                    .iter()
                    .any(|(line, col)| lines[*line].chars().nth(*col).unwrap() == '*')
            });

        let mut products = Vec::new();

        for p in iter {
            for symbol in &p.symbols {
                match products.iter_mut().find(|(s, _)| s == symbol) {
                    Some((_, (n, v))) => {
                        *n += 1;
                        *v *= p.number;
                    }
                    None => {
                        products.push((*symbol, (1, p.number)));
                    }
                }
            }
        }

        products
            .iter()
            .filter(|(_, (n, _))| *n > 1)
            .map(|(_, (_, v))| v)
            .sum()
    }

    fn is_symbol(c: char) -> bool {
        !c.is_ascii_digit() && c != '.'
    }

    fn is_adjacent_to(
        length: usize,
        line_one: usize,
        column_one: usize,
        line_two: usize,
        column_two: usize,
    ) -> bool {
        if (line_two > 0 && line_one < line_two - 1) || line_one > line_two + 1 {
            return false;
        }

        if line_one == line_two {
            for i in 0..length {
                if (column_two > 0 && column_one + i == column_two - 1)
                    || column_one + i == column_two + 1
                {
                    return true;
                }
            }
        } else if (line_two > 0 && line_one == line_two - 1) || line_one == line_two + 1 {
            for i in 0..length {
                if column_one + i == column_two {
                    return true;
                }

                if (column_two > 0 && column_one + i == column_two - 1)
                    || column_one + i == column_two + 1
                {
                    return true;
                }
            }
        }

        false
    }

    #[test]
    pub fn test() {
        crate::util::test_solution(3, solution, 84399773);
    }
}
