#!/bin/bash

cargo build 
\cp -f ./target/debug/trader .
alias tm="./trader"
