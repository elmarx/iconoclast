ci:
    cargo fmt --all -- --check
    cargo clippy --all-targets --all-features -- -D warnings

    cargo check -p iconoclast --all-features
    cargo check -p iconoclast --no-default-features
    cargo check --all-targets --all-features

    cargo nextest run --all-targets --no-default-features
    cargo nextest run --all-targets --all-features

    cargo clippy -p iconoclast --all-targets -- -D clippy::pedantic
    cargo clippy -p uservice --all-targets -- -W clippy::pedantic
