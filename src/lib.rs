use std::ops::Range;
use std::fs::File;

type BString = Vec<u8>;

const MAX_COMBINATION: usize = 1 << 20;

#[derive(Debug, Clone, PartialEq)]
enum Token<'a> {
	Ch(BString),
	Str(Vec<&'a str>),
	Out(&'a str),
}

impl Token<'_> {
	fn len(&self) -> usize {
		match self {
			Ch(x) => x.len(),
			Str(x) => x.len(),
			Out(x) => 1
		}
	}
}

use Token::{Ch, Str, Out};

#[derive(Debug)]
pub struct Parser<'a> {
	map: Vec<Token<'a>>,
}

impl<'a> Parser<'a> {
	pub fn new(data: &'a str) -> Self {
		let tokens = Self::tokenize(data);
		let mut map: Vec<Token<'a>> = Vec::with_capacity(tokens.len());

		for (token, alpha) in tokens.iter() {
			let bracket = token.as_bytes()[0];

			if bracket == b'[' {
				let mut piece = Vec::new();
				let data = token[1..].as_bytes();
				
				let mut i = 0;
				while i < data.len() {
					if data[i] == b'-' && i > 0 && i < data.len() - 1 {
						if data[i - 1].is_ascii_alphanumeric() && data[i + 1].is_ascii_alphanumeric() {
							for t in data[i - 1]..=data[i + 1] {
								if !piece.contains(&t) {
									piece.push(t);
								}
							}
							i += 1;
						}
					} else if data.get(i + 1).copied().unwrap_or(0) != b'-' {
						if !piece.contains(&data[i]) {
							piece.push(data[i]);
						}
					}

					i += 1;
				}

				if *alpha { piece.push(0); }

				map.push(Ch(piece));
			} else if bracket == b'(' {
				let mut piece = Vec::new();

				for t in token[1..].split('|') {
					piece.push(t);
				}

				if *alpha { piece.push(""); }

				map.push(Str(piece));
			} else {
				map.push(Out(token));
			}
		}

		Self { map }
	}

	pub fn iter(&self) -> Iter {
		Iter { parser: self, count: vec![0; self.map.len()] }
	}

	fn tokenize(data: &'a str) -> Vec<(&'a str, bool)> {
		let mut tokens: Vec<(&str, bool)> = Vec::new();

		let mut it = 0;
		while it < data.len() {
			let shift = &data[it..];

			match shift.as_bytes()[0] {
				b'[' => if let Some(end_b) = shift.find(']') {
					if let Some(end) = shift[..end_b].find(';') {
						let rng = range::from_str(&shift[(end + 1)..end_b]);
						let shift2 = &shift[..end];

						if rng.end == 0 || rng.start > rng.end {
							tokens.push((shift2, true));
						} else {
							for _ in 0..rng.start {
								tokens.push((shift2, false));
							}
							for _ in rng {
								tokens.push((shift2, true));
							}
						}
					} else {
						tokens.push((&shift[..end_b], false));
					}

					it += end_b;
				},
				b'(' => if let Some(end) = shift.find(')') {
					tokens.push((&shift[..end], !shift[..end].contains('|')));
					it += end;
				},
				_ => {
					if let Some(end) = shift.find(|x| x == '[' || x == '(') {
						tokens.push((&shift[..end], false));
						it += end - 1;
					} else {
						tokens.push((shift, false));
						it = data.len();
					}
				},
			}

			it += 1;
		}

		tokens
	}
}

#[derive(Debug)]
pub struct Iter<'a> {
	parser: &'a Parser<'a>,
	count: BString
}

impl<'a> Iter<'a> {
	// FIX: DOSEN'T ITERATE THE LAST ELEMENT
	pub fn next(&mut self) -> Option<String> {
		let mut out = String::new();

		for (mask, i) in self.parser.map.iter().zip(self.count.iter()) {
			match mask {
				Ch(x) => if x[*i as usize] != 0 { out.push(x[*i as usize] as char) },
				Str(x) => {	out.push_str(x[*i as usize]) },
				Out(x) => { out.push_str(x) }
			}
		}

		for (mask, i) in self.parser.map.iter().zip(self.count.iter_mut()) {
			if *i as usize + 1 < mask.len() {
				*i += 1;
				return Some(out);
			}
			*i = 0;
		}

		None
	}

	pub fn combs(&self) -> Option<usize> {
		let mut p = 1;

		for x in self.parser.map.iter() {
			p *= x.len();

			if p > MAX_COMBINATION {
				return None;
			}
		}

		Some(p)
	}
}

mod range {
	use std::ops::Range;

	pub fn from_str(s: &str) -> Range<u8> {
		let mut iter = s.trim().split('-');

		if let Some(n) = iter.next() {
			if let Ok(start) = n.parse::<u8>() {
				if let Some(n) = iter.next() {
					if let Ok(end) = n.parse::<u8>() {
						return Range { start, end };
					}
				}

				return Range { start, end: start };
			}
		}

		Range { start: 1, end: 1 }
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn range_test() {
	}

	#[test]
	fn parser_a() {
		
	}
}