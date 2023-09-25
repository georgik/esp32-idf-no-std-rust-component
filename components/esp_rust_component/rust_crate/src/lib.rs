#![cfg_attr(not(feature = "std"), no_std)]

use core::ffi::c_void;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
    }
}

use nmea::Nmea;
static mut BUFFER: [u8; 128] = [0; 128];
#[no_mangle]
pub extern "C" fn nmea_gga() -> *const c_void {
    let mut nmea = Nmea::default();
    let gga = "$GPGGA,092750.000,5321.6802,N,00630.3372,W,1,8,1.03,61.7,M,55.2,M,,*76";
    // unsafe {
    //     BUFFER[0] = 48;
    //     BUFFER[1] = 0;
    // };
    // return unsafe { BUFFER.as_ptr() as *const c_void };
    let sentence_type = match nmea.parse(gga) {
        Ok(result) => result,
        Err(_) => { 
            unsafe {
                BUFFER[0] = 46;
                BUFFER[1] = 0;
            };
            return unsafe { BUFFER.as_ptr() as *const c_void };
        },
    };
    // let sentence_string = sentence_type.as_str();
    // let result = nmea.parse(gga).unwrap();
    // let first_char = result.as_str().chars().next().unwrap();
    unsafe {
        // BUFFER[0] = first_char as u8;
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

