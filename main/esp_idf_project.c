#include "stdio.h"
#include "esp_rust_component.h"

#include "freertos/FreeRTOS.h"
#include "freertos/task.h"

#include <math.h>

void print_gga_data(struct CGgaData c_gga_data) {
    if (!isnan(c_gga_data.latitude)) {
        double lat_deg = floor(c_gga_data.latitude);
        double lat_min = (c_gga_data.latitude - lat_deg) * 60.0;
        printf("- Latitude: %.0f°%.4f' N\n", lat_deg, lat_min);
    } else {
        printf("- Latitude: N/A\n");
    }

    if (!isnan(c_gga_data.longitude)) {
        double lon_deg = floor(c_gga_data.longitude);
        double lon_min = (c_gga_data.longitude - lon_deg) * 60.0;
        printf("- Longitude: %.0f°%.4f' W\n", lon_deg, lon_min);
    } else {
        printf("- Longitude: N/A\n");
    }

    if (c_gga_data.fix_type != -1) {
        printf("- GPS Quality: %d (GPS fix)\n", c_gga_data.fix_type);
    } else {
        printf("- GPS Quality: N/A\n");
    }

    if (c_gga_data.fix_satellites != -1) {
        printf("- Number of Satellites: %d\n", c_gga_data.fix_satellites);
    } else {
        printf("- Number of Satellites: N/A\n");
    }

    if (!isnan(c_gga_data.hdop)) {
        printf("- Horizontal Dilution of Precision: %.1f\n", c_gga_data.hdop);
    } else {
        printf("- Horizontal Dilution of Precision: N/A\n");
    }

    if (!isnan(c_gga_data.altitude)) {
        printf("- Altitude: %.1f Meters\n", c_gga_data.altitude);
    } else {
        printf("- Altitude: N/A\n");
    }

    if (!isnan(c_gga_data.geoid_separation)) {
        printf("- Height of Geoid above WGS84 Ellipsoid: %.1f Meters\n", c_gga_data.geoid_separation);
    } else {
        printf("- Height of Geoid above WGS84 Ellipsoid: N/A\n");
    }
}


void nmea_gga_task(void* param) {
    UBaseType_t remaining_stack = uxTaskGetStackHighWaterMark(NULL);
    printf("Remaining stack space before calling Rust function in nmea_gga_task: %u\n", remaining_stack);

    float altitude = nmea_gga_altitude("$GPGGA,092750.000,5321.6802,N,00630.3372,W,1,8,1.03,61.7,M,55.2,M,,*76");
    printf("NMEA altitude: %f\n", altitude);

    remaining_stack = uxTaskGetStackHighWaterMark(NULL);
    printf("Remaining stack space after calling Rust function in nmea_gga_task: %u\n", remaining_stack);

    struct CGgaData c_gga_data = parse_nmea_gga("$GPGGA,092750.000,5321.6802,N,00630.3372,W,1,8,1.03,61.7,M,55.2,M,,*76");
    print_gga_data(c_gga_data);

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
