#![no_std]

use core::ffi::c_void;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
    }
}

// Define a null-terminated string as a byte array
static HELLO_ESP32: &'static [u8] = b"Hello ESP64\0";

#[no_mangle]
pub extern "C" fn hello() -> *const c_void {
    HELLO_ESP32.as_ptr() as *const c_void
}

