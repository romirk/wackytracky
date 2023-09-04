use core::f64::consts::PI;

fn altitude(latitude: f64, declination: f64, hour_angle: f64) -> f64 {
    libm::asin(
        libm::sin(latitude.to_radians()) * libm::sin(declination)
            + libm::cos(latitude.to_radians()) * libm::cos(declination) * libm::cos(hour_angle),
    )
}

fn refraction(altitude: f64) -> f64 {
    let h = altitude.to_degrees();
    let r: f64 = 1.02 / (60.0 * libm::tan((h + 10.3 / (h + 5.11)).to_radians()));
    r.to_radians()
}

fn azimuth(latitude: f64, declination: f64, hour_angle: f64, altitude: f64) -> f64 {
    let azimuth = libm::sin(hour_angle) * libm::cos(declination)
        / (libm::cos(latitude.to_radians()) * libm::sin(altitude)
            - libm::sin(latitude.to_radians()) * libm::cos(altitude) * libm::cos(hour_angle));
    let azimuth = libm::acos(azimuth);
    if hour_angle > 0.0 {
        azimuth
    } else {
        2.0 * PI - azimuth
    }
}

pub fn true_coordinates(hour_angle: f64, declination: f64, latitude: f64) -> (f64, f64) {
    let altitude = altitude(latitude, declination, hour_angle);
    let altitude = altitude - refraction(altitude);
    let azimuth = azimuth(latitude, declination, hour_angle, altitude);
    (altitude, azimuth)
}
