#![no_std]
#![no_main]
#![feature(exclusive_range_pattern)]
#![allow(unused, dead_code)]

mod led_control;
mod pin_consts;

extern crate alloc;
use core::{cell::RefCell, mem::MaybeUninit};

use alloc::rc::Rc;
use bsp::{entry, pac::Peripherals};
use cortex_m::delay::Delay;
use defmt::*;
use defmt_rtt as _;
use embedded_alloc::Heap;
use embedded_hal::digital::v2::OutputPin;
use led_control::DisplayController;
use panic_probe as _;

use rp_pico as bsp;

use bsp::hal::{
    clocks::{init_clocks_and_plls, Clock},
    pac,
    sio::Sio,
    watchdog::Watchdog,
};

#[global_allocator]
static HEAP: Heap = Heap::empty();

#[entry]
fn main() -> ! {
    {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 2048;
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
    }
    let (mut controller, delay) = setup();
    loop {
        controller.show_number(1);
    }
}

fn setup() -> (DisplayController, Rc<RefCell<Delay>>) {
    let mut pac = pac::Peripherals::take().unwrap();

    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let external_xtal_freq_hz = 12_000_000u32;
    let clocks = init_clocks_and_plls(
        external_xtal_freq_hz,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();
    let delay = Rc::new(RefCell::new(Delay::new(
        core.SYST,
        clocks.system_clock.freq().to_Hz(),
    )));
    let sio = Sio::new(pac.SIO);

    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut digit_1 = pins.gpio12.into_push_pull_output();
    let mut digit_2 = pins.gpio14.into_push_pull_output();
    let mut digit_3 = pins.gpio13.into_push_pull_output();
    let mut digit_4 = pins.gpio15.into_push_pull_output();

    let mut ser = pins.gpio19.into_push_pull_output();
    let mut rclock = pins.gpio18.into_push_pull_output();
    let mut srclock = pins.gpio17.into_push_pull_output();

    (
        DisplayController::new(
            digit_1.into(),
            digit_2.into(),
            digit_3.into(),
            digit_4.into(),
            srclock.into(),
            ser.into(),
            rclock.into(),
            Rc::clone(&delay),
        ),
        delay,
    )
}

// End of file
