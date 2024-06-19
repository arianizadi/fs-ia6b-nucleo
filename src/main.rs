#![no_std]
#![no_main]

// use cortex_m_semihosting::hprintln;
// pick a panicking behavior
use panic_halt as _;
use stm32f4xx_hal::uart::Config;
// use stm32f4xx_hal::uart::Serial; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use core::fmt::Write;
use cortex_m_rt::entry;
use stm32f4xx_hal::pac::Peripherals;
use stm32f4xx_hal::prelude::*;

#[entry]
fn main() -> ! {
    let dp = Peripherals::take().expect("cannot take peripherals");

    let gpioa = dp.GPIOA.split();

    let rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.use_hse(8.MHz()).freeze();

    let mut delay = dp.TIM1.delay_ms(&clocks);

    // define RX/TX pins for usb serial
    let tx_pin = gpioa.pa2.into_alternate();

    // configure serial

    // use the USART2 peripheral
    let mut tx = dp
        .USART2
        .tx(tx_pin, Config::default().baudrate(9600.bps()), &clocks)
        .unwrap();

    let mut value: u8 = 0;

    let mut led = gpioa.pa8.into_push_pull_output();

    led.set_low();

    loop {
        led.toggle();
        // print some value every 500 ms, value will overflow after 255
        writeln!(tx, "value: {value:02}\r").unwrap();
        // transmit the value
        value = value.wrapping_add(1);
        delay.delay(250.millis());
    }
}
