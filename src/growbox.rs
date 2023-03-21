// use cortex_m_rt::entry;

// The runtime
// the `set_high/low`function
use stm32f1xx_hal::{delay::Delay, pac, prelude::*};
// STM32F1 specific functions
// the `set_high/low`function
use embedded_hal::digital::v2::OutputPin;
// https://crates.io/crates/dht11
// use dht11::Dht11;

#[allow(unused_imports)]
use panic_halt; // When a panic occurs, stop the microcontroller

use crate::heater::HeaterAPI;
use crate::temp_sensor::DHT11API;

pub(crate) struct GrowboxAPI {
    heater_api: HeaterAPI,
    temp_sensor_api: DHT11API,
    delay: Delay,
    // to store target_temp and temp_variance
    default_target_temp: i16,
    default_temp_variance: i16,
}

impl GrowboxAPI {
    pub fn new(
        // heater_api: HeaterAPI,
        // temp_sensor_api: DHT11API,
        // delay: &mut Delay,

        // diff of temp triggering change, to not to switch heater on/off too often
        // for example max temp = 230, temp_variance = 5,
        // real max temp will be 23,5 celsius degrees
        default_target_temp: i16,
        default_temp_variance: i16,
    ) -> GrowboxAPI {
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
        let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);
        let gpioa = dp.GPIOA.split(&mut rcc.apb2);

        // disable jtag to use pb14
        let mut afio = dp.AFIO.constrain();
        afio.mapr.disable_jtag(gpioa.pa15, gpiob.pb3, gpiob.pb4);
        // #############################################
        // This gives us an exclusive handle to the GPIOC peripheral. To get the
        // handle to a single pin, we need to configure the pin first. Pin C13
        // is usually connected to the Bluepills onboard LED.
        let mut relay_power_supply = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
        relay_power_supply.set_high().ok();

        // let mut heath_pin = gpiob.pb14.into_open_drain_output(&mut gpiob.crh);
        let dht11_pin = gpioc.pc14.into_open_drain_output(&mut gpioc.crh);
        let dht11_error_pin = gpioc.pc15.into_push_pull_output(&mut gpioc.crh);

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
        let delay = Delay::new(cp.SYST, clocks);

        let heather_api = HeaterAPI {
            heath_pin: gpiob.pb14.into_push_pull_output(&mut gpiob.crh),
        };
        let temp_sensor_api = DHT11API::new(dht11_pin, dht11_error_pin);

        GrowboxAPI {
            heater_api: heather_api,
            temp_sensor_api,
            delay,
            default_target_temp,
            default_temp_variance,
        }
    }

    /// check the dht11 sensor and switch heater
    /// if sensor is not working, turn error diode on
    pub fn update_heater(&mut self) {
        match self.is_heat_on() {
            Some(true) => {
                self.heat_on();
            }
            Some(false) => {
                self.heat_off();
            }
            None => (),
        }
    }

    pub fn delay_ms(&mut self, time: u16) {
        self.delay.delay_ms(time)
    }

    /// returns true if heating should be turned on. false fpr off
    /// none if heater state shouldn't be changed
    pub fn is_heat_on(&mut self) -> Option<bool> {
        let temp = self.get_temp();
        match temp {
            Some(result) => {
                if result < self.get_min_temp() {
                    Some(true)
                } else if result > self.get_max_temp() {
                    Some(false)
                } else {
                    None
                }
            }
            None => Some(false),
        }
    }

    /// max temperature, above heater is off
    fn get_max_temp(&self) -> i16 {
        self.default_target_temp + self.default_temp_variance
    }

    /// minimum temperature, below heater is on
    fn get_min_temp(&self) -> i16 {
        self.default_target_temp - self.default_temp_variance
    }

    /// turn heat on
    fn heat_on(&mut self) {
        self.heater_api.heat_on()
    }

    /// turn heat on
    fn heat_off(&mut self) {
        self.heater_api.heat_off()
    }

    ///get temp from dht11
    /// if sensor is not working, return none
    /// and turn on error led
    fn get_temp(&mut self) -> Option<i16> {
        let measurement = self
            .temp_sensor_api
            .dht11_driver
            .perform_measurement(&mut self.delay);
        match measurement {
            Ok(result_measurement) => {
                self.error_led_off();
                Some(result_measurement.temperature)
            }
            Err(_) => {
                self.error_led_on();
                self.heat_off();
                None
            }
        }
    }

    /// turn on error led
    fn error_led_on(&mut self) {
        self.temp_sensor_api.error_led_on()
    }

    /// turn off dht11 error led
    fn error_led_off(&mut self) {
        self.temp_sensor_api.error_led_off()
    }
}
