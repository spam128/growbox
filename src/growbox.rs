#![no_std]
#![no_main]

use heater::HeaterAPI;
use temp_sensor::DHT11API;

struct Growbox {
    heater_api: HeaterAPI,
    temp_sensor_api: DHT11API,

    // to store target_temp and temp_variance
    extra_fields: HashMap<String, i16>,
}

impl Growbox {
    fn new(
        heater_api: HeaterAPI,
        temp_sensor_api: DHT11API,
        default_target_temp: i16,
        default_temp_variance: i16,
    ) -> Self {
        let extra_fields = HashMap::new();
        extra_fields.insert("target_temp".to_strint(), default_target_temp);
        // diff of temp triggering change, to not to switch heater on/off too often
        // for example max temp = 230, temp_variance = 5,
        // real max temp will be 23,5 celsius degrees
        extra_fields.insert("temp_variance".to_strint(), default_temp_variance);
        Growbox {
            heater_api,
            temp_sensor_api,
            extra_fields,
        }
    }

    pub fn update_heater(&self) {
        /// check the dht11 sensor and switch heater
        /// if sensor is not working, turn error diode on
        match self.is_heat_on() {
            Ok(new_status) => {
                self.error_led_off();
                match new_status {
                    // match new status, if its none, don't switch heater
                    Some(true) => self.heat_on(),
                    Some(false) => self.heat_off()
                }
            }
            Err(e) => self.error_led_on()
        }
        self.heat_on().ok
    }

    fn set_target_temp(&mut self, value: i16) {
        self.extra_fields.insert("target_temp".to_string(), value)
    }

    fn get_target_temp(&self) {
        self.extra_fields.get("target_temp".to_string())
    }

    fn set_temp_variance(&mut self, value: i16) {
        self.extra_fields.insert("temp_variance".to_string(), value)
    }

    fn get_temp_variance(&self) {
        self.extra_fields.get("temp_variance".to_string())
    }

    fn get_max_temp(&self) {
        /// max temperature, above heater is off
        self.get_target_temp() + self.get_temp_variance()
    }

    fn get_min_temp(&self) {
        /// minimum temperature, below heater is on
        self.get_target_temp() - self.get_temp_variance()
    }


    fn is_heat_on(&self) -> Option<bool> {
        /// returns true if heating should be turned on
        measurement = self.temp_sensor_api.get_measurement();
        if measurement < self.get_min_temp() {
            Some(true)
        } else if measurement > self.get_max_temp() {
            Some(false)
        }
    }

    fn heat_on(&self) {
        /// turn heat on
        self.heater_api.heat_on()
    }

    fn get_temp(&self) {
        ///get temp from dht11
        self.temp_sensor_api.get_temp()
    }

    fn error_led_on(&self) {
        /// turn on error led
        self.temp_sensor_api.error_led_on()
    }

    fn error_led_off(&self) {
        /// turn off dht11 error led
        self.temp_sensor_api.error_led_off()
    }
}
