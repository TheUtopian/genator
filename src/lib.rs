use std::ops::Range;
use std::fs::File;

type BString = Vec<u8>;

const MAX_COMBINATION: usize = 1 << 20;

#[derive(Debug, Clone, PartialEq)]
enum Token<'a> {
	Ch(u8),
	Str(&'a str),
}

use Token::{Ch, Str};

#[derive(Debug)]
pub struct Parser<'a> {
	map: Vec<Vec<Token<'a>>>,
	template: BString
}

impl<'a> Parser<'a> {
	pub fn new(data: &'a str) -> Self {
		let (tokens, template) = Parser::tokenize(data);
		let mut map: Vec<Vec<Token<'a>>> = Vec::with_capacity(tokens.len());

		for (token, alpha) in tokens.iter() {
			let bracket = token.as_bytes()[0];
			let mut piece = Vec::new();

			if *alpha { piece.push(Ch(0)); }

			if bracket == b'[' {
				let data = token[1..].as_bytes();
				
				let mut i = 0;
				while i < data.len() {
					if data[i] == b'-' && i > 0 && i < data.len() - 1 {
						if data[i - 1].is_ascii_alphanumeric() && data[i + 1].is_ascii_alphanumeric() {
							for t in data[i - 1]..=data[i + 1] {
								if !piece.contains(&Ch(t)) {
									piece.push(Ch(t));
								}
							}
							i += 1;
						}
					} else if data.get(i + 1).copied().unwrap_or(0) != b'-' {
						if !piece.contains(&Ch(data[i])) {
							piece.push(Ch(data[i]));
						}
					}

					i += 1;
				}
			} else if bracket == b'(' {
				for t in token[1..].split('|') {
					piece.push(Str(t));
				}
			}

			map.push(piece);
		}

		Self { map, template }
	}

	pub fn iter(&self) -> Iter {
		Iter { parser: self, count: vec![0; self.map.len()] }
	}

	fn tokenize(data: &'a str) -> (Vec<(&'a str, bool)>, BString) {
		let mut template = Vec::new();
		let mut tokens: Vec<(&str, bool)> = Vec::new();

		let mut it = 0;
		while it < data.len() {
			let shift = &data[it..];

			match data.as_bytes()[it] {
				b'[' => if let Some(end_b) = shift.find(']') {
					if let Some(end) = shift[0..end_b].find(';') {
						let rng = range::from_str(&shift[(end + 1)..end_b]);
						let shift2 = &shift[0..end];

						if rng.end == 0 || rng.start > rng.end {
							template.push(0);
							tokens.push((shift2, true));
						} else {
							for _ in 0..rng.start {
								template.push(0);
								tokens.push((shift2, false));
							}
							for _ in rng {
								template.push(0);
								tokens.push((shift2, true));
							}
						}
					} else {
						template.push(0);
						tokens.push((&shift[0..end_b], false));
					}

					it += end_b;
				},
				b'(' => if let Some(end) = shift.find(')') {
					template.push(0);
					tokens.push((&shift[0..end], !shift[0..end].contains('|')));
					it += end;
				},
				_ => template.push(data.as_bytes()[it]),
			}

			it += 1;
		}

		(tokens, template)
	}
}

#[derive(Debug)]
pub struct Iter<'a> {
	parser: &'a Parser<'a>,
	count: BString
}

impl<'a> Iter<'a> {
	pub fn next(&mut self) -> Option<String> {
		let mut it = 0;
		let mut out = String::new();
		for t in self.parser.template.iter() {
			if *t == 0 {
				match self.parser.map[it][self.count[it] as usize] {
					Ch(x) if x != 0 => { out.push(x as char) },
					Str(x) => {	out.push_str(x) },
					_ => {}
				}
				it += 1;
			} else {
				out.push(*t as char);
			}
		}

		for (mask, it) in self.parser.map.iter().zip(self.count.iter_mut()) {
			if *it as usize + 1 < mask.len() {
				*it += 1;
				return Some(out);
			}
			*it = 0;
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