#!/bin/bash
cargo clean
cargo build --release
cp target/release/drones-attack target/drones-attack
cp -r sounds target

