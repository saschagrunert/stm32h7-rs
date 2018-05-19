# STM32H7 Rust

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

```console
cargo build --release
```
