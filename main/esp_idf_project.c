#include "stdio.h"
#include "esp_rust_component.h"

#include "freertos/FreeRTOS.h"
#include "freertos/task.h"

void nmea_gga_task(void* param) {
    UBaseType_t remaining_stack = uxTaskGetStackHighWaterMark(NULL);
    printf("Remaining stack space before calling Rust function in nmea_gga_task: %u\n", remaining_stack);

    const char* message2 = nmea_gga();
    printf("MSG2: %s\n", message2);

    remaining_stack = uxTaskGetStackHighWaterMark(NULL);
    printf("Remaining stack space after calling Rust function in nmea_gga_task: %u\n", remaining_stack);

    vTaskDelete(NULL); // Delete this task when done
}

void app_main() {
    UBaseType_t remaining_stack = uxTaskGetStackHighWaterMark(NULL);
    printf("Remaining stack space before calling Rust function in app_main: %u\n", remaining_stack);

    const char* message = hello();
    printf("MSG1: %s\n", message);

    printf("size: %lu\n", nmea_size());

    remaining_stack = uxTaskGetStackHighWaterMark(NULL);
    printf("Remaining stack space after calling Rust function in app_main: %u\n", remaining_stack);

    // Create a new task for running nmea_gga with a 10KB stack
    xTaskCreate(nmea_gga_task, "nmea_gga_task", 10240, NULL, 5, NULL);
}
