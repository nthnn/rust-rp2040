#![no_std]
#![no_main]

extern crate cortex_m;
extern crate embedded_hal;
extern crate panic_halt;
extern crate rp2040_hal;

#[allow(unused_imports)]
use panic_halt as _;
#[warn(unused_imports)]

use rp2040_hal::pac;

#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_GENERIC_03H;

mod program;

#[rp2040_hal::entry]
fn main() -> ! {
    let pac: pac::Peripherals = pac::Peripherals::take().unwrap();
    let core: pac::CorePeripherals = pac::CorePeripherals::take().unwrap();

    program::setup(pac, core);
    loop { program::cycle(); }
}
