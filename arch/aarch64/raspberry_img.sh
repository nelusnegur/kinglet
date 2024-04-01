#! /bin/bash

set -euo pipefail

KERNEL_IMAGE=kernel8.img

echo "Build the kernel"
cargo build --release

echo "Create kernel image"
rust-objcopy -O binary ./../../target/aarch64-unknown-none/release/kinglet_aarch64 $KERNEL_IMAGE
echo "Done!"
