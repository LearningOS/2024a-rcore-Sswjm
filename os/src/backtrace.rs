//! print stack info

use core::{arch::asm, ptr};

pub unsafe fn back_trace() -> () {
    //! backtrace
    let mut fp:*const usize;
    asm!("mv {}, fp", out(reg) fp);

    println!("=== Begin Backtrace ===");

    while fp != ptr::null() {
        let saved_ra = *fp.sub(1);  // ra
        let saved_fp = *fp.sub(2);  // last fp

        println!("ra = 0x{:016x}, fp = 0x{:016x}", saved_ra, saved_fp);

        fp = saved_fp as *const usize;
    }

    println!("=== End Backtrace ===");
}