use assert_cmd::Command;

#[test]
fn main() {
	let exp =
		"\"Then there’s a pair of us - don’t tell!\"\n\"They’d banish us, you know.\"\n";
	
	Command::cargo_bin("minigrep")
		.unwrap()
		.args(&["us", "stubs/poem.txt"])
		.assert()
		.stdout(exp);
}
