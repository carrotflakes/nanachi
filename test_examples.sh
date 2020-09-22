set -e

cargo run --release --example composite_test
cargo run --release --example composite_test_f32
cargo run --release --example path3
cargo run --release --example k_curve
cargo run --release --example nanachi
cargo run --release --example context
cargo run --release --example path_data_notation --features "path-data-notation"
cargo run --release --example new_nanachi --features "path-data-notation"

echo "All tests passed! 🍰😃"
