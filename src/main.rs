#![no_main]
#![no_std]

#[link_section=".text._start"]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe {
        // This is a random memory write the compiler should not optimize away.
        core::ptr::write_volatile(0x1234_5678 as *mut u32, 1<<3);
    }

    // Must never return.
    loop {}
}

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_panic: &PanicInfo) -> ! {
    loop {}
}
