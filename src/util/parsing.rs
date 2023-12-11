/// Shorthand for `Char::to_digit(10)`
#[inline(always)]
pub fn to_denary(c: char) -> Option<u32> {
    c.to_digit(10)
}

/// Number of digits in a number
pub fn number_length(mut n: usize) -> usize {
    let mut length = 0;

    while n > 0 {
        n /= 10;
        length += 1;
    }

    length
}

/// Solves a quadratic equation using the quadratic formula.
#[inline(always)]
pub fn solve_quadratic<T>(a: T, b: T, c: T) -> (f64, f64)
where
    T: Into<f64> + Copy,
{
    let a_f64 = a.into();
    let b_f64 = b.into();
    let c_f64 = c.into();
    let discrim = b_f64.powi(2) - 4.0 * a_f64 * c_f64;
    let root = discrim.sqrt();
    let denom = 2.0 * a_f64;

    ((-b_f64 + root) / denom, (-b_f64 - root) / denom)
}

/// Greatest Common Divisor/Denominator of two numbers
#[inline(always)]
pub fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        if b < a {
            std::mem::swap(&mut b, &mut a);
        }

        b %= a;
    }

    a
}

/// Lowest Common Multiple of two numbers
#[inline(always)]
pub fn lcm(a: usize, b: usize) -> usize {
    a * (b / gcd(a, b))
}
