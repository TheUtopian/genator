use std::env;
use std::ops::{RangeInclusive, Range};
use std::cmp::Ordering;
use std::str::from_utf8_unchecked as from_ascii;
use std::time::{Duration, Instant};
use std::fs::File;
use std::io::prelude::*;

use genator::Parser;

const MAX_COMBINATION: usize = 1 << 31;

fn main() {
	let args: Vec<String> = env::args().collect();

	if args.len() < 2 || args[1].is_empty() || !args[1].is_ansii() {
		panic!("Write something and don't forget about quotes please.");
	}

	let parser = Parser::new(&args[1]);
	let mut mp = parser.iter();

	let start = Instant::now();
	if mp.combinations() > MAX_COMBINATION {
		println!("Too much combinations: {}", mp.combinations());
		return;
	}

	loop {

		println!("{} | ", mp.get());

		if !mp.next() { break; }
	}

	println!("-----------------\nCombinations: {}\n-----------------", mp.combinations());
	println!("-----------------\nElapsed: {}\n-----------------", start.elapsed().as_secs_f64());
}