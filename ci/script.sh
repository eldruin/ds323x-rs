set -exo pipefail

main() {
    export CARGO_OPTIONS="--target $TARGET"
    if [[ ! $TARGET =~ .*linux.* ]]; then
        sed -i "s/linux-embedded-hal/#linux-embedded-hal/g" Cargo.toml
        sed -i "s/embedded-hal-mock/#embedded-hal-mock/g" Cargo.toml
    fi

    if [ ! -z $FEATURES ]; then
       export CARGO_OPTIONS="$CARGO_OPTIONS --features $FEATURES"
    fi

    cargo check $CARGO_OPTIONS
    cargo build $CARGO_OPTIONS
    if [ -z $DISABLE_EXAMPLES ] && [[ $TARGET =~ .*linux.* ]]; then
        cargo build $CARGO_OPTIONS --examples
    fi
    cargo doc $CARGO_OPTIONS

    if [ -z $DISABLE_TESTS ] && [ $TRAVIS_RUST_VERSION = nightly ] && [[ $TARGET =~ .*linux.* ]]; then
        cargo test $CARGO_OPTIONS
    fi
}

main
