ci:
    cargo fmt --all -- --check
    cargo clippy -- -D warnings

    cargo check --all-targets --no-default-features
    cargo nextest run --all-targets --no-default-features

    cargo check --all-targets --all-features
    cargo nextest run --all-targets --all-features

    cargo clippy --all-targets -- -W clippy::pedantic

    # finally, build the docker image to ensure it works, too
    docker build .
