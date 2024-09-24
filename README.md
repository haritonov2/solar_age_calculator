# Solar Age Calculator

This is a Rust library that calculates a person's age on different planets in the Solar System based on their age in seconds on Earth.

## Problem Overview

In the future, you need to fill out a form on another planet, but the customs officer expects your age in that planet’s years. This library solves that problem by converting your age from Earth years to other planets’ years, considering each planet's unique orbital period.

## Supported Planets

* Mercury
* Venus
* Earth
* Mars
* Jupiter
* Saturn
* Uranus
* Neptune

## Usage Example
To calculate your age on any planet:

```rust
use solar_age_calculator::{Duration, Earth, Mars, Planet};

// Let's say you are 1,000,000,000 seconds old:
let duration = Duration::from(1_000_000_000);

// Age on Earth
let earth_age = Earth::years_during(&duration);
println!("Age on Earth: {:.2}", earth_age);

// Age on Mars
let mars_age = Mars::years_during(&duration);
println!("Age on Mars: {:.2}", mars_age);
```

This example calculates your age on Earth and Mars, given that you are 1 billion seconds old.

## Running Tests

To run the tests included in this project, use:

```bash
cargo test
```


