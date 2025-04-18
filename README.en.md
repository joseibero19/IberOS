# IberOS ✨

**A next-generation operating system focused on security, performance, and artificial intelligence, built on a modern microkernel in Rust.**

![IberOS Logo](docs/logo.png)

## Vision

IberOS aspires to be a robust, secure, and adaptable platform for general users, professionals, and researchers. We believe in the need for an operating system built from the ground up with the principles of security by design, modularity, and intelligent AI integration, leveraging the power and safety of Rust and a microkernel architecture. We want to build an open and collaborative community around this project.

## Current Project Status ⚠️

**Attention! IberOS is in a VERY EARLY stage of development (Pre-Alpha / Proof of Concept).**

We are currently working on the **initial Proof of Concept (PoC):**

* A **minimalist** microkernel written in **Rust**.
* Capable of **booting** in the **QEMU** emulator (x86_64 target).
* Basic hardware initialization (VGA console for printing messages).
* Simple Inter-Process Communication (IPC) demonstration between simulated processes.
* Custom welcome message and logo.

**THIS PROJECT IS NOT USABLE FOR DAILY TASKS AND IS HIGHLY EXPERIMENTAL.**

## PoC Objectives

* Validate the basic microkernel architecture.
* Demonstrate the ability to boot Rust code on bare-metal (virtualized).
* Establish the build and execution environment (`no_std`, QEMU).
* Implement a basic IPC demonstration.
* Serve as a foundation to attract future collaborators and interest.

## Technology Stack (Current and Planned)

* **Main Language (Kernel & Critical Services):** Rust
* **Kernel Architecture:** Microkernel
* **User Interface (Future):** C++ with Qt (Planned, subject to change)
* **Other Languages (Services, AI, Apps):** C++, Python, Go (Potentially)
* **Initial Development/Testing Platform:** QEMU (x86_64)

## Implemented Features

* Basic boot in QEMU
* VGA text output
* Custom IberOS logo
* Basic interrupt handling system
* IPC (Inter-Process Communication) simulation
* Basic memory implementation

## How to Compile and Run

### Prerequisites

* Rust (nightly) - `rustup default nightly`
* QEMU - For Windows, download from [qemu.org](https://www.qemu.org/download/) or install with `choco install qemu`
* cargo-binutils - `cargo install cargo-binutils`
* bootimage - `cargo install bootimage`
* llvm-tools-preview - `rustup component add llvm-tools-preview`

### Steps to Compile and Run

```bash
# 1. Clone the repository
git clone https://github.com/joseibero19/IberOS.git
cd IberOS

# 2. Compile and run (using the included script)
.\build.ps1  # On Windows
# or
./build.sh   # On Linux/macOS (when available)
```

Alternatively, you can run the commands manually:

```bash
# Compile the kernel
cargo build

# Create the boot image
cargo bootimage

# Run in QEMU
qemu-system-x86_64 -drive format=raw,file=target/x86_64-iberos/debug/bootimage-iberos.bin
```

## Project Structure

```
IberOS/
├── src/                    # Kernel source code
│   ├── main.rs             # Kernel entry point
│   ├── vga_buffer.rs       # VGA screen driver
│   ├── ipc.rs              # IPC implementation
│   ├── allocator.rs        # Memory allocator
│   └── string.rs           # Basic string implementation
├── Cargo.toml              # Rust project configuration
├── Cargo.lock              # Locked dependency versions
├── build.ps1               # Windows build script
├── .cargo/                 # Cargo-specific configuration
│   └── config.toml         # Cross-compilation configuration
└── README.md               # This file
```

## Roadmap

### Phase 1 (Current - PoC)
- [x] Basic kernel that boots in QEMU
- [x] Basic text output
- [x] IPC simulation

### Phase 2 (Next)
- [ ] Implement a more robust memory system
- [ ] Add support for hardware interrupts
- [ ] Implement a basic scheduler

### Phase 3 (Future)
- [ ] Add support for basic drivers
- [ ] Implement a simple file system
- [ ] Improve inter-process communication

### Phase 4 (Long Term)
- [ ] Add support for multiple architectures
- [ ] Implement basic networking capabilities
- [ ] Create a simple user interface

## Contributing

Contributions are welcome! If you're interested in contributing to IberOS, please:

1. Check open issues or create a new one to discuss what you'd like to change.
2. Fork the repository.
3. Create a branch for your feature (`git checkout -b feature/amazing-feature`).
4. Make your changes and commit them (`git commit -m 'Add some amazing feature'`).
5. Push to the branch (`git push origin feature/amazing-feature`).
6. Open a Pull Request.

## License

This project is licensed under the terms of the Apache 2.0 license.
See the [LICENSE](LICENSE) file for more details.

## Contact

Jose Ibero - [ibersoft96@gmail.com](mailto:ibersoft96@gmail.com)

Project Link: [https://github.com/joseibero19/IberOS](https://github.com/joseibero19/IberOS)

## Acknowledgments

* [Philipp Oppermann](https://os.phil-opp.com/) for his excellent "Writing an OS in Rust" series
* The Rust community for their tools and libraries
* All contributors and people who have shown interest in this project
