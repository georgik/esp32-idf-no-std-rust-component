#include "stdio.h"
#include "esp_rust_component.h"

#include "freertos/FreeRTOS.h"
#include "freertos/task.h"

void nmea_gga_task(void* param) {
    UBaseType_t remaining_stack = uxTaskGetStackHighWaterMark(NULL);
    printf("Remaining stack space before calling Rust function in nmea_gga_task: %u\n", remaining_stack);

    float altitude1 = nmea_gga_altitude("$GPGGA,092750.000,5321.6802,N,00630.3372,W,1,8,1.03,61.7,M,55.2,M,,*76");
    printf("NMEA altitude: %f\n", altitude1);

    float altitude2 = nmea_gga_altitude("$GPGGA,101212.000,5107.0001,N,11402.5002,W,1,10,0.9,430.8,M,3.3,M,,*65");
    printf("NMEA altitude: %f\n", altitude2);

    float altitude3 = nmea_gga_altitude("$GPGGA,183040.000,4759.7547,N,12220.9000,W,1,7,1.1,92.3,M,3.0,M,,*6A");
    printf("NMEA altitude: %f\n", altitude3);

    remaining_stack = uxTaskGetStackHighWaterMark(NULL);
    printf("Remaining stack space after calling Rust function in nmea_gga_task: %u\n", remaining_stack);

    vTaskDelete(NULL);
}

void app_main() {
    UBaseType_t remaining_stack = uxTaskGetStackHighWaterMark(NULL);
    printf("Remaining stack space before calling Rust function in app_main: %u\n", remaining_stack);

    const char* message = hello();
    printf("Message from Rust: %s\n", message);

    printf("Size of NMEA: %lu\n", nmea_size());

    remaining_stack = uxTaskGetStackHighWaterMark(NULL);
    printf("Remaining stack space after calling Rust function in app_main: %u\n", remaining_stack);

    // Create a new task for running nmea_gga with a 12KB stack
    xTaskCreate(nmea_gga_task, "nmea_gga_task", 36240, NULL, 5, NULL);
}
