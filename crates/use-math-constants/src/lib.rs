#![forbid(unsafe_code)]

//! Reusable mathematical constants expressed as plain `f64` values.

/// Mathematical constant $\pi$, dimensionless.
pub const PI: f64 = std::f64::consts::PI;

/// Mathematical constant $\tau = 2\pi$, dimensionless.
pub const TAU: f64 = std::f64::consts::TAU;

/// Mathematical constant $e$, dimensionless.
pub const E: f64 = std::f64::consts::E;

/// Approximate mathematical constant $\varphi$, the golden ratio, dimensionless.
pub const PHI: f64 = 1.618_033_988_749_895_f64;

/// Mathematical constant $\sqrt{2}$, dimensionless.
pub const SQRT_2: f64 = std::f64::consts::SQRT_2;

/// Approximate mathematical constant $\sqrt{3}$, dimensionless.
pub const SQRT_3: f64 = 1.732_050_807_568_877_2_f64;

/// Mathematical constant $\ln(2)$, dimensionless.
pub const LN_2: f64 = std::f64::consts::LN_2;

/// Mathematical constant $\ln(10)$, dimensionless.
pub const LN_10: f64 = std::f64::consts::LN_10;

/// Mathematical constant $\log_2(e)$, dimensionless.
pub const LOG2_E: f64 = std::f64::consts::LOG2_E;

/// Mathematical constant $\log_{10}(e)$, dimensionless.
pub const LOG10_E: f64 = std::f64::consts::LOG10_E;

/// Mathematical constant $1 / \pi$, dimensionless.
pub const FRAC_1_PI: f64 = std::f64::consts::FRAC_1_PI;

/// Mathematical constant $2 / \pi$, dimensionless.
pub const FRAC_2_PI: f64 = std::f64::consts::FRAC_2_PI;

/// Mathematical constant $\pi / 2$, dimensionless.
pub const FRAC_PI_2: f64 = std::f64::consts::FRAC_PI_2;

/// Mathematical constant $\pi / 3$, dimensionless.
pub const FRAC_PI_3: f64 = std::f64::consts::FRAC_PI_3;

/// Mathematical constant $\pi / 4$, dimensionless.
pub const FRAC_PI_4: f64 = std::f64::consts::FRAC_PI_4;

/// Mathematical constant $\pi / 6$, dimensionless.
pub const FRAC_PI_6: f64 = std::f64::consts::FRAC_PI_6;

/// Mathematical constant $\pi / 8$, dimensionless.
pub const FRAC_PI_8: f64 = std::f64::consts::FRAC_PI_8;

#[cfg(test)]
mod tests {
    use super::{FRAC_PI_4, PHI, PI, TAU};

    fn approx_eq(left: f64, right: f64, relative_tolerance: f64) {
        let scale = left.abs().max(right.abs()).max(1.0);
        let delta = (left - right).abs();

        assert!(
            delta <= relative_tolerance * scale,
            "left={left:e} right={right:e} delta={delta:e} rel_tol={relative_tolerance:e}"
        );
    }

    #[test]
    fn tau_matches_twice_pi() {
        assert_eq!(TAU, 2.0 * PI);
    }

    #[test]
    fn phi_matches_its_defining_quadratic() {
        approx_eq(PHI * PHI, PHI + 1.0, 1.0e-12);
    }

    #[test]
    fn quarter_turns_reconstruct_pi() {
        assert_eq!(FRAC_PI_4 * 4.0, PI);
    }
}
