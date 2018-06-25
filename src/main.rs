#![no_main]
#![no_std]

#[macro_use(entry, exception)]
extern crate cortex_m_rt;
extern crate cortex_m;
extern crate panic_semihosting;

use cortex_m::asm;
use cortex_m_rt::ExceptionFrame;

// the main entry point
entry!(main);
fn main() -> ! {
    loop {
        asm::bkpt();
    }
}

// the hard fault handler
exception!(HardFault, hard_fault);
fn hard_fault(ef: &ExceptionFrame) -> ! {
    panic!("HardFault at {:#?}", ef);
}

// the default exception handler
exception!(*, default_handler);
fn default_handler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}
