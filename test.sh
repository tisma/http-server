#!/bin/sh

RUSTFLAGS="-C instrument-coverage" cargo test --tests
grcov . -s . --binary-path ./target/debug/ -t html --branch -o ./target/debug/coverage

path=$(pwd)
echo "Openning in browser! `open file://$path/target/debug/coverage/index.html`"