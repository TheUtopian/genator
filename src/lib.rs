use std::ops::Range;

const MAX_COMBINATION: usize = 1 << 30;


trait Combinator {
	fn combinations(&self) -> Option<u32>;
}

impl Iterator for Box<dyn Combinator> {
	type Item = String;

	fn next(&mut self) -> Option<Self::Item> {
		None
	}
}

#[derive(Debug, Clone, PartialEq)]
enum Token<'a> {
	Ch(Vec<u8>),
	Str(Vec<&'a str>),
	Out(&'a str),
}

use Token::*;

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

		for (k, token) in tokens.iter().enumerate() {
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

				if token.is_empty() {
					piece.push(0);
				}

				keys.push(k as u8);
				map.push(Ch(piece));
			} else if bracket == b'(' {
				let mut piece: Vec<&str> = token[1..].split('|').collect();

				if token.is_empty() {
					piece.push(token);
				}

				keys.push(k as u8);
				map.push(Str(piece));
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

	fn tokenize(data: &'a str) -> Vec<&'a str> {
		let mut tokens: Vec<&str> = Vec::new();

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
									tokens.push(&shift[..end]);
								}

								for _ in rng {
									tokens.push(&shift[..end]);
								}
							},
							Err(_) => {
								let num: u8 = shift[(end + 1)..end_b].parse().unwrap_or(1);

								if num == 0 {
									tokens.push(Default::default());
								} else {
									for _ in 0..num {
										tokens.push(&shift[..end]);
									}
								}
							},
						}
					} else {
						tokens.push(&shift[..end_b]);
					}

					i += end_b;
				},
				b'(' => if let Some(end) = shift.find(')') {
					if end > 1 {
						let slice = &shift[..end];

						if slice.contains(',') {
							tokens.push(slice);
						} else {
							tokens.push(Default::default());
						}
					}
					i += end;
				},
				_ => {
					if let Some(end) = shift.find(|x| x == '[' || x == '(') {
						tokens.push(&shift[..end]);
						i += end - 1;
					} else {
						tokens.push(shift);
						break;
					}
				},
			}

			i += 1;
		}

		tokens
	}

	fn lexems(data: &str) -> Vec<&str> {
		let vec = Vec::new();
		let open_brackets = [b'(', b'['];
		let brackets = [b'(', b')', b'[', b']'];

		let mut state = None;
		for (i, b) in data.bytes().enumerate() {
			match state {
				Some(s) => {
				},
				None => {
					if (open_brackets.contains(&b)) {
						state = Some(b);
					}
				}
			}
		}
		vec
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

pub fn request(mut args: impl Iterator<Item = String>) -> Result<String, &'static str> {
	args.next();

	match args.next() {
		Some(req) if !req.is_empty() && req.is_ascii() => Ok(req),
		_ => Err("Write something and don't forget about quotes please.")
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