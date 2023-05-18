#![no_std]
#![no_main]
#![feature(exclusive_range_pattern)]
#![allow(unused, dead_code)]

use cortex_m::delay::Delay;
use defmt::*;
use defmt_rtt as _;
use embedded_hal::{digital::v2::OutputPin, watchdog};
use panic_probe as _;
use rp_pico::{
    entry,
    hal::{
        clocks::{init_clocks_and_plls, Clock},
        gpio::{DynPin, Output, Pin, PinId, PushPull},
        pac,
        watchdog::Watchdog,
        Sio,
    },
    Pins,
};

const PIN_DELAY_LENGTH: u32 = 100;
const CHAR_TABLE: [u8; 63] = [
    // 11101110
    // 00000001
    0x01, // Dot
    0xFC, 0x60, 0xDA, 0xF2, 0x66, 0xB6, 0xBE, 0xE0, 0xFE, 0xF6, // 0-9
    0xEE, 0xFE, 0x9C, 0xF8, 0x9E, 0x8E, 0xBC, 0x6E, 0x0C, 0x70, 0xAE, 0x1C, 0xD4, 0xEC, 0xFC, 0xCE,
    0xD6, 0xDE, 0xB6, 0x8C, 0x7C, 0x74, 0xB8, 0x92, 0x56, 0xDA, // A-Z
    0x32, 0x3E, 0x1A, 0x7A, 0x18, 0x0E, 0x9A, 0x2E, 0x98, 0xB0, 0x96, 0x0C, 0xAA, 0x2A, 0x3A, 0xCE,
    0xE6, 0x0A, 0x30, 0x1E, 0x38, 0x30, 0x54, 0x12, 0x76, 0x12,
];

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let sio = Sio::new(pac.SIO);
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

    let mut delay = Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let pin_bank = Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut pin_a = pin_bank.gpio18.into_push_pull_output();
    let mut pin_b = pin_bank.gpio19.into_push_pull_output();
    let mut pin_c = pin_bank.gpio13.into_push_pull_output();
    let mut pin_d = pin_bank.gpio14.into_push_pull_output();
    let mut pin_e = pin_bank.gpio15.into_push_pull_output();
    let mut pin_f = pin_bank.gpio17.into_push_pull_output();
    let mut pin_g = pin_bank.gpio16.into_push_pull_output();
    let mut pin_dp = pin_bank.gpio12.into_push_pull_output();

    let mut pin_array: [DynPin; 8] = [
        pin_dp.into(),
        pin_g.into(),
        pin_f.into(),
        pin_e.into(),
        pin_d.into(),
        pin_c.into(),
        pin_b.into(),
        pin_a.into(),
    ];

    for pin in pin_array.iter_mut() {
        pin.set_low();
    }

    loop {
        // toggle_pin(&mut delay, &mut pin_a);
        // toggle_pin(&mut delay, &mut pin_b);
        // toggle_pin(&mut delay, &mut pin_c);

        // toggle_pin(&mut delay, &mut pin_d);
        // toggle_pin(&mut delay, &mut pin_e);
        // toggle_pin(&mut delay, &mut pin_f);

        display_string("Hello World.", &mut pin_array, &mut delay, 500);
    }
}

fn toggle_pin<I: PinId>(delay: &mut Delay, pin: &mut Pin<I, Output<PushPull>>) {
    pin.set_high();
    delay.delay_ms(PIN_DELAY_LENGTH);
    pin.set_low();
}

fn display_string(str: &str, pins: &mut [DynPin; 8], delay: &mut Delay, delay_time: u32) {
    for str_char in str.as_bytes() {
        let char = convert_char_to_table_item(str_char);
        display_char(&char, pins, delay, delay_time)
    }
}
fn convert_char_to_table_item(char: &u8) -> u8 {
    let idx = match char {
        0x30..0x40 => char - 0x2F,
        0x41..0x5B => char - 0x36,
        0x61..0x7B => char - 0x3C,
        _ => 0,
    };
    let conv_idx = usize::try_from(idx).unwrap();
    CHAR_TABLE[conv_idx]
}
fn display_char(char: &u8, pins: &mut [DynPin; 8], delay: &mut Delay, delay_time: u32) {
    for (idx, pin) in pins.iter_mut().enumerate() {
        match (char >> idx) & 1 {
            0x1 => pin.set_high(),
            _ => pin.set_low(),
        };
    }
    delay.delay_ms(delay_time);
}
