pub fn run_solution<F, R>(day: usize, func: F) where F: FnOnce(String) -> R, R: std::fmt::Display {
	let input = std::fs::read_to_string(format!("src/inputs/{}.txt", day)).expect("unable to read input file");
	let instant = std::time::Instant::now();
	let result = func(input);
	println!("Took {:?}", instant.elapsed());
	println!("{}", result);
}

pub fn bench_solution<F, R>(day: usize, func: F) where F: Fn(String) -> R, R: std::fmt::Display {
	let mut sum = 0;
	
	for _ in 0..10_000 {
		let input = std::fs::read_to_string(format!("src/inputs/{}.txt", day)).expect("unable to read input file");
		let instant = std::time::Instant::now();
		func(input);
		sum += instant.elapsed().as_micros();
	}

	println!("Took {}micros on average", sum / 10_000);
}

#[cfg(test)]
pub fn test_solution<F, R>(day: usize, func: F, expected: R) where F: FnOnce(String) -> R, R: std::fmt::Display + std::cmp::PartialEq + std::fmt::Debug {
	let input = std::fs::read_to_string(format!("src/inputs/{}.txt", day)).expect("unable to read input file");
	let result = func(input);
	assert_eq!(result, expected)
}

pub fn to_denary(c: char) -> Option<u32> {
	c.to_digit(10)
}
