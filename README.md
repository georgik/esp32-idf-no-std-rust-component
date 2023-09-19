# Integrating a Rust Component into an ESP-IDF Project

ESP-IDF, the official development framework for the ESP32 Series SoCs, supports integration of components written in C/C++ and Rust which is gaining traction for embedded development due to its safety features. This article outlines the steps to add a Rust component to your ESP-IDF project.

## Prerequisites

- Installed ESP-IDF
- Installed Rust and Cargo
- Installed xtensa LLVM toolchain for Rust
- Basic knowledge of ESP-IDF, CMake, and Rust

## Structure

Here's how your project directory might look after the setup:

```
esp_idf_project/
|-- CMakeLists.txt
|-- sdkconfig
|-- components/
|   |-- esp_rust_component/
|       |-- CMakeLists.txt
|       |-- include/
|           |-- esp_rust_component.h
|       |-- src/
|           |-- esp_rust_wrapper.c
|       |-- rust_crate/
|           |-- Cargo.toml
|           |-- src/
|               |-- lib.rs
```

### Architecture

Key elements:
- `esp_idf_project` contains main C code like any other ESP-IDF application.
- The ESP-IDF componet with name `esp_rust_component` is stored in subdirectory with components. The component contains C adapter layer which helps interfacing with Rust crate.
- The Rust code is then stored in `components/esp_rust_component/rust_crate` subdirectory.

The component can be uploaded later on to [Component Manager](https://components.espressif.com/).

## Step-by-Step Guide

### Create ESP-IDF project

Use ESP-IDF tooling to create new project with name `esp_idf_project`.

```
idf.py create-project esp_idf_project
cd esp_idf_project
```

### Create the ESP-IDF Component

Create a new directory in your `components/` folder. You can name it `rust_component`.

```
mkdir components
cd components
idf.py create-component rust_component
```

### Set up the CMakeLists.txt File

In your `rust_component` directory, create a `CMakeLists.txt` file with the following content:

```cmake
# Basic component registration
idf_component_register(
    SRCS "src/esp_rust_wrapper.c"
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

### Create a Rust Project Inside the Component

Create a new Rust crate inside `rust_component` called `rust_crate`. Then initialize a new Rust library:

```bash
cargo init --lib rust_crate
```

Update the `Cargo.toml` to match the settings for your ESP32 board. Also set the crate type to `staticlib`:

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
static HELLO_ESP32: &'static [u8] = b"Hello ESP-RS\0";

#[no_mangle]
pub extern "C" fn hello() -> *const c_void {
    HELLO_ESP32.as_ptr() as *const c_void
}
```

### Create a C Wrapper

Create file `rust_component/src/esp_rust_wrapper.c` to include the Rust functions.

```c
#include "rust_component.h"
```

### Update the Header File

Include the C header file in your `rust_component/include/rust_component.h`:

```c
extern const void* hello();
```

### Select target

#### Targets: ESP32, ESP32-S2, ESP32-S3

This chapter applies to chips with Xtensa architecture.

Define which toolchain should be used for the Rust component in file `rust_component/rust-toolchain.toml`

```toml
[toolchain]
channel = "esp"
```

Set target for main ESP-IDF application:

```bash
idf.py set-target esp32
# idf.py set-target esp32-s2
# idf.py set-target esp32-s3
```

#### Targets ESP32-C*, ESP32-H*

This chapter applies to chips with RISC-V architecture

Define which toolchain should be used for the Rust component in file `rust_component/rust-toolchain.toml`

```toml
[toolchain]
channel = "nightly"
```

Set target for main ESP-IDF application:

```bash
idf.py set-target esp32-c3
# idf.py set-target esp32-h2
# idf.py set-target esp32-c6
```


### Build the Project

Run the build process as you usually would for an ESP-IDF project:

```bash
idf.py build
```

## Troubleshooting

- If you encounter linker errors, you may need to update your Rust flags. For example, you might need to add the `-Zbuild-std-features=compiler-builtins-weak-intrinsics` flag to `CARGO_BUILD_FLAGS` in your `CMakeLists.txt`.

---

That's it! You have successfully added a Rust component to your ESP-IDF project. Now you can leverage the safety and robustness of Rust while taking advantage of ESP-IDF's features.