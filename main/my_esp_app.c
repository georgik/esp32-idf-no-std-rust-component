#include "stdio.h"
#include "test_cmp.h"

void app_main() {
    const char* message = hello();
    printf("%s\n", message);

    const void *framebuffer_ptr = framebuffer();

    if (framebuffer_ptr == NULL) {
        // handle the error
        return;
    }

    // Cast the void pointer to a byte pointer (uint8_t*)
    const uint8_t *byte_ptr = (const uint8_t *)framebuffer_ptr;

    // Dereference to get the first byte
    uint8_t first_byte = *byte_ptr;
    printf("The first byte: %d\n", first_byte);
}
