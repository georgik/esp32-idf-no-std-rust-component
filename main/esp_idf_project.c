#include "stdio.h"
#include "esp_rust_component.h"

void app_main() {
    const char* message = hello();
    printf("MSG1: %s\n", message);

    const char* message2 = nmea_gga();
    printf("MSG2: %s\n", message2);
}

