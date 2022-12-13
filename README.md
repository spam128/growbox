### Description

Driver to control heat and read tempareture and humidity inside growbox. Project written in rust on stm32f103C8 using
st-link v2

### Instalation

install the cross-toolchain for the STM32F1, which runs a thumbv7m-none-eabi ARM core:

`rustup update`

`rustup target install thumbv7m-none-eabi`

# we use cargo flash to program the microcontroller

`cargo install cargo-flash`

### Build

cargo build --release

### Release

cargo flash --chip stm32f103C8 --release

### DOCS

https://docs.rs/stm32f1xx-hal/latest/stm32f1xx_hal/index.html

### How to adapt it to different microcontrolers ?

- change .cargo/config file according to proper driver
- change memory.x file

### LCD
https://rastating.github.io/using-a-jhd162a-lcd-screen-with-an-arduino-uno/
https://circuitdigest.com/microcontroller-projects/interfacing-stm32f103c8t6-blue-pill-board-with-lcd-display

### Build issues

```commandline
       Error Failed to open the debug probe.

  Caused by:  
          0: Probe could not be created
          1: Access denied (insufficient permissions)
          2: Access denied (insufficient permissions)
```

Solution:

https://probe.rs/docs/getting-started/probe-setup/

