# Integrating a Rust Component into an ESP-IDF Project

ESP-IDF, the official development framework for the ESP32 Series SoCs, supports integration of components written in C/C++. However, Rust is gaining traction for embedded development due to its safety features. This article outlines the steps to add a Rust component to your ESP-IDF project.

## Prerequisites

- Installed ESP-IDF
- Installed Rust and Cargo
- Installed xtensa LLVM toolchain for Rust
- Basic knowledge of ESP-IDF, CMake, and Rust

## Structure

Here's how your project directory might look after the setup:

```
my_esp_app/
|-- CMakeLists.txt
|-- sdkconfig
|-- components/
|   |-- rust_component/
|       |-- CMakeLists.txt
|       |-- include/
|           |-- rust_component.h
|       |-- src/
|           |-- wrapper.c
|       |-- rust_hello/
|           |-- Cargo.toml
|           |-- src/
|               |-- lib.rs
```

## Step-by-Step Guide

### 1. Create the ESP-IDF Component

Create a new directory in your `components/` folder. You can name it `rust_component`.

### 2. Set up the CMakeLists.txt File

In your `rust_component` directory, create a `CMakeLists.txt` file with the following content:

```cmake
# CMakeLists.txt
# Basic component registration
idf_component_register(
    SRCS "src/wrapper.c"
    INCLUDE_DIRS "include"
)

# Define the Rust target for the Xtensa architecture
if (CONFIG_IDF_TARGET_ARCH_XTENSA)
    set(Rust_CARGO_TARGET "xtensa-${IDF_TARGET}-none-elf")
else()
    message(FATAL_ERROR "Architecture currently not supported")
endif()

# Set the flags for cargo build
set(CARGO_BUILD_FLAGS "-Zbuild-std=core")

# Set directories and target
set(RUST_PROJECT_DIR "${CMAKE_CURRENT_LIST_DIR}/rust_hello")
set(RUST_BUILD_DIR "${CMAKE_CURRENT_BINARY_DIR}")
set(RUST_TARGET_DIR "${RUST_BUILD_DIR}/target")
set(RUST_STATIC_LIBRARY "${RUST_TARGET_DIR}/${Rust_CARGO_TARGET}/release/librust_hello.a")

# ExternalProject_Add for building the Rust project
ExternalProject_Add(
    rust_hello_target
    PREFIX "${RUST_PROJECT_DIR}"
    DOWNLOAD_COMMAND ""
    CONFIGURE_COMMAND ""
    BUILD_COMMAND ${CMAKE_COMMAND} -E env
        CARGO_BUILD_TARGET=${Rust_CARGO_TARGET}
        CARGO_BUILD_TARGET_DIR=${RUST_TARGET_DIR}
        cargo build --release ${CARGO_BUILD_FLAGS} -Zbuild-std-features=compiler-builtins-weak-intrinsics
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
add_prebuilt_library(rust_hello_lib "${RUST_STATIC_LIBRARY}" REQUIRES "")

# Add dependencies and link Rust library
add_dependencies(${COMPONENT_LIB} rust_hello_target)
target_link_libraries(${COMPONENT_LIB} PUBLIC rust_hello_lib)
```

### 3. Create a Rust Project Inside the Component

Create a new folder inside `rust_component` called `rust_hello`. Then initialize a new Rust library:

```bash
$ cargo init --lib rust_hello
```

Update the `Cargo.toml` to match the settings for your ESP32 board. Also set the crate type to `staticlib`:

```toml
# Cargo.toml
[package]
name = "rust_hello"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[lib]
crate-type = ["staticlib"]

[dependencies]

[features]
default = [ ]
```

### 4. Rust to C Interoperability

Add a Rust function with C linkage in your `lib.rs` that will be callable from C code. An example might be:

```rust
static HELLO_ESP32: &'static [u8] = b"Hello Spooky\0";

#[no_mangle]
pub extern "C" fn hello() -> *const c_void {
    HELLO_ESP32.as_ptr() as *const c_void
}
```

### 5. Create a C Wrapper

Create a `wrapper.c` file in the `src/` directory inside `rust_component` to include the Rust functions.

```c
#include "rust_component.h"

// call Rust functions here
```

### 6. Update the Header File

Include the C header file in your `include/` directory:

```c
// rust_component.h
extern const void* hello();
```

### 7. Configure the ESP-IDF Project

Update the top-level `CMakeLists.txt` file in your ESP-IDF project to include the new component.

### 8. Build the Project

Run the build process as you usually would for an ESP-IDF project:

```bash
$ idf.py build
```

## Troubleshooting

- If you encounter linker errors, you may need to update your Rust flags. For example, you might need to add the `-Zbuild-std-features=compiler-builtins-weak-intrinsics` flag to `CARGO_BUILD_FLAGS` in your `CMakeLists.txt`.

---

That's it! You have successfully added a Rust component to your ESP-IDF project. Now you can leverage the safety and robustness of Rust while taking advantage of ESP-IDF's features.