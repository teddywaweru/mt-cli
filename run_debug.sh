#!/bin/bash

cargo build 
cp ./target/debug/rust-zmq .
./rust-zmq
