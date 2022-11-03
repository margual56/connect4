#!/bin/sh

CARGO_DIR=$HOME/.cargo/bin

echo Compiling linux gnu
$CARGO_DIR/cross build --target x86_64-unknown-linux-gnu --release

echo Compiling linux musl
$CARGO_DIR/cross build --target x86_64-unknown-linux-musl --release

echo Compiling windows
$CARGO_DIR/cross build --target x86_64-pc-windows-gnu --release

echo Compiling android
$CARGO_DIR/cross build --target aarch64-linux-android --release

