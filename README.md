# STM32H7 Rust

A template for building applications for ARM Cortex-M7 STM32H7xx
Microcontrollers.

# Dependencies

- [Rust nightly toolchain](https://github.com/rust-lang-nursery/rustup.rs)
- [xargo](https://github.com/japaric/xargo)
- ARM Cross Compiler (arm-none-eabi-gcc)

# Building

Be sure that your `env` contains `RUST_TARGET_PATH=$(pwd)`.

```console
xargo build
```
