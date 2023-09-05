#pragma once
#define DEFAULT_STAMP 1693873904L  // 2023-09-05T00:31:44+00:00
#define TIMEZONE 2                 // UTC+2

// The following are coordinates near the TU Delft Elektrotechniek, Wiskunde
// en Informatica building.
#define LATITUDE 52.000000
#define LONGITUDE 4.373400
#define ALTITUDE 0.000000

// ISS TLE
#define ISS_TLE_LINE0 "ISS (ZARYA)"
#define ISS_TLE_LINE1 "1 25544U 98067A   23247.87296296  .00013631  00000-0  24440-3 0  9998"
#define ISS_TLE_LINE2 "2 25544  51.6429 293.7760 0005622  29.3608 322.2542 15.50223322414145"

typedef struct {
    int year;
    int month;
    int day;
    int hour;
    int minute;
    double second;
} DateTime;

unsigned long unixtime();
void ticker_callback();