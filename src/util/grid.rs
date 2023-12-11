use std::fmt::{Debug, Display, Formatter, Write};
use std::ops::Index;

/// A 2D grid of `char`s
pub struct Grid {
    backer: Vec<Vec<char>>,
    rows: Vec<Vec<char>>,
}

impl Grid {
    pub fn parse(input: &str) -> Self {
        let mut backer = Vec::new();

        for line in input.lines() {
            for (x, c) in line.chars().enumerate() {
                if x >= backer.len() {
                    backer.push(Vec::new());
                }

                backer[x].push(c);
            }
        }

        let rows = (0..backer[0].len())
            .map(|row| backer.iter().map(|c| c[row]).collect::<Vec<_>>())
            .collect();

        Self { backer, rows }
    }

    /// Rebuilds the grid. Call when the grid has been updated.
    pub fn rebuild(&mut self) {
        self.rows = (0..self.backer[0].len())
            .map(|row| self.iter_columns().map(|c| c[row]).collect::<Vec<_>>())
            .collect();
    }

    /// Returns an iterator over the columns of this grid
    pub fn iter_columns(&self) -> impl Iterator<Item = &Vec<char>> {
        self.backer.iter()
    }

    /// Returns an iterator over the rows of this grid
    pub fn iter_rows(&self) -> impl Iterator<Item = &Vec<char>> {
        self.rows.iter()
    }

    /// Returns an iterator over all coordinates in this grid
    pub fn iter_points<'a>(&'a self) -> impl Iterator<Item = (usize, usize)> + 'a {
        self.iter_rows().enumerate().flat_map(move |(row_idx, row)| {
            row.iter().enumerate().map(move |(col_idx, _)| (col_idx, row_idx))
        })
    }

    /// Returns an iterator over all the coordinates of the provided character
    pub fn find<'a>(&'a self, search_char: char) -> impl Iterator<Item = (usize, usize)> + 'a {
        self
            .iter_points()
            .filter(move |p| self[p] == search_char)
    }
}

impl Index<(usize, usize)> for Grid {
    type Output = char;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.backer[index.0][index.1]
    }
}

impl Index<&(usize, usize)> for Grid {
    type Output = char;

    fn index(&self, index: &(usize, usize)) -> &Self::Output {
        &self.backer[index.0][index.1]
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.backer[0].len() {
            for x in 0..self.backer.len() {
                f.write_char(self[(x, y)])?;
            }

            f.write_char('\n')?;
        }

        Ok(())
    }
}

impl Debug for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for x in 0..self.backer.len() {
            f.write_str(&format!("{}", x % 10))?;
        }

        for y in 0..self.backer[0].len() {
            f.write_str(&format!("{}", y % 10))?;

            for x in 0..self.backer.len() {
                f.write_char(self[(x, y)])?;
            }

            f.write_char('\n')?;
        }

        Ok(())
    }
}
