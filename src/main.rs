use std::time::Instant;
use genator::Parser;

fn main() {
	let args: Vec<String> = std::env::args().collect();

	if args.len() < 2 || args[1].is_empty() || !args[1].is_ascii() {
		panic!("Write something and don't forget about quotes please.");
	}

	let parser = Parser::new(&args[1]);
	let mut gen = parser.iter();

	let combs = match gen.combs() {
		Some(x) => x,
		None => panic!("Too much combinations!"),
	};

	let start = Instant::now();
	let mut i = 0;

	while let Some(result) = gen.next() {
		println!("{}) {} | ", i, result);
		i += 1;
	}

	println!("-----------------\nCombinations: {}\n-----------------", combs);
	println!("-----------------\nElapsed: {}\n-----------------", start.elapsed().as_millis());
}