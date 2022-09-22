#![allow(unused)]

use core::arch::asm;

const SBI_SET_TIMER: usize = 0;
const SBI_CONSOLE_PUTCHAR: usize = 1;
const SBI_CONSOLE_GETCHAR: usize = 2;
const SBI_CLEAR_IPI: usize = 3;
const SBI_SEND_IPI: usize = 4;
const SBI_REMOTE_FENCE_I: usize = 5;
const SBI_REMOTE_SFENCE_VMA: usize = 6;
const SBI_REMOTE_SFENCE_VMA_ASID: usize = 7;
const SBI_SHUTDOWN: usize = 8;

//Pengw: sbi_call's parameters, which means the service type of RustSBI, arg0-argN are different parameters that send to the system call.
//Pengw: Also,
#[inline(always)]
fn sbi_call(which: usize, arg0: usize, arg1: usize, arg2: usize) -> usize {
    let mut ret;
    unsafe {
        asm!(
            "li x16, 0", // Pengw: li instrument is load the immediate. In there, I think it want to clear the x16 register.
            "ecall",     // Pengw: Environment Call
            inlateout("x10") arg0 => ret, // Pengw:Ecall's parameters
            in("x11") arg1,
            in("x12") arg2,
            in("x17") which,
        );
    }
    ret
}

/// use sbi call to putchar in console (qemu uart handler)
pub fn console_putchar(c: usize) {
    sbi_call(SBI_CONSOLE_PUTCHAR, c, 0, 0);
}

/// use sbi call to getchar from console (qemu uart handler)
pub fn console_getchar() -> usize {
    sbi_call(SBI_CONSOLE_GETCHAR, 0, 0, 0)
}

#[cfg(feature = "board_qemu")]
use crate::board::QEMUExit;
/// use sbi call to shutdown the kernel
pub fn shutdown() -> ! {
    #[cfg(feature = "board_k210")]
    sbi_call(SBI_SHUTDOWN, 0, 0, 0);

    #[cfg(feature = "board_qemu")]
    crate::board::QEMU_EXIT_HANDLE.exit_failure();

    panic!("It should shutdown!");
}
