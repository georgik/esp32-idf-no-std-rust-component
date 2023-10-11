#![cfg_attr(not(feature = "std"), no_std)]

use core::ffi::{c_void, c_char, CStr};
use core::panic::PanicInfo;

use nmea::{Nmea, ParseResult};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static mut BUFFER: [u8; 128] = [0; 128];
use core::mem::size_of;
// use esp_println::println;

#[no_mangle]
pub extern "C" fn nmea_size() -> u32 {
    let size = core::mem::size_of::<Nmea>();
    size as u32
}

#[no_mangle]
pub extern "C" fn nmea_gga_altitude(gga_cstr: *const c_char) -> f32 {
    if gga_cstr.is_null() {
        return 0.0;
    }

    let c_str = unsafe { CStr::from_ptr(gga_cstr) };
    let gga_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return 0.0,
    };
    match nmea::parse_str(gga_str) {
        Ok(result) => {
            match result {
                ParseResult::GGA(gga) => gga.altitude.unwrap_or(0.0),
                _ => 0.0
            }
        },
        Err(_) => 0.0
    }
}

static HELLO_ESP32: &'static [u8] = b"Hello ESP-RS. https://github.com/esp-rs\0";

#[no_mangle]
pub extern "C" fn hello() -> *const c_void {
    HELLO_ESP32.as_ptr() as *const c_void
}

