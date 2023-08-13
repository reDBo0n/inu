use std::env;

fn main() {
	let mut args = env::args().peekable();

	if args.len() < 2 {
		println!("No filenames supplied");
		return;
	}

	// skip own file name
	args.next();

	let opts: inu::Options = {
		let mut opts = inu::Options {
			n: false,
			e: false,
		};
		// safe to do, because we checked arg len above
		let tmp = args.peek().unwrap();
		if tmp.len() == 2 || tmp.len() == 3 {
			let mut it = tmp.chars();
			if it.next().unwrap() == '-' {
				for o in it {
					match o {
						'n' => opts.n = true,
						'e' => opts.e = true,
						_ => {
							println!("Unknown Option");
							return;
						}
					}
				}

				// consume the options from args
				args.next();
			}
		}

		opts
	};

	// run inu for each supplied file (if possible)
	for i in args {
		inu::inu(&i, &opts);
	}
}
