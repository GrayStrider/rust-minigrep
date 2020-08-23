use std::{env, fs, process};
use minigrep::Config;

fn main() {
	let args: Vec<String> = env::args().collect();
	
	let cfg = Config::new(&args)
		.unwrap_or_else(|err| {
		eprintln!("Problem parsing arguments: {}", err);
		process::exit(1);
	});
	
	let contents = fs::read_to_string(cfg.filename)
		.expect("Something went wrong reading the file");
	
	let res = find_strings(&contents, &cfg.query);
	res.iter().for_each(|line| println!("{:#?}", line));
}

fn find_strings<'a>(text: &'a str, slice: &'a str) -> Vec<&'a str> {
	text.lines().filter(|line| line
		.contains(slice))
		.collect::<Vec<_>>()
}
