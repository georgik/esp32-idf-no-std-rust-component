#include "stdio.h"
#include "test_cmp.h"

void app_main() {
    const char* message = hello();
    printf("%s\n", message);
}
