# Build and run script for iberOS

# Install bootimage if not already installed
if (-not (cargo install --list | Select-String -Pattern "bootimage")) {
    Write-Host "Installing bootimage tool..."
    cargo install bootimage
    rustup component add llvm-tools-preview
}

# Build the kernel
Write-Host "Building iberOS kernel..."
cargo build

# Create bootable disk image
Write-Host "Creating bootable disk image..."
cargo bootimage

# Run in QEMU
Write-Host "Running iberOS in QEMU..."
& "C:\Program Files\qemu\qemu-system-x86_64.exe" "-drive" "format=raw,file=target/x86_64-iberos/debug/bootimage-iberos.bin" "-serial" "stdio"
