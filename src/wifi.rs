use nb::block;
use stm32f1xx_hal::timer::delay::SysDelay;
use stm32f1xx_hal::timer::*;
use unwrap_infallible::UnwrapInfallible;

use stm32f1xx_hal::{pac, prelude::*, serial::{Serial, Config}, gpio};
use crate::env;


const WIFI_SSID: &[u8] = env::WIFI_SSID.as_bytes();
const WIFI_PASSWORD: &[u8] = env::WIFI_PASSWORD.as_bytes();
const RESET_ESP8266: &[u8] = b"AT\r\n";
const SET_ESP8266_MODE: &[u8] = b"AT+CWMODE=3\r\n";
const DISCONNECT_WIFI_AP: &[u8] = b"AT+CWQAP\r\n";
const RESTART_ESP8266: &[u8] = b"AT+RST\r\n";

pub fn main() {
    let dp = pac::Peripherals::take().unwrap();
    let rcc = dp.RCC.constrain();
    let mut afio = dp.AFIO.constrain();
    let mut gpioa = dp.GPIOA.split();
    let mut gpioc = dp.GPIOC.split();
    let mut flash = dp.FLASH.constrain();

    // frequency should be set as multiple
    // of 1,8432MHz for USART2
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let cp = cortex_m::Peripherals::take().unwrap();
    let delay = cp.SYST.delay(&clocks);

    // transmitter
    let tx_pin = gpioa.pa2.into_alternate_push_pull(&mut gpioa.crl);
    // receiver
    let rx_pin = gpioa.pa3.into_floating_input(&mut gpioa.crl);
    // transmission rate
    // calculated 12.22
    // let baudrate = 57600.bps();
    let baudrate = 115200.bps();
    // let baudrate_command = b"AT+UART_CUR=115200,8,1,0,0\r\n";

    let p = gpioc.pc15.into_push_pull_output(&mut gpioc.crh);


    let wifi_serial = Serial::new(
        dp.USART2,
        (tx_pin, rx_pin),
        &mut afio.mapr,
        Config::default()
            .baudrate(baudrate)
            .wordlength_9bits()
            .parity_none(),
        &clocks,
    );
    // Separate into tx and rx channels
    let (mut wifi_tx, _) = wifi_serial.split();
    //wait for startup 8s
    // blink(&mut delay, &mut p, Some(4_000_u32));

    block!(wifi_tx.write_u16(0x1FF)).unwrap_infallible();
    block!(wifi_tx.write(b'R')).unwrap_infallible();

    let mut wifi = ESP8266API { tx: wifi_tx, blink, delay, led: p };

    wifi.at_command(&RESET_ESP8266);
    wifi.at_command(&SET_ESP8266_MODE);
    wifi.at_command(&DISCONNECT_WIFI_AP);
    wifi.at_command(&RESTART_ESP8266);
}

struct ESP8266API {
    tx: stm32f1xx_hal::serial::Tx<pac::USART2>,
    blink: fn(&mut SysDelay, &mut gpio::gpioc::PC15<gpio::Output<gpio::PushPull>>, Option<u32>) -> (),
    delay: SysDelay,
    led: gpio::gpioc::PC15<gpio::Output<gpio::PushPull>>,

}

impl ESP8266API {
    /// https://circuitdigest.com/microcontroller-projects/interfacing-esp8266-with-stm32f103c8-stm32-to-create-a-webserver
    /// https://www.youtube.com/watch?v=Sd7xE52zL5U
    ///!!!!!! https://www.youtube.com/watch?v=-p0arb42OsI&t=25s
    /// !! https://www.youtube.com/watch?v=uuIlcX99yOs
    /// examples: https://github.com/stm32-rs/stm32f1xx-hal/tree/master/examples
    /// https://www.youtube.com/watch?v=o_alVYMBBco&list=PLL2SCPK5xSRWBPj-nKOVYIhxRw7C4kYeI
    /// https://circuitdigest.com/microcontroller-projects/interfacing-esp8266-with-stm32f103c8-stm32-to-create-a-webserver
    fn at_command(&mut self, command: &[u8]) {
        for byte in command {
            match block!(self.tx.write(*byte)) {
                Ok(_) => {}
                Err(_) => (self.blink)(&mut self.delay, &mut self.led, Some(1000_u32)),
            }
        }
    }
}

fn blink(
    delay: &mut SysDelay,
    led: &mut gpio::gpioc::PC15<gpio::Output<gpio::PushPull>>,
    time_ms: Option<u32>)
{
    led.set_high();
    delay.delay_ms(time_ms.unwrap_or(50_u32));
    led.set_low();
    delay.delay_ms(time_ms.unwrap_or(50_u32));
}


/*
use nb::block;
use stm32f1xx_hal::timer::delay::SysDelay;
use stm32f1xx_hal::timer::*;
use stm32f1xx_hal::{
    pac, prelude::*, serial::{Config, Serial}, gpio,
};

use crate::env;

const WIFI_SSID: &[u8] = env::WIFI_SSID.as_bytes();
const WIFI_PASSWORD: &[u8] = env::WIFI_PASSWORD.as_bytes();

fn blink<T>(delay: &mut SysDelay, led: &mut gpio::gpioc::PC13<gpio::Output<gpio::PushPull>>, time_ms: Option<T>)
where
    T: Into<MicroSeconds>,
{
    led.set_high().unwrap();
    delay.delay_us(time_ms.unwrap_or(50_u32).us());
    led.set_low().unwrap();
    delay.delay_us(time_ms.unwrap_or(50_u32).us());
}

pub fn main() {
    let dp = pac::Peripherals::take().unwrap();
    let rcc = dp.RCC.constrain();
    let mut afio = dp.AFIO.constrain();
    let mut gpioa = dp.GPIOA.split(&mut rcc.apb2);
    let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);
    let mut gpioc = dp.GPIOC.split(&mut rcc.apb2);

    let clocks = rcc.cfgr.freeze(&mut dp.FLASH.constrain().acr);

    let tx = gpioa.pa2.into_alternate_push_pull(&mut gpioa.crl);
    let rx = gpioa.pa3;

    let mut serial = Serial::usart2(
        dp.USART2,
        (tx, rx),
        &mut afio.mapr,
        Config::default().baudrate(115200.bps()),
        clocks,
        &mut rcc.apb1,
    );

    let mut delay = Delay::new(cp.SYST, clocks);
    let mut p = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

    block!(tx.write_all(b"AT+RST\r\n")).unwrap();
    blink(&mut delay, &mut p, Some(100_u32));

    block!(tx.write_all(b"AT+CWMODE=3\r\n")).unwrap();
    blink(&mut delay, &mut p, Some(100_u32));

    block!(tx.write_all(b"AT+CWJAP=\"")).unwrap();
    block!(tx.write_all(WIFI_SSID)).unwrap();
    block!(tx.write_all(b"\",\"")).unwrap();
    block!(tx.write_all(WIFI_PASSWORD)).unwrap();
    block!(tx.write_all(b"\"\r\n")).unwrap();
    blink(&mut delay, &mut p, Some(100_u32));

    loop {
        blink(&mut delay, &mut p, Some(1000_u32));
    }
}

 */