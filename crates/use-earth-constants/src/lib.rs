#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

/// Approximate mean Earth radius, in meters.
pub const EARTH_RADIUS_MEAN: f64 = 6_371_000.0;

/// WGS84-defined equatorial Earth radius, in meters.
pub const EARTH_RADIUS_EQUATORIAL: f64 = 6_378_137.0;

/// WGS84-defined polar Earth radius, in meters.
pub const EARTH_RADIUS_POLAR: f64 = 6_356_752.314_245;

/// Approximate Earth mass, in kilograms.
pub const EARTH_MASS: f64 = 5.972_2e24;

/// Conventional standard gravity, in meters per second squared.
pub const STANDARD_GRAVITY: f64 = 9.806_65;

/// Conventional standard sea-level atmospheric pressure, in pascals.
pub const SEA_LEVEL_STANDARD_PRESSURE: f64 = 101_325.0;

/// WGS84-defined semi-major axis, in meters.
pub const WGS84_A: f64 = 6_378_137.0;

/// WGS84-defined semi-minor axis, in meters.
pub const WGS84_B: f64 = 6_356_752.314_245;

/// WGS84-defined flattening, dimensionless.
pub const WGS84_FLATTENING: f64 = 1.0 / 298.257_223_563;

#[cfg(test)]
mod tests {
    use super::{
        EARTH_MASS, EARTH_RADIUS_EQUATORIAL, EARTH_RADIUS_MEAN, EARTH_RADIUS_POLAR,
        SEA_LEVEL_STANDARD_PRESSURE, STANDARD_GRAVITY, WGS84_A, WGS84_B, WGS84_FLATTENING,
    };

    fn runtime(value: f64) -> f64 {
        value
    }

    #[test]
    fn earth_radii_follow_expected_ordering() {
        assert!(runtime(EARTH_RADIUS_EQUATORIAL) > runtime(EARTH_RADIUS_MEAN));
        assert!(runtime(EARTH_RADIUS_MEAN) > runtime(EARTH_RADIUS_POLAR));
    }

    #[test]
    fn wgs84_axes_match_radius_aliases() {
        assert_eq!(EARTH_RADIUS_EQUATORIAL, WGS84_A);
        assert_eq!(EARTH_RADIUS_POLAR, WGS84_B);
    }

    #[test]
    fn representative_environment_constants_are_reasonable() {
        assert!(runtime(WGS84_FLATTENING) > 0.0);
        assert!(runtime(STANDARD_GRAVITY) > 9.8 && runtime(STANDARD_GRAVITY) < 9.9);
        assert_eq!(SEA_LEVEL_STANDARD_PRESSURE, 101_325.0);
        assert!(runtime(EARTH_MASS) > 0.0);
    }
}
