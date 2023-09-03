use core::f64::consts::PI;

fn altitude(latitude: f64, declination: f64, hour_angle: f64) -> f64 {
    (latitude.sin() * declination.sin()
        + latitude.cos() * declination.cos() * hour_angle.cos().abs())
    .asin()
}

fn refraction(altitude: f64) -> f64 {
    let h = altitude.to_degrees();
    let r: f64 = 1.02 / (60.0 * tan((h + 10.3 / (h + 5.11)).to_radians()));
    r.to_radians()
}

fn azimuth(latitude: f64, declination: f64, hour_angle: f64, altitude: f64) -> f64 {
    let azimuth = (hour_angle.sin() * declination.cos()) / altitude.cos();
    let azimuth = if azimuth.abs() > 1.0 {
        azimuth.signum()
    } else {
        azimuth
    };
    let azimuth = azimuth.asin();
    if hour_angle.sin() > 0.0 {
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
