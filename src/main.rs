#![no_main]
#![no_std]

extern crate cortex_m;

#[macro_use(entry, exception)]
extern crate cortex_m_rt as rt;
extern crate panic_semihosting;

use cortex_m::asm;
use rt::ExceptionFrame;

entry!(main);
fn main() -> ! {
    loop {
        asm::bkpt();
    }
}

// define the hard fault handler
exception!(HardFault, hard_fault);

fn hard_fault(ef: &ExceptionFrame) -> ! {
    panic!("HardFault at {:#?}", ef);
}

// define the default exception handler
exception!(*, default_handler);

fn default_handler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}
