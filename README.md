# RustPill

Basic "blink" hello world programm for [STM32F103C8T6][stcom-stm32f103c8] based «Blue Pill» dev board.

Based on **@lupyuen** project [stm32-blue-pill-rust](https://github.com/lupyuen/stm32-blue-pill-rust).

Peripheral | STM32F103C8T6
------------ | -------------
Flash | 64 Kbytes
SRAM | 20 Kbytes
Timers | 3 General-purpose<br>1 Advanced-control
SPI | 2
I2C | 2
USART | 3
USB | 1
CAN | 1
GPIOs | 37
12-bit synchronized ADC | 2 (10 channels)
CPU frequency | 72 MHz
Operating voltage | 2.0 to 3.6 V
Operating ambient temperatures: | -40 to +85 °C
Operating junction temperature: | -40 to +105 °C
Packages | LQFP48, UFQFPN48

## Install tools

See [The Embedded Rust Book][erb-install-base] for more info

##### Add Cortex-M3 build target
```
$ rustup target add thumbv7m-none-eabi
```

##### Install cargo-binutils, llvm-tools-preview and cargo-generate
```
$ cargo install cargo-binutils
$ rustup component add llvm-tools-preview
$ cargo install cargo-generate
```

##### OS-Specific

See [The Embedded Rust Book][erb-install-specific]

## License

Distributed under the terms of the MIT license. See [LICENSE](LICENSE) for details.

[stcom-stm32f103c8]: https://www.st.com/en/microcontrollers-microprocessors/stm32f103c8.html
[erb-install-base]: https://docs.rust-embedded.org/book/intro/install.html
[erb-install-specific]: https://docs.rust-embedded.org/book/intro/install.html#os-specific-instructions