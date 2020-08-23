$env:CARGO_INCREMENTAL=0
$env:RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort"
$env:RUSTDOCFLAGS="-Cpanic=abort"

cargo +nightly build

cargo test

grcov ./target/debug/ -s . -t coveralls+ --llvm --branch --ignore-not-existing -o ./target/debug/coverage/


pause