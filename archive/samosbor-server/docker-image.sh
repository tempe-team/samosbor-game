#!/bin/sh
cargo build --release
docker build -t $1 .
