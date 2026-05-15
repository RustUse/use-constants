#![forbid(unsafe_code)]

//! Reusable astronomical constants expressed as plain `f64` values.

/// Exact astronomical unit by IAU definition, in meters.
pub const ASTRONOMICAL_UNIT: f64 = 149_597_870_700.0;

/// Conventional light-year distance for one Julian year of light travel, in meters.
pub const LIGHT_YEAR: f64 = 9.460_730_472_580_8e15;

/// Approximate parsec distance, in meters.
pub const PARSEC: f64 = 3.085_677_581_491_367e16;

/// Measured solar mass, in kilograms.
pub const SOLAR_MASS: f64 = 1.988_47e30;

#[cfg(test)]
mod tests {
    use super::{ASTRONOMICAL_UNIT, LIGHT_YEAR, PARSEC, SOLAR_MASS};

    fn runtime(value: f64) -> f64 {
        value
    }

    #[test]
    fn larger_astronomical_distances_exceed_smaller_ones() {
        assert!(runtime(LIGHT_YEAR) > runtime(ASTRONOMICAL_UNIT));
        assert!(runtime(PARSEC) > runtime(LIGHT_YEAR));
    }

    #[test]
    fn representative_stellar_mass_is_positive() {
        assert!(runtime(SOLAR_MASS) > 0.0);
    }

    #[test]
    fn parsec_exceeds_astronomical_unit() {
        assert!(runtime(PARSEC) > runtime(ASTRONOMICAL_UNIT));
    }
}
