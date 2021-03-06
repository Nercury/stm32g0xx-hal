#![deny(warnings)]
#![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate nb;
extern crate panic_semihosting;
#[macro_use]
extern crate stm32g0xx_hal as hal;

use core::fmt::Write;

use hal::prelude::*;
use hal::serial::Config;
use hal::stm32;
use rt::entry;

#[entry]
fn main() -> ! {
    hal::debug::init();

    let dp = stm32::Peripherals::take().expect("cannot take peripherals");
    let mut rcc = dp.RCC.constrain();
    let gpio = dp.GPIOC.split(&mut rcc);
    let mut usart = dp
        .USART1
        .usart(gpio.pc4, gpio.pc5, Config::default(), &mut rcc)
        .unwrap();

    write!(usart, "Hello\r\n").unwrap();

    let mut cnt = 0;
    loop {
        let byte = block!(usart.read()).unwrap();
        write!(usart, "{}: {}\r\n", cnt, byte).unwrap();
        cnt += 1;
    }
}
