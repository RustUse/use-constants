# use-earth-constants

Reusable Earth, WGS84, geodesy-adjacent, and standard-environment constants for RustUse.

This crate provides plain exported constants for Earth size, Earth mass, standard gravity,
standard sea-level pressure, and WGS84 reference ellipsoid values.

It does not include geodesy algorithms, coordinate transforms, or unit conversion logic.

## Usage

```rust
use use_earth_constants::{EARTH_RADIUS_MEAN, STANDARD_GRAVITY, WGS84_A};

assert!(EARTH_RADIUS_MEAN > 0.0);
assert!(STANDARD_GRAVITY > 9.8);
assert_eq!(WGS84_A, 6_378_137.0);
```
