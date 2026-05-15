# use-constants

`use-constants` is a top-level RustUse workspace for small, focused crates that publish reusable constants across mathematics, physics, chemistry, computing, astronomy, and closely related domains.

This workspace exists at the top level because these constants are reused across multiple RustUse sets, including `use-math`, `use-physics`, `use-chemistry`, `use-units`, `use-measure`, `use-optics`, `use-acoustics`, `use-wave`, `use-signal`, and `use-simulation`.

## Crates

- `use-constants`: thin umbrella crate that re-exports the focused constants crates.
- `use-math-constants`: pure mathematical constants such as `PI`, `TAU`, `E`, and common fractional multiples of pi.
- `use-physical-constants`: exact SI-defined constants, derived physical constants, and well-known measured physical constants.
- `use-chemical-constants`: chemical and thermodynamic constants such as the gas constant, Faraday constant, and standard reference values.
- `use-computing-constants`: binary and decimal computing constants for bytes and common storage multiples.
- `use-astronomical-constants`: astronomical distance constants and non-Earth body reference constants.
- `use-earth-constants`: Earth, WGS84, geodesy-adjacent, and standard-environment constants.

## What Belongs Here

- Plain exported constant values.
- Small, domain-focused constant crates.
- Constants that are shared across more than one RustUse domain.
- Constants with clear units and scope.

## What Does Not Belong Here

- Unit conversion logic. That belongs in `use-units`.
- Measurement abstractions. Those belong in `use-measure`.
- Formula implementations. Those belong in the relevant domain crate.
- Complex types or runtime lookup systems.

## Usage

Import directly from a focused crate when you only need one domain:

```rust
use use_math_constants::{PI, TAU};
use use_physical_constants::SPEED_OF_LIGHT;

let radius = 3.0_f64;
let seconds = 2.0_f64;

let circumference = TAU * radius;
let light_distance = SPEED_OF_LIGHT * seconds;

assert!(circumference > PI);
assert!(light_distance > 0.0);
```

Earth-specific constants live in their own focused crate:

```rust
use use_earth_constants::{EARTH_RADIUS_MEAN, STANDARD_GRAVITY, WGS84_A};

assert!(EARTH_RADIUS_MEAN > 0.0);
assert!(STANDARD_GRAVITY > 9.8);
assert_eq!(WGS84_A, 6_378_137.0);
```

Import through the umbrella crate when you want grouped access:

```rust
use use_constants::{math, physical};

let radius = 3.0_f64;
let seconds = 2.0_f64;

let circumference = math::TAU * radius;
let light_distance = physical::SPEED_OF_LIGHT * seconds;

assert!(circumference > 0.0);
assert!(light_distance > 0.0);
```

The umbrella crate also exposes Earth constants:

```rust
use use_constants::earth;

let radius = earth::EARTH_RADIUS_MEAN;
let gravity = earth::STANDARD_GRAVITY;

assert!(radius > 0.0);
assert!(gravity > 9.8);
```

## Design Notes

- APIs stay simple and explicit.
- Constants are exposed as plain values, not wrappers.
- Precision notes in the docs avoid overclaiming exactness.
- Dependencies stay minimal and only exist where they clarify the data.
- Geodesy algorithms, coordinate transforms, and unit conversions stay outside this workspace.

## License

Licensed under `MIT OR Apache-2.0`.
