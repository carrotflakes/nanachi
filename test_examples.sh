set -e

cargo build --release --example composite_test
cargo build --release --example path3
cargo build --release --example k_curve
cargo build --release --example nanachi
cargo build --release --example context
cargo build --release --example path_data_notation --features "path-data-notation"
cargo build --release --example new_nanachi --features "path-data-notation"

echo "All tests passed! 🍰😃"
