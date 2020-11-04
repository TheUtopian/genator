use std::ops::Range;
use std::fs::File;

const MAX_COMBINATION: usize = 1 << 30;

#[derive(Debug, Clone, PartialEq)]
enum Token<'a> {
	Ch(Vec<u8>),
	Str(Vec<&'a str>),
	Out(&'a str),
	Var(usize),
}

use Token::{Ch, Str, Out, Var};

#[derive(Debug)]
pub struct Parser<'a> {
	map: Vec<Token<'a>>,
	keys: Vec<u8>
}

impl<'a> Parser<'a> {
	pub fn new(data: &'a str) -> Self {
		let tokens = Self::tokenize(data);
		let mut map: Vec<Token<'a>> = Vec::with_capacity(tokens.len());
		let mut keys = Vec::new();

		for (k, (token, alpha)) in tokens.iter().enumerate() {
			let bracket = token.as_bytes()[0];

			if bracket == b'[' {
				let mut piece = Vec::new();
				let data = token[1..].as_bytes();
				
				let mut i = 0;
				while i < data.len() {
					if data[i] == b'-' && i > 0 && (i + 1) < data.len() {
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

				if *alpha { piece.push(Default::default()); }

				keys.push(k as u8);
				map.push(Ch(piece));
			} else if bracket == b'(' {
				let mut piece: Vec<&str> = token[1..].split('|').collect();

				if *alpha { piece.push(Default::default()); }

				keys.push(k as u8);
				map.push(Str(piece));
			} else if bracket == b'{' {
				if token.as_bytes()[1].is_ascii_digit() {
					let num = (token.as_bytes()[1] - b'0') as usize;
					if num < keys.len() {
						map.push(Var(num));
					}
				}
			} else {
				map.push(Out(token));
			}
		}

		Self { map, keys }
	}

	pub fn iter(&self) -> Iter {
		let mut count = Vec::with_capacity(self.keys.len());
		for &k in self.keys.iter() {
			match &self.map[k as usize] {
				Ch(x) => { count.push(Range{ start: 0, end: x.len() as u8 }) },
				Str(x) => { count.push(Range{ start: 0, end: x.len() as u8 }) },
				_ => {}
			}
		}
		Iter { parser: self, count }
	}

	fn tokenize(data: &'a str) -> Vec<(&'a str, bool)> {
		let mut tokens: Vec<(&str, bool)> = Vec::new();

		let mut i = 0;
		while i < data.len() {
			let shift = &data[i..];

			match shift.as_bytes()[0] {
				b'[' => if let Some(end_b) = shift.find(']') {
					if end_b == 1 {
					} else if let Some(end) = shift[..end_b].find(';') {
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

					i += end_b;
				},
				b'(' => if let Some(end) = shift.find(')') {
					if end > 1 {
						tokens.push((&shift[..end], !shift[..end].contains(',')));
					}
					i += end;
				},
				b'{' => if let Some(end) = shift.find('}') {
					if end > 1 {
						tokens.push((&shift[..end], false));
					}
					i += end;
				},
				_ => {
					if let Some(end) = shift.find(|x| x == '[' || x == '(' || x == '{') {
						tokens.push((&shift[..end], false));
						i += end - 1;
					} else {
						tokens.push((shift, false));
						break;
					}
				},
			}

			i += 1;
		}

		tokens
	}
}

#[derive(Clone, Debug)]
pub struct Iter<'a> {
	parser: &'a Parser<'a>,
	count: Vec<Range<u8>>
}

impl Iter<'_> {
	pub fn get(&self) -> String {
		let mut out = String::new();
		let mut i = 0;

		for mask in self.parser.map.iter() {
			match mask {
				Ch(x) => {
					out.push(x[self.count[i].start as usize] as char);
					i += 1;
				},
				Str(x) => {
					out.push_str(x[self.count[i].start as usize]);
					i += 1;
				},
				Var(n) => {
					let j = self.count[*n].start as usize;
					match &self.parser.map[self.parser.keys[*n] as usize] {
						Ch(x) => { out.push(x[j] as char) },
						Str(x) => {	out.push_str(x[j]) },
						_ => {},
					}
				},
				Out(x) => { out.push_str(x) },
			}
		}

		out
	}

	pub fn combs(&self) -> Option<usize> {
		let mut p = 1;

		for x in self.parser.map.iter() {
			match x {
				Ch(x) => p *= x.len(),
				Str(x) => p *= x.len(),
				_ => {}
			}

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
		if self.count.len() == 0 {
			return None;
		}
		if self.count[0].start == u8::MAX {
			self.count[0].start = 0;
			return None;
		}

		let out = self.get();

		for Range { start, end } in self.count.iter_mut() {
			if *start + 1 < *end {
				*start += 1;
				return Some(out);
			}
			*start = 0;
		}

		self.count[0].start = u8::MAX;

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