set -e

# Cargo.
flutter pub get
cargo fmt -- --check
cargo test
cargo build

# Flutter.
dart analyze
flutter test
