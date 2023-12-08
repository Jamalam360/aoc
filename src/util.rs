use std::time::Duration;

pub fn run_solution<F, R>(day: usize, func: F)
where
    F: FnOnce(String) -> R,
    R: std::fmt::Display,
{
    let input = std::fs::read_to_string(format!("src/inputs/{}.txt", day))
        .expect("unable to read input file");
    let instant = std::time::Instant::now();
    let result = func(input);
    println!("Took {:?}", instant.elapsed());
    println!("{}", result);
}

pub fn bench_solution<F, R>(day: usize, func: F)
where
    F: Fn(String) -> R,
    R: std::fmt::Display,
{
    let mut sum = Duration::new(0, 0);

    for _ in 0..1_000 {
        // yes, reading the file every iteration. I'm lazy
        let input = std::fs::read_to_string(format!("src/inputs/{}.txt", day))
            .expect("unable to read input file");
        let instant = std::time::Instant::now();
        func(input);
        sum += instant.elapsed();
    }

    println!("Took {:?} on average", sum / 1_000);
}

#[cfg(test)]
pub fn test_solution<F, R>(day: usize, func: F, expected: R)
where
    F: FnOnce(String) -> R,
    R: std::fmt::Display + PartialEq + std::fmt::Debug,
{
    let input = std::fs::read_to_string(format!("src/inputs/{}.txt", day))
        .expect("unable to read input file");
    let result = func(input);
    assert_eq!(result, expected)
}

pub fn to_denary(c: char) -> Option<u32> {
    c.to_digit(10)
}

pub fn number_length(mut n: u32) -> usize {
    let mut length = 0;

    while n > 0 {
        n /= 10;
        length += 1;
    }

    length
}

#[inline(always)]
pub fn solve_quadratic(a: i32, b: i32, c: i32) -> (f32, f32) {
    let discrim = (b as f32).powi(2) - 4.0 * (a as f32) * (c as f32);
    let root = discrim.sqrt();
    let denom = 2.0 * (a as f32);

    ((-b as f32 + root) / denom, (-b as f32 - root) / denom)
}

#[inline(always)]
pub fn solve_long_quadratic(a: i64, b: i64, c: i64) -> (f64, f64) {
    let discrim = (b as f64).powi(2) - 4.0 * (a as f64) * (c as f64);
    let root = discrim.sqrt();
    let denom = 2.0 * (a as f64);

    ((-b as f64 + root) / denom, (-b as f64 - root) / denom)
}

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

#[inline(always)]
pub fn lcm(a: usize, b: usize) -> usize {
    a * (b / gcd(a, b))
}

/// The position of iter.max()
#[inline(always)]
pub fn max_position<I>(mut iter: impl Iterator<Item = I>) -> Option<usize>
where
    I: Ord,
{
    let mut i = 0usize;
    let mut max = None;

    while let Some(next) = iter.next() {
        if let Some((_, prev_max)) = &max {
            if &next > prev_max {
                max = Some((i, next))
            }
        } else {
            max = Some((i, next));
        }

        i += 1;
    }

    max.map(|i| i.0)
}
