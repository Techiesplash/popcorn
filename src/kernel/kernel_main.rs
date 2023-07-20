use core::arch::asm;
use crate::{println, system};

pub fn sys_exit(_args: system::syscall::SyscallArgs) -> u64 {
    println!("'exit' syscall called, with args: {} {}", _args.arg_a, _args.arg_b);
    0
}

pub fn sys_fallback(_args: system::syscall::SyscallArgs) -> u64 {
    println!("A syscall was called, but no handler was registered for it. Arg a: {}", _args.arg_a);
    0
}

/// @brief The main function of the kernel
pub fn main() {

    // The heap should be initialized by now, along with everything we need to get started.
    println!("Hello, x86!");

    let mut st = system::syscall::SyscallTableDescriptor::new();
    st.register_syscall(1, sys_exit);
    st.register_fallback(sys_fallback);
    system::syscall::set_table(st);

    unsafe
        {
            asm!(
            "mov rax, 1",
            "mov rbx, 42",
            "mov rcx, 21",
            "int 0x80",
            );

            asm!(
            "mov rax, 5",
            "mov rbx, 42",
            "int 0x80",
            );
        }
    system::task::hlt_loop();
}