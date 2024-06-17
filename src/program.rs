use cortex_m::delay::Delay;
use embedded_hal::digital::v2::OutputPin;

use rp2040_hal::gpio::bank0::Gpio25;
use rp2040_hal::clocks::{Clock, ClocksManager};
use rp2040_hal::gpio::{Output, Pin, Pins, PushPull};
use rp2040_hal::{pac, Sio, Watchdog};

const XTAL_FREQ_HZ: u32 = 12_000_000u32;

static mut DELAY: Option<Delay> = None;
static mut PIN: Option<Pin<Gpio25, Output<PushPull>>> = None;

pub fn setup(mut paccess: pac::Peripherals, core: pac::CorePeripherals) -> () {
    let mut watchdog: Watchdog = rp2040_hal::Watchdog::new(paccess.WATCHDOG);
    let clocks: ClocksManager = rp2040_hal::clocks::init_clocks_and_plls(
        XTAL_FREQ_HZ,
        paccess.XOSC,
        paccess.CLOCKS,
        paccess.PLL_SYS,
        paccess.PLL_USB,
        &mut paccess.RESETS,
        &mut watchdog,
    )
        .ok()
        .unwrap();

    let sio: Sio = Sio::new(paccess.SIO);
    let pins: Pins = rp2040_hal::gpio::Pins::new(
        paccess.IO_BANK0,
        paccess.PADS_BANK0,
        sio.gpio_bank0,
        &mut paccess.RESETS,
    );

    unsafe {
        DELAY = Some(Delay::new(
            core.SYST,
            clocks.system_clock
                .freq()
                .to_Hz()
            ));
        PIN = Some(pins.gpio25.into_push_pull_output());
    }
}

pub fn cycle() -> () {
    let delay: &mut Delay =
        unsafe { DELAY.as_mut().unwrap() };
    let pin: &mut Pin<Gpio25, Output<PushPull>> =
        unsafe { PIN.as_mut().unwrap() };

    pin.set_high().unwrap();
    delay.delay_ms(1000);
    pin.set_low().unwrap();
    delay.delay_ms(1000);
}
