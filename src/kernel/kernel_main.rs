use core::arch::asm;

use crate::{print, println, set_color, system};
use crate::system::vga_buffer::Color;

pub fn sys_exit(_args: system::syscall::SyscallArgs) -> u64 {
    println!("'exit' syscall called, with args: {} {}", _args.arg_a, _args.arg_b);
    // Grab rax, rbx, rcx, rdx, rsi, rdi and println
    let mut rax: u64 = 0;
    let mut rbx: u64 = 0;
    let mut rcx: u64 = 0;
    let mut rdx: u64 = 0;
    let mut rsi: u64 = 0;
    let mut rdi: u64 = 0;

    unsafe
        {
            asm!(
            "mov {out_rax}, rax",
            "mov {out_rbx}, rbx",
            "mov {out_rcx}, rcx",
            "mov {out_rdx}, rdx",
            "mov {out_rsi}, rsi",
            "mov {out_rdi}, rdi",
            out_rax = out(reg) rax,
            out_rbx = out(reg) rbx,
            out_rcx = out(reg) rcx,
            out_rdx = out(reg) rdx,
            out_rsi = out(reg) rsi,
            out_rdi = out(reg) rdi,
            );
            println!("rax: {}, rbx: {}, rcx: {}, rdx: {}, rsi: {}, rdi: {}", rax, rbx, rcx, rdx, rsi, rdi);


            32
        }
}

pub fn sys_fallback(_args: system::syscall::SyscallArgs) -> u64 {
    println!("A syscall was called, but no handler was registered for it. Arg a: {}", _args.arg_a);
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