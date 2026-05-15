#![forbid(unsafe_code)]

//! Thin facade re-exports for the focused crates in the RustUse constants workspace.

pub use use_astronomical_constants as astronomical;
pub use use_chemical_constants as chemical;
pub use use_computing_constants as computing;
pub use use_earth_constants as earth;
pub use use_math_constants as math;
pub use use_physical_constants as physical;

#[cfg(test)]
mod tests {
    use super::{astronomical, chemical, computing, earth, math, physical};

    fn runtime(value: f64) -> f64 {
        value
    }

    fn runtime_usize(value: usize) -> usize {
        value
    }

    #[test]
    fn facade_reexports_domain_modules() {
        assert_eq!(math::TAU, 2.0 * math::PI);
        assert!(runtime(physical::SPEED_OF_LIGHT) > 0.0);
        assert!(runtime(chemical::FARADAY_CONSTANT) > 0.0);
        assert_eq!(
            runtime_usize(computing::MEBIBYTE),
            1024 * runtime_usize(computing::KIBIBYTE)
        );
        assert!(runtime(astronomical::LIGHT_YEAR) > runtime(astronomical::ASTRONOMICAL_UNIT));
        assert!(runtime(earth::EARTH_RADIUS_MEAN) > 0.0);
    }
}
