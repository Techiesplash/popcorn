#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(popcorn::testutils::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;
use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use popcorn::{serial_println, serial_print, testutils};
use popcorn::system::syscall::{SyscallTableDescriptor, SyscallArgs};
use popcorn::system::syscall;
use core::arch::asm;

pub fn sys_exit(_args: SyscallArgs) -> u64 {
    if _args.arg_a != 42 || _args.arg_b != 21 {
        panic!("Syscall did not receive the correct arguments!");
    }
    serial_println!("Syscall 1 called!");
    32
}

pub fn sys_fallback(_args: SyscallArgs) -> u64 {
    if _args.arg_a != 42 || _args.arg_b != 21 {
        panic!("Syscall fallback did not receive the correct arguments!");
    }
    serial_println!("Syscall fallback called!");
    64
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    serial_println!("Syscall test...");
    // Make sure that syscalls work.
    popcorn::system::interrupts::init_interrupts();
    let mut st = SyscallTableDescriptor::new();
    st.register_syscall(1, sys_exit);
    st.register_fallback(sys_fallback);
    popcorn::system::syscall::set_table(st);

    unsafe
        {
            let mut o: u64;
            asm!(
            "mov rax, 1",
            "mov rbx, 42",
            "mov rcx, 21",
            "int 0x80",
            "mov {out}, rax",
            out = out(reg) o,
            );
            if o != 32 {
                serial_println!("o: {}", o);
                panic!("Syscall did not return the correct value!");
            }

            let mut o: u64 = 0;
            asm!(
            "mov rax, 5",
            "mov rbx, 42",
            "mov rcx, 21",
            "int 0x80",
            "mov {out}, rax",
            out = out(reg) o,
            );
            if o != 64 {
                serial_println!("o: {}", o);
                panic!("Syscall did not return the correct value!");
            }
        }

    serial_println!("[ok]");
    testutils::exit_qemu(popcorn::testutils::QemuExitCode::Success);
    loop {}
}

lazy_static! {
    static ref TEST_IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        unsafe {
            idt.double_fault
                .set_handler_fn(test_double_fault_handler)
                .set_stack_index(popcorn::system::gdt::DOUBLE_FAULT_IST_INDEX);
        }

        idt
    };
}

pub fn init_test_idt() {
    TEST_IDT.load();
}

extern "x86-interrupt" fn test_double_fault_handler(
    _stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    serial_println!("[failed]");
    testutils::exit_qemu(popcorn::testutils::QemuExitCode::Failed);
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    popcorn::testutils::test_panic_handler(info)
}
