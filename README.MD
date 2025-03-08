# Newnit

A library providing type safety and ergonomics for working with units of
measurement, based on the newtype pattern.

## Features

### Predefined newtypes

A rich library of unit newtypes for common quantities:

```rust
use newnit::length::imperial::Foot;
use newnit::substance_amount::metric::KiloMole;

fn main() {
  let length = Foot(32.0);
  let oxygen_amount = KiloMole(4.0);
}
```

### Traits representing quantities

Traits representing common quantities are provided. All units in this library
implement their respective quantity trait.

You can use the `to_base()` method on any unit to get its representation in base
units (see individual quantity's module documentation for definition of its
respective base unit).

```rust
use newnit::Unit;
use newnit::mass::Mass;
use newnit::mass::imperial::Ounce;

fn display_weight(weight: &dyn Mass) {
    println!("Weight in kilograms is: {}", weight.to_base());
}

fn main() {
    let weight_in_ounces = Ounce(4.0);
    display_weight(&weight_in_ounces);
}
```

### Seamless conversion between compatible units

Units implementing the same quantity are inter-convertible seamlessly:

```rust
use newnit::Unit;
use newnit::area::metric::SquareMeter;
use newnit::area::imperial::SquareFoot;

fn main() {
    let bathroom_area = SquareMeter(6.0);
    let bathroom_area_in_sq_feet = SquareFoot::from(&bathroom_area);
}
```

### Compile time protection

Incompatible conversions are compile time errors:

```rust
use newnit::Unit;
use newnit::area::metric::SquareMeter;
use newnit::mass::metric::KiloGram;

fn main() {
    let bathroom_area = SquareMeter(6.0);

    // This is a compile error!
    let bad_conversion = KiloGram::from(&bathroom_area);
}
```

### Perform common mathematical operations with the units

You can add and subtract compatible units:

```rust
use newnit::length::metric::Meter;
use newnit::length::imperial::Foot;

fn main() {
    let meters = Meter(4.0);
    let feet = Foot(2.0);

    let sum = meters + &feet;
}
```

You can multiply and divide the units with a constant:

```rust
use newnit::length::metric::Meter;
use newnit::length::imperial::Foot;

fn main() {
    let meters = Meter(4.0);

    let times_three = 3.0 * meters;

    let divided_by_negative_five = meters / -5.0;
}
```

You can even multiply and divide units between each other, if such operation is
recognized by the library (or extended ad hoc by the user).

```rust
use newnit::length::Length;
use newnit::length::metric::Meter;
use newnit::time::metric::Second;
use newnit::velocity::metric::MeterPerSecond;

fn main() {
    let meters = Meter(4.0);
    let seconds = Second(3.1);

    let speed: MeterPerSecond = meters.divide_time(&seconds);
}
```

### Extend the library as needed

As usual, you may define your own units that will implement the quantity traits
provided by the library. You may even implement your own traits! To keep your
units (or quantities) compatible with the rest of the library, you have to:

- bind your quantity traits by the `Unit` trait
- implement the `Unit` trait for your units
  - make sure your conversions to/ from the base unit match the ones in the
    library

To make defining custom units easier, you can use a derive macro:

```rust
use newnit::{Unit, mass::Mass};
use newnit::derive::{Unit, Mass};

#[derive(Unit, Mass)]
#[unit(factor = 6000.0)]
#[quantity(partial_eq, ops)]  // opt in to provided PartialEq and Ops implementations
struct Elephants(f64);
```

## Installation

Available from [crates.io](https://crates.io/crates/newnit):

```sh
cargo add newnit
```

## Documentation

Available at [docs.rs](https://docs.rs/newnit).

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
