set -e
cargo build --release --no-default-features
strip target/release/mandelwow

(
	echo "---------------------------------"
	date
	git branch -v
	rustc --version
	ls -la target/release/mandelwow
	size --format=sysv target/release/mandelwow
) | tee -a size.log

upx -9 target/release/mandelwow
