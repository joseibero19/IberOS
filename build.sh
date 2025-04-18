#!/bin/bash

# Build and run script for iberOS

# Install bootimage if not already installed
if ! cargo install --list | grep -q "bootimage"; then
    echo "Installing bootimage tool..."
    cargo install bootimage
    rustup component add llvm-tools-preview
fi

# Build the kernel
echo "Building iberOS kernel..."
cargo build

# Create bootable disk image
echo "Creating bootable disk image..."
cargo bootimage

# Run in QEMU
echo "Running iberOS in QEMU..."
qemu-system-x86_64 -drive format=raw,file=target/x86_64-iberos/debug/bootimage-iberos.bin -serial stdio
