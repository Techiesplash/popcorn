use lazy_static::lazy_static;
use spin::Mutex;
use crate::kernel::kernel_main::sys_fallback;

/* INTENDED USAGE:
pub fn sys_exit(_args: system::syscall::SyscallArgs) -> u64 {
    println!("'exit' syscall called, with args: {} {}", _args.arg_a, _args.arg_b);
    0
}

pub fn sys_fallback(_args: system::syscall::SyscallArgs) -> u64 {
    println!("A syscall was called, but no handler was registered for it. Arg a: {}", _args.arg_a);
    0
}

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
 */

/// @brief The arguments passed to a syscall
/// @details This struct contains the arguments passed to a syscall, through the registers.
pub struct SyscallArgs {
    pub arg_a: u64,
    pub arg_b: u64,
    pub arg_c: u64,
    pub arg_src: u64,
    pub arg_dest: u64,
}

impl SyscallArgs {
    pub fn new(arg_a: u64, arg_b: u64, arg_c: u64, arg_src: u64, arg_dest: u64) -> SyscallArgs {
        SyscallArgs {
            arg_a,
            arg_b,
            arg_c,
            arg_src,
            arg_dest,
        }
    }
}

/// @brief A function signature for a syscall handler
pub type SyscallHandler = fn(SyscallArgs) -> u64;

/// @brief A descriptor for a syscall table.
/// @details This struct contains a syscall table, and a fallback handler.
/// The fallback handler is called when a syscall is called, but no handler is registered for it.
pub struct SyscallTableDescriptor {
    pub syscall_table: [SyscallHandler; 256],
    pub syscall_fallback: SyscallHandler,
}

impl SyscallTableDescriptor {
    pub fn new() -> SyscallTableDescriptor {
        let mut x = SyscallTableDescriptor {
            syscall_table: [default_syscall_handler; 256],
            syscall_fallback: default_syscall_handler,
        };

        for i in 0..256 {
            x.syscall_table[i] = sys_fallback;
        }
        x
    }

    /// @brief Clone the syscall table descriptor, creating a new one with the same contents.
    pub fn clone(&self) -> SyscallTableDescriptor {
        SyscallTableDescriptor {
            syscall_table: self.syscall_table,
            syscall_fallback: self.syscall_fallback,
        }
    }

    /// @brief Register a syscall handler for a given syscall number
    pub fn register_syscall(&mut self, syscall_num: u8, handler: SyscallHandler) {
        self.syscall_table[syscall_num as usize] = handler;
    }

    /// @brief Register a fallback handler for all syscalls
    pub fn register_fallback(&mut self, handler: SyscallHandler) {
        for x in 0..256 {
            if self.syscall_table[x] == sys_fallback {
                self.syscall_table[x] = handler;
            }
        }
        self.syscall_fallback = handler;
    }

    /// @brief Get the syscall handler for a given syscall number
    pub fn get_syscall(&self, syscall_num: u8) -> SyscallHandler {
        if syscall_num > 255 {
            return self.syscall_fallback;
        }
        self.syscall_table[syscall_num as usize]
    }

    /// @brief Get the fallback handler for all syscalls
    pub fn get_fallback(&self) -> SyscallHandler {
        self.syscall_fallback
    }

    /// @brief Deregister a syscall handler for a given syscall number
    pub fn deregister_syscall(&mut self, syscall_num: u8) {
        self.register_syscall(syscall_num, self.syscall_fallback);
    }
}

pub fn default_syscall_handler(_args: SyscallArgs) -> u64 {
    0
}

lazy_static! {
    pub static ref SYSCALL_TABLE: Mutex<SyscallTableDescriptor> = Mutex::new(SyscallTableDescriptor {
        syscall_table: [default_syscall_handler; 256],
        syscall_fallback: default_syscall_handler,
    });
}

/// @brief Set the main syscall table to a given table descriptor
pub fn set_table(table: SyscallTableDescriptor) {
    let mut t = SYSCALL_TABLE.lock();
    unsafe {
        for i in 0..256 {
            t.syscall_table[i] = table.syscall_table[i];
        }
        t.syscall_fallback = table.syscall_fallback;
    }
}

/// @brief Get a copy of the main syscall table
pub fn get_table() -> SyscallTableDescriptor {
    unsafe { SYSCALL_TABLE.lock().clone() }
}