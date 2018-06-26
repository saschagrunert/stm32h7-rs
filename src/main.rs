#![feature(lang_items)]
#![no_main]
#![no_std]

#[macro_use(entry, exception)]
extern crate cortex_m_rt;
extern crate cortex_m;
extern crate panic_semihosting;
extern crate stm32h7xx;

use cortex_m::asm;
use cortex_m_rt::ExceptionFrame;
use stm32h7xx::App;

// The main entry point
entry!(main);
fn main() -> ! {
    // Create a new application instance
    match App::new() {
        Ok(mut app) => {
            // Run the application
            if let Err(e) = app.run() {
                panic!("Unable to run app: {:?}", e);
            }
        }
        Err(e) => panic!("Unable to create app: {:?}", e),
    };

    unreachable!();
}

#[lang = "oom"]
#[no_mangle]
// The out of memory handler
pub fn rust_oom() -> ! {
    loop {
        asm::bkpt();
    }
}

// the hard fault handler
exception!(HardFault, hard_fault);
fn hard_fault(ef: &ExceptionFrame) -> ! {
    panic!("HardFault at {:#?}", ef);
}

// The default exception handler
exception!(*, default_handler);
fn default_handler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}
