use std::{env, fs, process};
use minigrep::{Config, find_strings};

fn main() {
	let args: Vec<String> = env::args().collect();
	
	let cfg = Config::new(&args)
		.unwrap_or_else(|err| {
		eprintln!("Problem parsing arguments: {}", err);
		process::exit(1);
	});
	
	let contents = fs::read_to_string(cfg.filename)
		.expect("Something went wrong reading the file");
	
	find_strings(&contents, &cfg.query)
	.iter().for_each(|line| println!("{:#?}", line));
}

