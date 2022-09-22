//! The panic handler

use crate::sbi::shutdown;
use core::panic::PanicInfo;

#[panic_handler] //Pengw: It is a panic flag in rust
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println!(
            "[kernel] Panicked at {}:{} {}",
            location.file(),
            location.line(),
            info.message().unwrap()
        );
    } else {
        println!("[kernel] Panicked: {}", info.message().unwrap());
    }
    shutdown()
}
