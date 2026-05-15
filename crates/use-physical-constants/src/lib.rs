#![forbid(unsafe_code)]

//! Reusable physical constants expressed as plain `f64` values.

/// Exact SI-defined speed of light in vacuum, in meters per second.
pub const SPEED_OF_LIGHT: f64 = 299_792_458.0;

/// Exact SI-defined Planck constant, in joule seconds.
pub const PLANCK_CONSTANT: f64 = 6.626_070_15e-34;

/// Derived physical constant h-bar, represented here as a rounded `f64` in joule seconds.
pub const REDUCED_PLANCK_CONSTANT: f64 = 1.054_571_817e-34;

/// Exact SI-defined elementary charge, in coulombs.
pub const ELEMENTARY_CHARGE: f64 = 1.602_176_634e-19;

/// Exact SI-defined Boltzmann constant, in joules per kelvin.
pub const BOLTZMANN_CONSTANT: f64 = 1.380_649e-23;

/// Exact SI-defined Avogadro constant, in reciprocal moles.
pub const AVOGADRO_CONSTANT: f64 = 6.022_140_76e23;

/// Measured fine-structure constant, dimensionless.
pub const FINE_STRUCTURE_CONSTANT: f64 = 7.297_352_564_3e-3;

/// Measured Newtonian constant of gravitation, in cubic meters per kilogram second squared.
pub const GRAVITATIONAL_CONSTANT: f64 = 6.674_30e-11;

/// Measured vacuum permittivity, in farads per meter.
pub const VACUUM_PERMITTIVITY: f64 = 8.854_187_812_8e-12;

/// Measured vacuum permeability, in newtons per ampere squared.
pub const VACUUM_PERMEABILITY: f64 = 1.256_637_062_12e-6;

/// Derived radiative constant sigma, represented here as a rounded `f64` in watts per square meter kelvin to the fourth.
pub const STEFAN_BOLTZMANN_CONSTANT: f64 = 5.670_374_419e-8;

#[cfg(test)]
mod tests {
    use core::f64::consts::TAU;

    use super::{
        PLANCK_CONSTANT, REDUCED_PLANCK_CONSTANT, SPEED_OF_LIGHT, STEFAN_BOLTZMANN_CONSTANT,
    };

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
    fn reduced_planck_matches_h_over_tau() {
        approx_eq(REDUCED_PLANCK_CONSTANT, PLANCK_CONSTANT / TAU, 1.0e-9);
    }

    #[test]
    fn representative_constants_are_positive() {
        assert!(runtime(SPEED_OF_LIGHT) > 0.0);
        assert!(runtime(PLANCK_CONSTANT) > 0.0);
        assert!(runtime(STEFAN_BOLTZMANN_CONSTANT) > 0.0);
    }
}
