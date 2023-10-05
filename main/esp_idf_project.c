#include "stdio.h"
#include "esp_rust_component.h"

#include "freertos/FreeRTOS.h"
#include "freertos/task.h"

void app_main() {
    UBaseType_t remaining_stack;
    remaining_stack = uxTaskGetStackHighWaterMark(NULL);
    printf("Remaining stack space before calling Rust function: %u\n", remaining_stack);
    const char* message = hello();
    printf("MSG1: %s\n", message);

remaining_stack = uxTaskGetStackHighWaterMark(NULL);
    printf("Remaining stack space before calling Rust function: %u\n", remaining_stack);
    const char* message2 = nmea_gga();
    printf("MSG2: %s\n", message2);
}

