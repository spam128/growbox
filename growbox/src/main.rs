// std and main are not available for bare metal software
// stm32f1xx examples
//https://github.com/stm32-rs/stm32f1xx-hal/tree/master/examples
// esp8266 tutorial
//https://circuitdigest.com/microcontroller-projects/interfacing-esp8266-with-stm32f103c8-stm32-to-create-a-webserver

// blinki tutorial
// https://jonathanklimt.de/electronics/programming/embedded-rust/rust-on-stm32-2/
#![no_std]
#![no_main]

use cortex_m_rt::entry;

// The runtime
// the `set_high/low`function
use stm32f1xx_hal::{delay::Delay, pac, prelude::*, gpio};
// STM32F1 specific functions
// the `set_high/low`function
use embedded_hal::digital::v2::OutputPin;
// https://crates.io/crates/dht11
use dht11::Dht11;

#[allow(unused_imports)]
use panic_halt; // When a panic occurs, stop the microcontroller

const HEAT_TEMP: i16 = 230;

// This marks the entrypoint of our application. The cortex_m_rt creates some
// startup code before this, but we don't need to worry about this
#[entry]
fn main() -> ! {
    // PIN setup
    // PC13 - heather
    // PC14 - dht11 - temperature and humidity sensor
    // PC15 - led, turn on when dht11 is working incorrectly

    // Get handles to the hardware objects. These functions can only be called
    // once, so that the borrowchecker can ensure you don't reconfigure
    // something by accident.
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    // GPIO pins on the STM32F1 must be driven by the APB2 peripheral clock.
    // This must be enabled first. The HAL provides some abstractions for
    // us: First get a handle to the RCC peripheral:
    let mut rcc = dp.RCC.constrain();
    // Now we have access to the RCC's registers. The GPIOC can be enabled in
    // RCC_APB2ENR (Prog. Ref. Manual 8.3.7), therefore we must pass this
    // register to the `split` function.

    let mut gpioc = dp.GPIOC.split(&mut rcc.apb2);

    // #############################################
    // This gives us an exclusive handle to the GPIOC peripheral. To get the
    // handle to a single pin, we need to configure the pin first. Pin C13
    // is usually connected to the Bluepills onboard LED.
    let mut heath_pin = gpioc.pc13.into_open_drain_output(&mut gpioc.crh);
    let dht11_pin = gpioc.pc14.into_open_drain_output(&mut gpioc.crh);
    let mut dht11_error_pin = gpioc.pc15.into_push_pull_output(&mut gpioc.crh);

    // Create an instance of the DHT11 device
    let mut dht11_driver = Dht11::new(dht11_pin);

    // Now we need a delay object. The delay is of course depending on the clock
    // frequency of the microcontroller, so we need to fix the frequency
    // first. The system frequency is set via the FLASH_ACR register, so we
    // need to get a handle to the FLASH peripheral first:
    let mut flash = dp.FLASH.constrain();
    // Now we can set the controllers frequency to 8 MHz:
    let clocks = rcc.cfgr.sysclk(8.mhz()).freeze(&mut flash.acr);
    // The `clocks` handle ensures that the clocks are now configured and gives
    // the `Delay::new` function access to the configured frequency. With
    // this information it can later calculate how many cycles it has to
    // wait. The function also consumes the System Timer peripheral, so that no
    // other function can access it. Otherwise the timer could be reset during a
    // delay.
    let mut delay = Delay::new(cp.SYST, clocks);

    loop {
        match dht11_driver.perform_measurement(&mut delay) {
            Ok(measurement) => control_heat(measurement, &mut heath_pin, &mut dht11_error_pin),
            Err(_e) => {
                dht11_error_pin.set_high().ok();
                // heat off
                heath_pin.set_high().ok();
            }
        };

        delay.delay_ms(1_000_u16);
    }
}

fn control_heat(measurement: dht11::Measurement,
                heath_pin: &mut gpio::gpioc::PC13<gpio::Output<gpio::OpenDrain>>,
                dht11_error_pin: &mut gpio::gpioc::PC15<gpio::Output<gpio::PushPull>>) {
    // The measured temperature is in tenths of degrees Celsius.
    if (measurement.temperature) < HEAT_TEMP {
        // heat on
        heath_pin.set_high().ok();
    } else {
        // heat off
        heath_pin.set_low().ok();
    }
    // reset error pin
    dht11_error_pin.set_low().ok();
}