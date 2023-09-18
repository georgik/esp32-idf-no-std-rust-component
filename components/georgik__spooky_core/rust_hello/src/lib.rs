#![cfg_attr(not(feature = "std"), no_std)]
pub mod assets;
pub mod demo_movement_controller;
pub mod engine;
pub mod maze;
pub mod movement_controller;
pub mod nomovement_controller;
pub mod spritebuf;
pub mod universe;

use embedded_graphics::{pixelcolor::Rgb565, prelude::DrawTarget};
use embedded_graphics_framebuf::FrameBuf;
use embedded_graphics::prelude::RgbColor;


use core::ffi::c_void;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
    }
}

// Define a null-terminated string as a byte array
static HELLO_ESP32: &'static [u8] = b"Hello Spooky\0";

#[no_mangle]
pub extern "C" fn hello() -> *const c_void {
    HELLO_ESP32.as_ptr() as *const c_void
}

static mut framebuffer_data: [Rgb565; 240 * 240 * 2] = [Rgb565::WHITE ; 240 * 240 * 2];

#[no_mangle]
pub extern "C" fn framebuffer() -> *const c_void {
    unsafe {
        framebuffer_data.as_ptr() as *const c_void
    }
}