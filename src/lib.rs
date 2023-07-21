#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![test_runner(crate::testutils::test_runner)]
#![feature(panic_info_message)]
#![feature(fmt_internals)]
#![feature(fn_traits)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use bootloader::{BootInfo, entry_point};
use crate::system::task::hlt_loop;
use crate::system::vga_buffer::Color;
use crate::testutils::exit_qemu;
use crate::testutils::QemuExitCode::Success;

pub mod system;
pub mod testutils;
pub mod kernel;
mod utils;


/**
 * @brief Initializes the kernel.
 * @details This function initializes the kernel. Call this function before doing anything else.
 * To be used in Main, and to be used in Tests.
 */
pub fn init(boot_info: &'static BootInfo) {
    system::init_system(boot_info);
}

/**
 * @brief Shuts down the kernel.
 * @details This function clears the screen, then shuts down the kernel, then shuts down the computer.
 * You should be able to call this anywhere if needed.
 */
pub fn shutdown() {
    clear_screen!(Color::Black);
    set_color!(Color::White, Color::Black);
    println!("Shutting down...");

    // Put deinitialization code here.

    // Stop processor
    system::task::hlt_loop();
}

#[cfg(test)]
entry_point!(test_kernel_main);

/// Entry point for `cargo xtest`
#[cfg(test)]
fn test_kernel_main(_boot_info: &'static BootInfo) -> ! {
    test_main();
    exit_qemu(Success);
    hlt_loop();
}
