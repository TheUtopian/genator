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

	for (i, result) in gen.enumerate() {
		println!("| {} | {}", result, i);
	}

	println!("-----------------\nElapsed: {}ms\n-----------------", start.elapsed().as_millis());
	println!("-----------------\nCombinations: {}\n-----------------", combs);
}