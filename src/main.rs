use std::env;
use std::ops::{RangeInclusive, Range};
use std::cmp::Ordering;
use std::str::from_utf8_unchecked as from_ascii;
use std::time::{Duration, Instant};
use std::fs::File;
use std::io::prelude::*;

use genator::Parser;

fn main() {
	let args: Vec<String> = env::args().collect();

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

	loop {

		println!("{} | ", gen.get());

		if !gen.next() { break; }
	}

	println!("-----------------\nCombinations: {}\n-----------------", combs);
	println!("-----------------\nElapsed: {}\n-----------------", start.elapsed().as_secs_f64());
}