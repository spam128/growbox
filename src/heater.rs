use embedded_hal::digital::v2::OutputPin;
use stm32f1xx_hal::{gpio};

pub struct HeaterAPI {
    // pin to which heather is connected
    // by setting high, heater is on, low is off
    pub heath_pin: gpio::gpiob::PB14<gpio::Output<gpio::OpenDrain>>,
}

impl HeaterAPI {
    pub(crate) fn heat_on(&mut self) { self.heath_pin.set_high().ok(); }

    pub(crate) fn heat_off(&mut self) {
        self.heath_pin.set_low().ok();
    }
}
