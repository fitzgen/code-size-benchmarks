#!/usr/bin/env bash

set -eu
cd "$(dirname $0)"

# When running in Travis CI, log a bunch of stuff to console so that we can see
# progress and don't get shutdown for the build hanging.
if [[ "$CI" != "" ]]; then
    set -x
fi

function quiet {
    local stdout_log=$(mktemp stdout-XXXXXX)
    local stderr_log=$(mktemp stdout-XXXXXX)
    $@ 2>"$stderr_log" >"$stdout_log" || {
        local code=$?
        echo "command failed: $@"
        echo "stdout ==="
        cat "$stdout_log"
        echo "stderr ==="
        cat "$stderr_log"
        rm "$stdout_log"
        rm "$stderr_log"
        return "$code"
    }
    rm "$stdout_log"
    rm "$stderr_log"
}

# Ensure that the submodules are initialized and checked out.
quiet git submodule update --init

# Ensure we have the latest wasm32-unknown-unknown compiler.
quiet rustup toolchain add nightly
quiet rustup update nightly
quiet rustup target add wasm32-unknown-unknown --toolchain nightly

# Ensure we have `wasm-gc` installed.
which wasm-gc > /dev/null || quiet cargo +nightly install wasm-gc

# Remove any old `.wasm` files.
quiet rm -f ./target/wasm32-unknown-unknown/release/*.wasm

# Build every crate in the project.
quiet cargo +nightly build --all --release --locked --target wasm32-unknown-unknown

for w in $(ls -1 ./target/wasm32-unknown-unknown/release/ | grep '\.wasm$'); do
    quiet wasm-gc "./target/wasm32-unknown-unknown/release/$w"
done

wc -c ./target/wasm32-unknown-unknown/release/*.wasm
