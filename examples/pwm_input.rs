#![deny(warnings)]
#![no_main]
#![no_std]

//use cortex_m::asm::delay;
use cortex_m_rt::entry;
#[macro_use]
mod utilities;
use stm32h7xx_hal::{pac, prelude::*};
use stm32h7xx_hal::{pac::TIM1};

use log::info;

#[entry]
fn main() -> ! {
    utilities::logger::init();
    let dp = pac::Peripherals::take().expect("Cannot take peripherals");

    info!("");
    info!("stm32h7xx-hal example - PWM");
    info!("");

    // Constrain and Freeze power
    info!("Setup PWR...");
    let pwr = dp.PWR.constrain();
    let pwrcfg = example_power!(pwr).freeze();

    // Constrain and Freeze clock
    info!("Setup RCC...");
    let rcc = dp.RCC.constrain();
    let _ccdr = rcc.sys_ck(400.mhz()).freeze(pwrcfg, &dp.SYSCFG);

    //  RM0433, page 1572
    dp.TIM1.ccmr1_input().write(|w| unsafe {
        w
            .cc1s()     // 2 - TI1 selected
            .bits(0b01)

            .cc2s()     // 4 - TI1 selected
            .bits(0b01)
    });

    dp.TIM1.ccer.write(|w| {
        w
            .cc1p()     // 3 - active on rising edge
            .clear_bit()
            .cc1np()
            .clear_bit()

            .cc2p()     // 5 - active on falling edge
            .set_bit()
            .cc2np()
            .clear_bit()

            .cc1e()     // 8 - enable captures
            .set_bit()
            .cc2e()
            .set_bit()
    });

    dp.TIM1.smcr.write( |w| unsafe {
        w
            .ts().bits(0b00101) // 6 - TI1FP1 selected
            .sms().bits(0b0100) // 7 - ???
    });

    loop {
        cortex_m::asm::delay(100000000);
        let ccr1 = unsafe { (*TIM1::ptr()).ccr1.read().bits() as u16};
        let ccr2 = unsafe { (*TIM1::ptr()).ccr2.read().bits() as u16};

        // All zeros ...
        info!("ccr1: {}", ccr1);
        info!("ccr2: {}", ccr2);
    }
}
