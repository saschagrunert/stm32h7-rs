# STM32H7 Rust
[![Build Status](https://travis-ci.org/saschagrunert/stm32h7-rs.svg)](https://travis-ci.org/saschagrunert/stm32h7-rs) [![License MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/saschagrunert/stm32h7-rs/blob/master/LICENSE) [![Crates.io](https://img.shields.io/crates/v/stm32h7xx.svg)](https://crates.io/crates/stm32h7xx) [![doc.rs](https://docs.rs/stm32h7xx/badge.svg)](https://docs.rs/crate/stm32h7xx)

A template for building applications for ARM Cortex-M7 STM32H7xx
Microcontrollers.

# Dependencies

- [Rust nightly toolchain](https://github.com/rust-lang-nursery/rustup.rs)
- ARM Cross Compiler (arm-none-eabi-gcc)

# Building

Install the rust-std component for your target, if you haven't done so already

```console
$ rustup target add thumbv7em-none-eabihf
```

Build the application:

```consol
$ cargo build --release
```
