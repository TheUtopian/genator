use std::time::Instant;
use genator::{Parser, request};

fn main() {
	let req = request(std::env::args()).unwrap();

	let init = Instant::now();

	let parser = Parser::new(&req);
	let gen = parser.iter();

	let parsing_time = init.elapsed().as_micros();

	let combs = match gen.combs() {
		Some(x) => x,
		None => panic!("Too much combinations!"),
	};

	let combs_w = (combs as f32).log10().floor() as usize + 1;

	let start = Instant::now();

	for (i, result) in gen.enumerate() {
		println!("{:0combs_w$} | {result}", i);
	}
	
	let elapsed = start.elapsed().as_millis();

	println!("-----------------\nParsing: {parsing_time}µs\n-----------------");
	println!("-----------------\nElapsed: {elapsed}ms\n-----------------");
	println!("-----------------\nCombinations: {combs}\n-----------------");
}