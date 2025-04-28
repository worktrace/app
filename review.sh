set -e

# Cargo.
cargo fmt -- --check
cargo test
cargo build

# Flutter.
dart analyze
flutter test
