
### En phase de développement, il est courant de forcer Clippy à bloquer la compilation en cas d'avertissement (très utile dans une CI) :
* $ cargo clippy -- -D warnings

### Pour être plus strict et vérifier l'ensemble des cibles (tests, exemples, benchmarks), utilisez :
* $ cargo clippy --all-targets --all-features

###
* $ cargo fmt --check
* $ cargo clippy --all-targets --all-features -- -D warnings
* $ cargo test
* $ cargo test resolve_and_validate_destination

/// project interne
cargo run -- --help
cargo run -- axum-forge --help
cargo run -- axum-forge new --help


// after push on github
cargo fmt --all
cargo test --all-features
cargo clippy --all-targets --all-features -- -D warnings
cargo package --list
cargo package