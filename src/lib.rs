use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

pub struct Options {
	pub n: bool,
	pub e: bool
}

pub fn inu(filename: &str, opts: &Options) {
	let file = File::open(filename);
	let file = match file {
		Ok(f) => f,
		Err(e) => {
			println!("Error opening file: {}", e);
			return;
		}
	};

	let mut cnt = 1;

	let reader = BufReader::new(file);
	for line in reader.lines() {
		match line {
			Ok(s) => {
				if opts.n {
					print!("{} ", cnt);
					cnt += 1;
				}
				print!("{}", s);
				if opts.e {
					print!(" $");
				}
				println!("");
			}
			Err(_e) => continue,
		}
	}
}