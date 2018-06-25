//! The main library interface
#![deny(missing_docs)]
#![feature(alloc)]
#![feature(lang_items)]
#![no_std]

#[macro_use]
extern crate alloc;
extern crate alloc_cortex_m;
extern crate cortex_m;
extern crate cortex_m_rt;
extern crate cortex_m_semihosting;
extern crate panic_semihosting;

mod error;

use alloc_cortex_m::CortexMHeap;
use core::fmt::Write;
use cortex_m::asm;
use cortex_m_rt::heap_start;
use cortex_m_semihosting::hio;
use error::Error;

// this is the allocator the application will use
#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

/// The Application state
pub struct App;

impl App {
    /// Run the main application
    pub fn run() -> Result<(), Error> {
        // Initialize the allocator before usage
        unsafe { ALLOCATOR.init(heap_start() as usize, 1024) }

        // Growable array allocated on the heap
        let xs = vec![0, 1, 2];

        let mut stdout = hio::hstdout()?;
        writeln!(stdout, "{:?}", xs)?;

        loop {
            asm::bkpt();
        }
    }
}
