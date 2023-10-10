#![cfg_attr(not(feature = "std"), no_std)]

use core::ffi::c_void;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

use nmea::{Nmea, SentenceType};

static mut BUFFER: [u8; 128] = [0; 128];
use core::mem::size_of;
// use esp_println::println;

#[no_mangle]
pub extern "C" fn nmea_size() -> u32 {
    let size = core::mem::size_of::<Nmea>();
    size as u32
}

#[no_mangle]
pub extern "C" fn nmea_gga() -> *const c_void {
    let sentence = [SentenceType::RMC, SentenceType::GGA];
    let mut nmea = Nmea::create_for_navigation(&sentence).unwrap();
    let gga = "$GPGGA,092750.000,5321.6802,N,00630.3372,W,1,8,1.03,61.7,M,55.2,M,,*76";
    let _sentence_type = match nmea.parse(gga) {
        Ok(result) => result,
        Err(_) => {
            unsafe {
                BUFFER[0] = 46;
                BUFFER[1] = 0;
            };
            return unsafe { BUFFER.as_ptr() as *const c_void };
        },
    };
    let result = nmea.parse(gga).unwrap();
    let first_char = result.as_str().chars().next().unwrap();
    unsafe {
        BUFFER[0] = first_char as u8;
        BUFFER[0] = 45;
        BUFFER[1] = 0;
    }
    unsafe { BUFFER.as_ptr() as *const c_void }
}

static HELLO_ESP32: &'static [u8] = b"Hello2 ESP-RS. https://github.com/esp-rs\0";

#[no_mangle]
pub extern "C" fn hello() -> *const c_void {
    HELLO_ESP32.as_ptr() as *const c_void
}

