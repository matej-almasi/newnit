# Newnit - Convert

A CLI utility for unit conversions, based on my [`newnit`] package.

## Installation

Binaries are available in the [releases] section.

## Usage

To convert between units, run the command like so:

```sh
convert 20 celsius fahrenheit
```

This will output the converted unit rounded to 4 decimal points. To use a
different precision, use the (-p|--precision) argument with a non-negative
integer:

```sh
convert 20 celsius fahrenheit -p 1
```

[`newnit`]: https://github.com/matej-almasi/newnit/tree/main/newnit
[releases]: https://github.com/matej-almasi/newnit/releases

## Help

To get help with available units and options, run with the (-h|--help) argument.

```sh
convert -h
```

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
