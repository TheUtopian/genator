use std::ops::Range;
use std::fs::File;

const MAX_COMBINATION: usize = 1 << 20;

#[derive(Debug, Clone, PartialEq)]
enum Token<'a> {
	Ch(Vec<u8>),
	Str(Vec<&'a str>),
	Out(&'a str),
	Var(usize),
}

impl Token<'_> {
	fn len(&self) -> usize {
		match self {
			Ch(x) => x.len(),
			Str(x) => x.len(),
			Out(_) => 1,
			Var(_) => 0,
		}
	}
}

use Token::{Ch, Str, Out, Var};

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
				let mut piece: Vec<&str> = token[1..].split('|').collect();

				if *alpha { piece.push(""); }

				map.push(Str(piece));
			} else if bracket == b'{' {
				let num = (token.as_bytes()[1] - b'0') as usize;
				if num >= 0 && num < map.len() {
					map.push(Var(num));
				}
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
						match range::from_str(&shift[(end + 1)..end_b]) {
							Ok(rng) => {
								for _ in 0..rng.start {
									tokens.push((&shift[..end], false));
								}

								for _ in rng {
									tokens.push((&shift[..end], true));
								}
							},
							Err(_) => {
								let num: u8 = shift[(end + 1)..end_b].parse().unwrap_or(1);

								if num == 0 {
									tokens.push((&shift[..end], true));
								} else {
									for _ in 0..num {
										tokens.push((&shift[..end], false));
									}
								}
							},
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
				b'{' => if let Some(end) = shift.find('}') {
					tokens.push((&shift[..end], false));
					it += end;
				},
				_ => {
					if let Some(end) = shift.find(|x| x == '[' || x == '(' || x == '{') {
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
	count: Vec<u8>
}

impl Iter<'_> {
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

impl Iterator for Iter<'_> {
	type Item = String;

	fn next(&mut self) -> Option<Self::Item> {
		if self.count[0] == u8::MAX {
			return None;
		}

		let mut out = String::new();

		for (mask, i) in self.parser.map.iter().zip(self.count.iter()) {
			match mask {
				Ch(x) => if x[*i as usize] != 0 { out.push(x[*i as usize] as char) },
				Str(x) => {	out.push_str(x[*i as usize]) },
				Out(x) => { out.push_str(x) },
				Var(n) => {
					match &self.parser.map[*n] {
						Ch(x) => if x[self.count[*n] as usize] != 0 { out.push(x[self.count[*n] as usize] as char) },
						Str(x) => {	out.push_str(x[self.count[*n] as usize]) },
						_ => {},
					}
				}
			}
		}

		for (mask, i) in self.parser.map.iter().zip(self.count.iter_mut()) {
			if *i + 1 < mask.len() as u8 {
				*i += 1;
				return Some(out);
			}
			*i = 0;
		}

		self.count[0] = u8::MAX;

		Some(out)
	}
}

mod range {
	use std::ops::Range;

	pub fn from_str(s: &str) -> Result<Range<u8>, ()> {
		if let Some(separator) = s.find('-') {
			if let Ok(start) = s[..separator].trim().parse::<u8>() {
				if let Ok(end) = s[(separator + 1)..].parse::<u8>() {
					return Ok(Range { start, end });
				}
			}
		}

		Err(())
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