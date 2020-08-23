use std::error::Error;
use std::fs;

#[derive(Debug, PartialEq)]
pub struct Config {
	pub query: String,
	pub filename: String,
}

impl Config {
	pub fn new(args: &[String]) -> Result<Config, &str> {
		if args.len() < 3 {
			return Err("not enough arguments");
		}
		
		let query = args[1].clone();
		let filename = args[2].clone();
		
		Ok(Config { query, filename })
	}
}

pub fn find_strings<'a>(text: &'a str, slice: &'a str) -> Vec<&'a str> {
	text.lines().filter(|line| line
		.contains(slice))
	    .collect()
}


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
	let contents = fs::read_to_string(config.filename)?;
	
	find_strings(&contents, &config.query)
		.iter().for_each(|line| println!("{:#?}", line));
	
	Ok(())
}


#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn should_read_args() {
		let args: Vec<String> = ["\\stubs\\poem.txt", "hello", "234"]
			.iter().map(|w| w.to_string())
			.collect();
		let act = Config::new(args.as_ref());
		let exp = Config::new(&args);
		assert_eq!(act, exp);
	}
}
