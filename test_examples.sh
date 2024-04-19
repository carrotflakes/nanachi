set -e

cargo run --release --example basic
cargo run --release --example composite_test
cargo run --release --example composite_test_f32
cargo run --release --example composite_test_premultiplied_f32
cargo run --release --example conic_grad
cargo run --release --example fast_gauss_blur
cargo run --release --example pseudo_blur
cargo run --release --example path
cargo run --release --example k_curve
cargo run --release --example context
cargo run --release --example path_data_notation
cargo run --release --example nanachi

echo "All tests passed! 🍰😃"
