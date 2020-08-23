use assert_cmd::Command;

#[test]
fn main() {
	let exp =
		"\"Then there’s a pair of us - don’t tell!\"\n\"They’d banish us, you know.\"\n";
	
	
	let mut cmd = Command::cargo_bin("minigrep").unwrap();
	let assert = cmd
		.args(&["us", "stubs/poem.txt"])
		.assert();
	assert
		.stdout(exp);
}
