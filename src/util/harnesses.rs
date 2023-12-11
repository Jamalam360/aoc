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
