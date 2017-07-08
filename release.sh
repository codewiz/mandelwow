set -e
cargo build --release --no-default-features
strip target/release/mandelwow
upx -9 target/release/mandelwow
