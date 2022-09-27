### Description

Driver to control heat and read tempareture and humidity inside growbox. Project written in rust on stm32f103C8 using st-link v2

### Instalation

install the cross-toolchain for the STM32F1, which runs a thumbv7m-none-eabi ARM core:

`rustup target install thumbv7m-none-eabi`

# we use cargo flash to program the microcontroller

`cargo install cargo-flash`


### DOCS
https://docs.rs/stm32f1xx-hal/latest/stm32f1xx_hal/index.html
