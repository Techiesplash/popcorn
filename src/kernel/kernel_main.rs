use alloc::format;
use core::arch::asm;
use core::str::from_utf8;
use crate::{printf, sprintf};
use crate::{print, println, set_color, system};
use crate::system::vga_buffer::Color;

pub fn sys_exit(_args: system::syscall::SyscallArgs) -> u64 {
    print!("sys_exit() called.\n");
    printf!(1024, "\
    EBX: 0x%X16\n\
    ECX: 0x%X16\n\
    EDX: 0x%X16\n\
    ESI: 0x%X16\n\
    EDI: 0x%X16\n",
    _args.arg_a,
    _args.arg_b,
    _args.arg_c,
    _args.arg_src,
    _args.arg_dest
    );
            32

}

pub fn sys_fallback(_args: system::syscall::SyscallArgs) -> u64 {
   // println!("A syscall was called, but no handler was registered for it. Arg a: {}", _args.arg_a);
    0
}



/// @brief The main function of the kernel
pub fn main() {
    let mut o: u64 = 0;
    // The heap should be initialized by now, along with everything we need to get started.
    print!("Welcome to ");
    set_color!(Color::LightCyan, Color::Black);
    print!("TRANS/Popcorn");
    set_color!(Color::White, Color::Black);
    println!("!");

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
        "mov {out}, rax",
        out = out(reg) o,
        );
    }


    // Use int formatter println
    println!("Syscall returned o={}", o);
    system::task::hlt_loop();
}