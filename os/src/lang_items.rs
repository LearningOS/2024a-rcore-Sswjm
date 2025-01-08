//! The panic handler

use crate::sbi::shutdown;
use core::panic::PanicInfo;
use crate::backtrace::back_trace;

#[panic_handler]
/// panic handler
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

    unsafe {back_trace();}
    shutdown()
}
