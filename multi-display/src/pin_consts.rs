pub const CHAR_TABLE: [u8; 11] = [
    0x01, // Dot
    0xFC, 0x60, 0xDA, 0xF2, 0x66, 0xB6, 0xBE, 0xE0, 0xFE, 0xF6, // 0-9
];

pub fn num_to_led_char(num: u8) -> u8 {
    match num {
        0..10 => CHAR_TABLE[usize::try_from(num + 1).unwrap()],
        _ => CHAR_TABLE[0],
    }
}
