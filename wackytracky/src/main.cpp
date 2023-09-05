#include "main.hpp"

#include <Arduino.h>
#include <Sgp4.h>
#include <Ticker.h>

const auto SPACE = " ";

Sgp4 sat;
Ticker ticker(ticker_callback, 1000);
int fps;

DateTime dt;

unsigned long start_time = DEFAULT_STAMP;

unsigned long unixtime() {
    return (unsigned long)(millis() / 1000L) + start_time;
}

// constexpr char OUT_STRING[] = "%04d-%02d-%02dT%02d:%02d:%02d+00:00 %f %f %f
// %f";
void ticker_callback() {
    invjday(sat.satJd, TIMEZONE, true, dt.year, dt.month, dt.day, dt.hour,
            dt.minute, dt.second);
    Serial.print(unixtime());
    Serial.print(SPACE);
    Serial.print(sat.satLat);
    Serial.print(SPACE);
    Serial.print(sat.satLon);
    Serial.print(SPACE);
    Serial.print(sat.satAlt);
    Serial.print(SPACE);
    Serial.println(sat.satAz);
}

void setup() {
    Serial.begin(115200);
    sat.site(LATITUDE, LONGITUDE, ALTITUDE);

    // Set the TLE
    sat.init(ISS_TLE_LINE0, ISS_TLE_LINE1, ISS_TLE_LINE2);
}
void loop() {}
