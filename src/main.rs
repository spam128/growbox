// std and main are not available for bare metal software
// stm32f1xx examples
//https://github.com/stm32-rs/stm32f1xx-hal/tree/master/examples
// esp8266 tutorial
//https://circuitdigest.com/microcontroller-projects/interfacing-esp8266-with-stm32f103c8-stm32-to-create-a-webserver

// blinki tutorial
// https://jonathanklimt.de/electronics/programming/embedded-rust/rust-on-stm32-2/
#![no_std]
#![no_main]

mod env;
mod growbox;
mod heater;
mod temp_sensor;

use cortex_m_rt::entry;
use growbox::GrowboxAPI;

#[allow(unused_imports)]
use panic_halt;


#[entry]
fn main() -> ! {
    let mut growbox_api = GrowboxAPI::new(280, 5);

    loop {
        growbox_api.update_heater();
        growbox_api.delay_ms(1_000_u16);
    }
}
