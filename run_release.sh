#!/bin/bash

cargo build --release
cp ./target/release/rust-zmq .
./rust-zmq
