use dht11::Dht11;
use embedded_hal::digital::v2::OutputPin;
use stm32f1xx_hal::{delay::Delay, pac, prelude::*, gpio};


pub struct DHT11API {
    pub dht11_pin: gpio::gpioc::PC14<gpio::Output<gpio::OpenDrain>>,
    pub dht11_error_pin: gpio::gpioc::PC15<gpio::Output<gpio::PushPull>>,
    pub dht11_driver: Dht11,
}

impl DHT11API {
    fn new(
        dht11_pin: gpio::gpioc::PC14<gpio::Output<gpio::OpenDrain>>,
        dht11_error_pin: gpio::gpioc::PC15<gpio::Output<gpio::PushPull>>) -> Self {
        let mut dht11_driver = Dht11::new(dht11_pin);
        DHT11API {
            dht11_pin,
            dht11_error_pin,
            dht11_driver,
        }
    }

    fn get_measurement(&self) {
        dht11_driver.perform_measurement(&mut delay)
    }

    fn error_led_off(&mut self) {
        self.dht11_error_pin.set_low().ok();
    }

    fn error_led_on(&mut self) {
        self.dht11_error_pin.set_high().ok();
    }
}

