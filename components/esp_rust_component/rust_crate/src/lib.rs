#![cfg_attr(not(feature = "std"), no_std)]
#![feature(alloc_error_handler)]

use core::ffi::c_void;
use core::panic::PanicInfo;
use core::alloc::{GlobalAlloc, Layout};

extern "C" {
    pub fn heap_caps_malloc(size: usize, caps: u32) -> *mut c_void;
    pub fn heap_caps_realloc(ptr: *mut c_void, size: usize, caps: u32) -> *mut c_void;
    pub fn heap_caps_free(ptr: *mut c_void);
}

pub const MALLOC_CAP_8BIT: u32 = 0x01;  // Update this to the correct value from ESP-IDF

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[global_allocator]
static HEAP: Esp32Alloc = Esp32Alloc;

#[alloc_error_handler]
fn on_oom(_layout: Layout) -> ! {
    panic!()
}

struct Esp32Alloc;

unsafe impl GlobalAlloc for Esp32Alloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        heap_caps_malloc(layout.size(), MALLOC_CAP_8BIT) as *mut _
    }

    unsafe fn realloc(&self, ptr: *mut u8, _layout: Layout, new_size: usize) -> *mut u8 {
        heap_caps_realloc(ptr as *mut _, new_size, MALLOC_CAP_8BIT) as *mut _
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        heap_caps_free(ptr as *mut _);
    }
}

use nmea::{Nmea, SentenceType};
use heapless::Deque;
use heapless::Vec;
use nmea::Satellite;

struct SatsPack {
    /// max number of visible GNSS satellites per hemisphere, assuming global coverage
    /// GPS: 16
    /// GLONASS: 12
    /// BeiDou: 12 + 3 IGSO + 3 GEO
    /// Galileo: 12
    /// => 58 total Satellites => max 15 rows of data
    #[cfg_attr(feature = "serde", serde(with = "serde_deq"))]
    data: Deque<Vec<Option<Satellite>, 4>, 15>,
    max_len: usize,
}

    /// Supported GNSS types
    enum GnssType {
        /// BeiDou Navigation Satellite System (BDS) from China.
        Beidou,
        /// European Global Navigation System (Galileo) from Europe.
        Galileo,
        /// Global Positioning System (GPS) from the United States.
        Gps,
        /// Globalnaya Navigazionnaya Sputnikovaya Sistema (GLONASS) from Russia.
        Glonass,
        /// Navigation Indian Constellation (NavIC) from India.
        NavIC,
        /// Quasi-Zenith Satellite System (QZSS) from Japan.
        Qzss,
    }

static mut BUFFER: [u8; 128] = [0; 128];
use nmea::SentenceMask;
use core::mem::size_of;
// use esp_println::println;

#[no_mangle]
pub extern "C" fn nmea_size() -> u32 {
    let size = core::mem::size_of::<SatsPack>();
    size as u32
}

#[no_mangle]
pub extern "C" fn nmea_gga() -> *const c_void {

    //let data: Deque<Vec<Option<Satellite>, 40>, 150> = Deque::new();
    // let sp = SatsPack {
    //     data,
    //     max_len: 15,
    // };
    // let mut nmea = Nmea::default();
    let sentence = [SentenceType::RMC, SentenceType::GGA];
    let required_sentences_for_nav: SentenceMask ;
    let mut nmea = Nmea::create_for_navigation(&sentence).unwrap();
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
    let sentence_string = sentence_type.as_str();
    let result = nmea.parse(gga).unwrap();
    let first_char = result.as_str().chars().next().unwrap();
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

