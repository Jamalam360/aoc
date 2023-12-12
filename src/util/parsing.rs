/// Shorthand for `Char::to_digit(10)`
#[inline(always)]
pub fn to_denary(c: char) -> Option<u32> {
    c.to_digit(10)
}

pub fn parse_dual_column(input: &str) -> impl Iterator<Item = (&str, &str)> {
    input.lines().filter_map(|l| l.split_once(' '))
}
