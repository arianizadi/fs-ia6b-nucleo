#![no_std]
#![no_main]

use panic_halt as _;
use stm32f4xx_hal::uart::Config;
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

    let tx_pin = gpioa.pa2.into_alternate();

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
