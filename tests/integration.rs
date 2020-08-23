use assert_cmd::Command;

#[test]
fn main() {
	let exp = "\
\"Then there’s a pair of us - don’t tell!\"
\"They’d banish us, you know.\"
";
	
	Command::cargo_bin("minigrep")
		.unwrap()
		.args(&["us", "stubs/poem.txt"])
		.assert()
		.stdout(exp);
}
