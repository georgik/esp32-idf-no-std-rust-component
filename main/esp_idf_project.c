#include "stdio.h"
#include "rust_component.h"

void app_main() {
    const char* message = hello();
    printf("%s\n", message);
}

