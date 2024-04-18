#!/bin/bash

cargo build 
\cp -f ./target/release/trader .
alias tm="./trader"
