// https://crates.io/crates/dht11
use dht11::{Dht11};
use embedded_hal::digital::v2::OutputPin;
use stm32f1xx_hal::{gpio};

pub struct DHT11API {
    pub dht11_error_pin: gpio::gpioc::PC15<gpio::Output<gpio::PushPull>>,
    pub dht11_driver: Dht11<gpio::gpioc::PC14<gpio::Output<gpio::OpenDrain>>>,
}

impl DHT11API {
    pub(crate) fn new(
        dht11_pin: gpio::gpioc::PC14<gpio::Output<gpio::OpenDrain>>,
        dht11_error_pin: gpio::gpioc::PC15<gpio::Output<gpio::PushPull>>,
    ) -> Self {
        let dht11_driver = Dht11::new(dht11_pin);
        DHT11API {
            dht11_error_pin,
            dht11_driver,
        }
    }

    pub(crate) fn error_led_off(&mut self) {
        self.dht11_error_pin.set_low().ok();
    }

    pub(crate) fn error_led_on(&mut self) {
        self.dht11_error_pin.set_high().ok();
    }
}

