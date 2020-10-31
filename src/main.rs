
#![deny(unsafe_code)]   //  Don't allow unsafe code in this file.
#![deny(warnings)]      //  If the Rust compiler generates a warning, stop the compilation with an error.
#![no_main]             //  Don't use the Rust standard bootstrap. We will provide our own.
#![no_std]              //  Don't use the Rust standard library. We are building a binary that can run on its own.

use cortex_m_rt::{entry, exception, ExceptionFrame};    //  Stack frame for exception handling.
use cortex_m_semihosting::hprintln;                     //  For displaying messages on the debug console.
use panic_semihosting as _;

use nb::block;

use stm32f1xx_hal::{pac, prelude::*, serial::{Config, Serial}};

#[entry]
fn main() -> ! {
    hprintln!("Hello, world!").unwrap();

    // Core peripherals
    //let cp = cortex_m::Peripherals::take().unwrap();

    // Device peripherals
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut afio = dp.AFIO.constrain(&mut rcc.apb2);
    let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);

    let tx = gpiob.pb10.into_alternate_push_pull(&mut gpiob.crh);
    let rx = gpiob.pb11;

    let mut serial = Serial::usart3(
        dp.USART3, 
        (tx, rx), 
        &mut afio.mapr, 
        Config::default().baudrate(9600.bps()), 
        clocks, 
        &mut rcc.apb1,
    ); 

    loop {
        for c in "Hello Rust!\n".bytes() {
            block!(serial.write(c)).ok();    
        }
    }
}

#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("Hard fault: {:#?}", ef);
}

#[exception]
fn DefaultHandler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}

