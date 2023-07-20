use lazy_static::lazy_static;
use spin::Mutex;
use crate::kernel::kernel_main::sys_fallback;

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

pub type SyscallHandler = fn(SyscallArgs) -> u64;
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

    pub fn clone(&self) -> SyscallTableDescriptor {
        SyscallTableDescriptor {
            syscall_table: self.syscall_table,
            syscall_fallback: self.syscall_fallback,
        }
    }

    pub fn register_syscall(&mut self, syscall_num: u8, handler: SyscallHandler) {
        self.syscall_table[syscall_num as usize] = handler;
    }

    pub fn register_fallback(&mut self, handler: SyscallHandler) {
        for x in 0..256 {
            if self.syscall_table[x] == sys_fallback {
                self.syscall_table[x] = handler;
            }
        }
        self.syscall_fallback = handler;
    }

    pub fn get_syscall(&self, syscall_num: u8) -> SyscallHandler {
        self.syscall_table[syscall_num as usize]
    }

    pub fn get_fallback(&self) -> SyscallHandler {
        self.syscall_fallback
    }


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

pub fn set_table(table: SyscallTableDescriptor) {
    let mut t = SYSCALL_TABLE.lock();
    unsafe {
        for i in 0..256 {
            t.syscall_table[i] = table.syscall_table[i];
        }
        t.syscall_fallback = table.syscall_fallback;
    }
}

pub fn get_table() -> SyscallTableDescriptor {
    unsafe { SYSCALL_TABLE.lock().clone() }
}