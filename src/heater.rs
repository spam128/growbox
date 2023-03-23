use stm32f1xx_hal::gpio;

pub struct HeaterAPI {
    // pin to which heather is connected
    // by setting high, heater is on, low is off
    pub heath_pin: gpio::gpiob::PB14<gpio::Output<gpio::PushPull>>,
}

impl HeaterAPI {
    /// its not a bug, while stm32 connected to relay on PushPull mode, it turn off led with high
    pub(crate) fn heat_off(&mut self) {
        self.heath_pin.set_high();
    }

    /// its not a bug, while stm32 connected to relay on PushPull mode, it turn on led with low
    pub(crate) fn heat_on(&mut self) {
        self.heath_pin.set_low();
    }
}
