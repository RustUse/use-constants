#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

pub mod prelude;

#[cfg(feature = "astronomical-constants")]
pub use use_astronomical_constants as astronomical_constants;

#[cfg(feature = "chemical-constants")]
pub use use_chemical_constants as chemical_constants;

#[cfg(feature = "computing-constants")]
pub use use_computing_constants as computing_constants;

#[cfg(feature = "earth-constants")]
pub use use_earth_constants as earth_constants;

#[cfg(feature = "math-constants")]
pub use use_math_constants as math_constants;

#[cfg(feature = "physical-constants")]
pub use use_physical_constants as physical_constants;
