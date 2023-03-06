use dht11::Dht11;
use embedded_hal::digital::v2::OutputPin;
use stm32f1xx_hal::{delay::Delay, gpio, pac, prelude::*};

pub struct HeaterAPI {
    // pin to which heather is connected
    // by setting high, heater is on, low is off
    pub heath_pin: gpio::gpiob::PB14<gpio::Output<gpio::OpenDrain>>,
}

impl HeaterAPI {
    fn heat_on(&self) {
        self.heath_pin.set_high().ok();
    }
    fn heat_off(&self) {
        self.heath_pin.set_low().ok();
    }
}
