//! examples/init.rs

#![deny(unsafe_code)]
#![no_main]
#![no_std]

use rtic::app;
use panic_rtt_target as _;

#[app(device = stm32f1xx_hal::stm32)]
mod app {
    use rtt_target::{rprintln, rtt_init_print};
    use stm32f1xx_hal::{pac, prelude::*};

    const CLOCK_MHZ: u32 = 72;

    #[init]
    fn init(cx: init::Context) -> init::LateResources {
        // Cortex-M peripherals (Core Peripherals)
        let mut cp = cx.core;

        // Device specific peripherals (Peripheral Access Crate)
        let pac = cx.device;

        // Setup RTT for logging
        rtt_init_print!();
        rprintln!("Hello, world!");

        // Clock setup
        let mut flash = pac.FLASH.constrain();
        let mut rcc = pac.RCC.constrain();
        let clocks = rcc
            .cfgr
            .use_hse(8.mhz())
            .sysclk(CLOCK_MHZ.mhz())
            .pclk1(24.mhz())
            .freeze(&mut flash.acr);

        // Initialize (enable) the monotonic timer (CYCCNT)
        cp.DCB.enable_trace();
        cp.DWT.enable_cycle_counter();

        init::LateResources{}
    }

    #[idle()]
    fn idle(mut cx: idle::Context) -> ! {
        loop{}
    }

}
