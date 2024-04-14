#!/bin/bash

mkdir -p ./bin/$(uname -m)

cargo build --release

cp ./target/release/generate-local-changelog ./bin/$(uname -m)/generate-local-changelog
