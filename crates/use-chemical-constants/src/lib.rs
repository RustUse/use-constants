#![forbid(unsafe_code)]

//! Reusable chemical and thermodynamic constants expressed as plain `f64` values.

/// Derived SI constant for the molar gas constant, represented here as a rounded `f64` in joules per mole kelvin.
pub const GAS_CONSTANT: f64 = 8.314_462_618_153_24;

/// Derived SI constant for the Faraday constant, represented here as a rounded `f64` in coulombs per mole.
#[allow(clippy::excessive_precision)]
pub const FARADAY_CONSTANT: f64 = 96_485.332_123_310_018;

/// Conventional standard atmosphere, in pascals.
pub const STANDARD_ATMOSPHERE: f64 = 101_325.0;

/// Conventional standard temperature, in kelvin.
pub const STANDARD_TEMPERATURE_KELVIN: f64 = 273.15;

/// Conventional standard-state pressure, in pascals.
pub const STANDARD_PRESSURE_PASCAL: f64 = 100_000.0;

#[cfg(test)]
mod tests {
    use super::{FARADAY_CONSTANT, GAS_CONSTANT, STANDARD_ATMOSPHERE, STANDARD_PRESSURE_PASCAL};

    const AVOGADRO_CONSTANT: f64 = 6.022_140_76e23;
    const BOLTZMANN_CONSTANT: f64 = 1.380_649e-23;
    const ELEMENTARY_CHARGE: f64 = 1.602_176_634e-19;

    fn runtime(value: f64) -> f64 {
        value
    }

    fn approx_eq(left: f64, right: f64, relative_tolerance: f64) {
        let scale = left.abs().max(right.abs()).max(1.0);
        let delta = (left - right).abs();

        assert!(
            delta <= relative_tolerance * scale,
            "left={left:e} right={right:e} delta={delta:e} rel_tol={relative_tolerance:e}"
        );
    }

    #[test]
    fn faraday_matches_avogadro_times_charge() {
        approx_eq(
            FARADAY_CONSTANT,
            AVOGADRO_CONSTANT * ELEMENTARY_CHARGE,
            1.0e-12,
        );
    }

    #[test]
    fn gas_constant_matches_avogadro_times_boltzmann() {
        approx_eq(
            GAS_CONSTANT,
            AVOGADRO_CONSTANT * BOLTZMANN_CONSTANT,
            1.0e-12,
        );
    }

    #[test]
    fn conventional_reference_pressures_are_positive() {
        assert!(runtime(STANDARD_ATMOSPHERE) > runtime(STANDARD_PRESSURE_PASCAL));
        assert!(runtime(STANDARD_PRESSURE_PASCAL) > 0.0);
    }
}
