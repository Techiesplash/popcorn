//
// Created by techiesplash on 7/19/23.
//

// This file is used to handle the system calls... Sadly.
// Because Rust likes to restore assembly registers, we have to use C, which does not.
// This is a workaround for that.

// So we jam this into the IDT, and use it to wrap the Rust syscall handler.

typedef unsigned long long int uint64_t;
static unsigned long long int (*handler_syscall)(uint64_t, uint64_t, uint64_t, uint64_t, uint64_t, uint64_t) = 0;

void syscall()
{
    // Get rax, rbx, rcx, rdx, rsi, rdi
   // uint64_t rax, rbx, rcx, rdx, rsi, rdi;
    /*asm volatile(
            "movq %%rax, %[out_rax]\n"
            "movq %%rbx, %[out_rbx]\n"
            "movq %%rcx, %[out_rcx]\n"
            "movq %%rdx, %[out_rdx]\n"
            "movq %%rsi, %[out_rsi]\n"
            "movq %%rdi, %[out_rdi]\n"

            : [out_rax] "=r" (rax), [out_rbx] "=r" (rbx), [out_rcx] "=r" (rcx), [out_rdx] "=r" (rdx), [out_rsi] "=r" (rsi), [out_rdi] "=r" (rdi)
    :
    : "r8", "r9", "r10", "r11", "r12", "r13" // Use extended registers instead
    );*/
   // uint64_t result = handler_syscall(rax, rbx, rcx, rdx, rsi, rdi);

    asm volatile("mov $42, %%rax" : : : "r9");
}

void set_syscall_handler(unsigned long long int (*handler)(void))
{
    handler_syscall = handler;
}