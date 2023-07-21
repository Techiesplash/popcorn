// Use this file for exporting functions from the System directory.

use x86_64::VirtAddr;
use crate::system::memory::{BootInfoFrameAllocator, init_pagetable};

pub mod allocation;
pub mod gdt;
pub mod interrupt_handlers;
pub mod interrupts;
pub mod memory;
pub mod panic;
pub mod power;
pub mod serial;
pub mod vga_buffer;
pub mod task;
pub mod syscall;
pub mod vga_video;


/// @brief Initializes the system's hardware, such as the GDT, IDT, etc.
pub fn init_system(boot_info: &'static bootloader::BootInfo)
{
    // Interrupts come first.
    interrupts::init_interrupts();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { init_pagetable(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocation::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    //   vga_video::init_vga();
}

/// @brief Make sure the system is properly set up.
pub fn validate_system() -> bool
{
    // TODO: Add more validation checks
    true
}