const char* hello();

#include <stdint.h>
#include <stddef.h> // for NULL and size_t

typedef struct {
    uint16_t value;
} Rgb565;

#ifdef __cplusplus
extern "C" {
#endif

// Function to get a pointer to the framebuffer data
const void* framebuffer();

#ifdef __cplusplus
}
#endif