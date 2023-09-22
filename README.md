# Integrating a Rust Component into an ESP-IDF Project

ESP-IDF, the official development framework for the ESP32 Series SoCs, supports integration of components written in C/C++ and Rust which is gaining traction for embedded development due to its safety features. This article outlines the steps to add a Rust component to your ESP-IDF project.

## Prerequisites

- [Installed ESP-IDF](https://docs.espressif.com/projects/esp-idf/en/latest/esp32/get-started/index.html#installation)
- [Installed Rust and Cargo](https://www.rust-lang.org/tools/install)
- [Installed Xtensa LLVM toolchain for Rust](https://esp-rs.github.io/book/installation/index.html)
- Basic knowledge of ESP-IDF, CMake, and Rust

## Structure

Here's how your project directory might look after the following the guide:

```
esp_idf_project/
|-- CMakeLists.txt
|-- main/
|   |-- CMakeLists.txt
|   |-- esp_idf_project.c
|-- sdkconfig
|-- components/
|   |-- esp_rust_component/
|       |-- CMakeLists.txt
|       |-- include/
|           |-- esp_rust_component.h
|       |-- esp_rust_component.c
|       |-- rust_crate/
|           |-- Cargo.toml
|           |-- rust-toolchain.toml
|           |-- src/
|               |-- lib.rs
```

### Architecture

Key elements:
- `esp_idf_project` contains main C code like any other ESP-IDF application.
- The ESP-IDF componet with name `esp_rust_component` is stored in subdirectory with components.
  - The `esp_rust_component` component contains C adapter layer which helps interfacing with Rust library.
- The Rust code is stored in `components/esp_rust_component/rust_crate` subdirectory.

The component can be uploaded later on to [Component Manager](https://components.espressif.com/).

## Step-by-Step Guide

### Set-up the environment

Before starting the project, make sure that the [Prerequisites](#prerequisites) are met, and that you have sourced the required export files.

### Create ESP-IDF project

Use ESP-IDF tooling to create new project with name `esp_idf_project`.

```
idf.py create-project esp_idf_project
cd esp_idf_project
```

### Create the ESP-IDF Component

Create a new directory in your `components/` folder. You can name it `esp_rust_component`.

```
mkdir components
cd components
idf.py create-component esp_rust_component
```

### Set up the CMakeLists.txt File

In your `esp_rust_component` directory, edit the [`CMakeLists.txt`](./components/esp_rust_component/CMakeLists.txt) file with the following content:

```cmake
idf_component_register(
    SRCS "esp_rust_component.c"
    INCLUDE_DIRS "include"
)

# Define the Rust target for the Xtensa and RISC-V architecture
if (CONFIG_IDF_TARGET_ARCH_XTENSA)
    set(RUST_CARGO_TOOLCHAIN "+esp")
    set(RUST_CARGO_TARGET "xtensa-${IDF_TARGET}-none-elf")
elseif (CONFIG_IDF_TARGET_ARCH_RISCV)
    set(RUST_CARGO_TOOLCHAIN "+nightly")
    set(RUST_CARGO_TARGET "riscv32imac-unknown-none-elf")
else()
    message(FATAL_ERROR "Architecture currently not supported")
endif()

# Set the flags for cargo build
set(CARGO_BUILD_FLAGS "-Zbuild-std=core")

# Set directories and target
set(RUST_PROJECT_DIR "${CMAKE_CURRENT_LIST_DIR}/rust_crate")
set(RUST_BUILD_DIR "${CMAKE_CURRENT_BINARY_DIR}")
set(RUST_TARGET_DIR "${RUST_BUILD_DIR}/target")
set(RUST_STATIC_LIBRARY "${RUST_TARGET_DIR}/${RUST_CARGO_TARGET}/release/librust_crate.a")

# ExternalProject_Add for building the Rust project
ExternalProject_Add(
    rust_crate_target
    PREFIX "${RUST_PROJECT_DIR}"
    DOWNLOAD_COMMAND ""
    CONFIGURE_COMMAND ""
    BUILD_COMMAND ${CMAKE_COMMAND} -E env
        CARGO_BUILD_TARGET=${RUST_CARGO_TARGET}
        CARGO_BUILD_TARGET_DIR=${RUST_TARGET_DIR}
        cargo ${RUST_CARGO_TOOLCHAIN} build --release ${CARGO_BUILD_FLAGS} -Zbuild-std-features=compiler-builtins-weak-intrinsics
    BUILD_ALWAYS TRUE
    INSTALL_COMMAND ""
    WORKING_DIRECTORY ${RUST_PROJECT_DIR}
    TMP_DIR "${RUST_BUILD_DIR}/tmp"
    STAMP_DIR "${RUST_BUILD_DIR}/stamp"
    DOWNLOAD_DIR "${RUST_BUILD_DIR}"
    SOURCE_DIR "${RUST_PROJECT_DIR}"
    BINARY_DIR "${RUST_PROJECT_DIR}"
    INSTALL_DIR "${RUST_BUILD_DIR}"
    BUILD_BYPRODUCTS "${RUST_STATIC_LIBRARY}"
)

# Add prebuilt Rust library
add_prebuilt_library(rust_crate_lib "${RUST_STATIC_LIBRARY}" REQUIRES "")

# Add dependencies and link Rust library
add_dependencies(${COMPONENT_LIB} rust_crate_target)
target_link_libraries(${COMPONENT_LIB} PUBLIC rust_crate_lib)
```

### Create a Rust Project Inside the Component

Create a new Rust crate, which will be a library, inside `esp_rust_component` called `rust_crate`:

```bash
cargo init --lib rust_crate
```

Update the `Cargo.toml` to match the settings for your target board. Also set the crate type to `staticlib`:

```toml
[package]
name = "rust_crate"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["staticlib"]

[dependencies]

[features]
default = [ ]
```

### Rust to C Interoperability

Add a Rust function with C linkage in your `lib.rs` that will be callable from C code. An example might be:

```rust
#![cfg_attr(not(feature = "std"), no_std)]

use core::ffi::c_void;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
    }
}

static HELLO_ESP32: &'static [u8] = b"Hello ESP-RS. https://github.com/esp-rs\0";

#[no_mangle]
pub extern "C" fn hello() -> *const c_void {
    HELLO_ESP32.as_ptr() as *const c_void
}
```

### Create a C Wrapper

Create file `esp_rust_component/esp_rust_component.c` to include the Rust functions.

```c
#include "rust_component.h"
```

### Update the Header File

Include the C header file in your `esp_rust_component/include/esp_rust_component.h`:

```c
extern const void* hello();
```

### Call Rust code from C

Update main ESP-IDF project file `main/esp_idf_project.c`:

```c
#include "stdio.h"
#include "esp_rust_component.h"

void app_main() {
    const char* message = hello();
    printf("%s\n", message);
}
```

### Select target

Set target for main ESP-IDF application:

```bash
idf.py set-target <target>
# idf.py set-target esp32
# idf.py set-target esp32-c3
# idf.pu set-target esp32-s3
```

Optional step when developers needs to build Rust component also manually:
Define which toolchain should be used for the Rust component in file `esp_rust_component/rust_crate/rust-toolchain.toml`

```toml
[toolchain]
# Use "esp" for ESP32, ESP32S2, and ESP32S3
channel = "esp"
# Use "nightly" for ESP32-C*, ESP32-H* targets
# channel = "nightly"

```


### Build the Project
From the base folder of the project (`esp_idf_project`), run the build process as you usually would for an ESP-IDF project:

```bash
idf.py build flash monitor
```

This command will build, flash the resulting binary to your board and open a serial monitor.

## Troubleshooting

- If you encounter linker errors, you may need to update your Rust flags. For example, you might need to add the `-Zbuild-std-features=compiler-builtins-weak-intrinsics` flag to `CARGO_BUILD_FLAGS` in your `CMakeLists.txt`.

---

That's it! You have successfully added a Rust component to your ESP-IDF project. Now you can leverage the safety and robustness of Rust while taking advantage of ESP-IDF's features.
