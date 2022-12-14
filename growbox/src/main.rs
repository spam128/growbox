// std and main are not available for bare metal software
#![no_std]
#![no_main]

use cortex_m_rt::entry; // The runtime
use embedded_hal::digital::v2::OutputPin; // the `set_high/low`function
use stm32f1xx_hal::{delay::Delay, pac, prelude::*}; // STM32F1 specific functions
use lcd1602_rs::LCD1602; // lcd https://crates.io/crates/lcd1602-rs

#[allow(unused_imports)]
use panic_halt; // When a panic occurs, stop the microcontroller

// This marks the entrypoint of our application. The cortex_m_rt creates some
// startup code before this, but we don't need to worry about this
#[entry]
fn main() -> ! {
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
    // This gives us an exclusive handle to the GPIOC peripheral. To get the
    // handle to a single pin, we need to configure the pin first. Pin C13
    // is usually connected to the Bluepills onboard LED.
    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

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
    //===========================
    // https://github.com/levpaul/lcd1602-rs/blob/92b348f76fd1d8611527d5fb21580e2bb7230dba/examples/teensy40/src/main.rs
    // LCD controller
    let mut p = bsp::Peripherals::take().unwrap();
    let pins = bsp::t40::into_pins(p.iomuxc);
    let mut led = bsp::configure_led(pins.p13);

    // Init pins
    let rs = gpio::GPIO::new(pins.p12).output();
    let en = gpio::GPIO::new(pins.p11).output();
    let d4 = gpio::GPIO::new(pins.p5).output();
    let d5 = gpio::GPIO::new(pins.p4).output();
    let d6 = gpio::GPIO::new(pins.p3).output();
    let d7 = gpio::GPIO::new(pins.p2).output();

    // General Purpose Timer setup
    let (_, ipg_hz) =
        p.ccm
            .pll1
            .set_arm_clock(imxrt_hal::ccm::PLL1::ARM_HZ, &mut p.ccm.handle, &mut p.dcdc);
    let mut cfg = p.ccm.perclk.configure(
        &mut p.ccm.handle,
        imxrt_hal::ccm::perclk::PODF::DIVIDE_3,
        imxrt_hal::ccm::perclk::CLKSEL::IPG(ipg_hz),
    );
    let mut gpt1 = p.gpt1.clock(&mut cfg);
    gpt1.set_output_interrupt_on_compare(imxrt_hal::gpt::OutputCompareRegister::Three, false);
    gpt1.set_mode(imxrt_hal::gpt::Mode::FreeRunning);
    gpt1.set_reset_on_enable(true);
    gpt1.set_enable(true);
    let t = gpt1.count_down(imxrt_hal::gpt::OutputCompareRegister::Three);

    // LCD Init
    let mut lcd = LCD1602::new(en, rs, d4, d5, d6, d7, t).unwrap();
    // =================================


    // Now, enjoy the lightshow!
    loop {
        // lcd display
        lcd.print("hello world!").ok();
        led.toggle();
        lcd.delay(1_000_000 as u64).ok();
        led.toggle();
        lcd.clear().ok();
        lcd.delay(1_000_000 as u64).ok();

        // led on micro
        delay.delay_ms(1_000_u16);
        led.set_low().ok();
        delay.delay_ms(1_000_u16);
    }
}