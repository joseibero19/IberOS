#![no_std] // Don't use the standard library
#![no_main] // Don't use the standard entry point chain
#![feature(alloc_error_handler)] // For handling allocation errors

extern crate alloc;

use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};
use crate::vga_buffer::Color;

mod vga_buffer;
mod ipc;
mod allocator;
mod string;

// Simple function to pause execution for a while
pub fn pause_execution(iterations: u64) {
    // Simple busy-wait loop with a much smaller multiplier
    // This should be more reasonable for QEMU
    for _ in 0..iterations * 10_000_000 {
        // This is just a busy-wait, doing nothing
        core::hint::spin_loop();
    }
}

entry_point!(kernel_main);

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("[KERNEL PANIC] {}", info);
    loop {}
}

fn kernel_main(_boot_info: &'static BootInfo) -> ! {
    // Initialize the kernel
    vga_buffer::clear_screen();
    println!("Initializing iberOS kernel...");

    // Very short pause to ensure message is visible
    pause_execution(1);

    // SECTION 1: Display the logo
    vga_buffer::clear_screen();
    vga_buffer::print_logo();
    println!("\n\nWelcome to iberOS - A Rust Microkernel");
    println!("----------------------------------");

    // Short pause for the logo
    pause_execution(3);

    // SECTION 2: Demonstrate IPC
    ipc::demo_ipc();

    // SECTION 3: Final screen
    vga_buffer::clear_screen();
    vga_buffer::print_centered("System initialized successfully!", Color::LightGreen);
    vga_buffer::print_centered("----------------------------------", Color::LightGray);
    println!("\n\n");
    vga_buffer::print_centered("[System paused - Press Ctrl+C to exit]", Color::Yellow);

    // Infinite loop to keep the system running
    loop {}
}
