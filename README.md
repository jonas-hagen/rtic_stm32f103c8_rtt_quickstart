# {{project-name}}

## About this template

This is a template for quickly starting an [RTIC](https://rtic.rs) project with an STM32F103C8 microcontroller. Particularly, this template uses:

- `cargo-embed` for flashing
- RTT for debug printing
- High optimisations in debug profile, to keep binaries small for all builds

### Using this template

Use the `cargo generate` sub command. [Installation instructions](https://github.com/ashleygwilliams/cargo-generate#installation).

``` console
$ cargo generate --git https://github.com/jonas-hagen/rtic_stm32f103c8_rtt_quickstart
 Project Name: app
 Creating project called `app`...
 Done! New project created /tmp/app

$ cd app
```

## Dependencies

To build embedded programs using this template you'll need:

- Rust nightly


- `rust-std` components (pre-compiled `core` crate) for the ARM Cortex-M targets. Run:
``` console
$ rustup target add thumbv6m-none-eabi thumbv7m-none-eabi thumbv7em-none-eabi thumbv7em-none-eabihf
```

- `cargo-embed` from the [probe.rs](https://probe.rs/) project.

## Build and flash

Build and flash the program and start the RTT terminal:

```console
$ cargo embed
```

This should show the "Hello World!" output from the MCU.

## License

This template is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
