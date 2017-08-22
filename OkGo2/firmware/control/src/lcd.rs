use bare_metal::CriticalSection;
use firmware_common::utils::{delay_ms, delay_us};

output!(LCD_RS, A, 15);
output!(LCD_E, B, 3);
output!(LCD_DB4, B, 4);
output!(LCD_DB5, B, 5);
output!(LCD_DB6, B, 6);
output!(LCD_DB7, B, 7);

/// Initialise the LCD with appropriate initialisation
pub fn init(cs: &CriticalSection) {
    LCD_RS.setup(cs);
    LCD_E.setup(cs);
    LCD_DB4.setup(cs);
    LCD_DB5.setup(cs);
    LCD_DB6.setup(cs);
    LCD_DB7.setup(cs);

    // Set LCD pins to idle state
    LCD_RS.clear(cs);
    LCD_E.clear(cs);

    // Wait for things to warm up: LCD wants 40ms guaranteed from 5V startup
    delay_ms(60);

    // Set 4-bit mode
    _hd44780_write_nibble(cs, 0x03, true);
    delay_ms(6);
    _hd44780_write_nibble(cs, 0x03, true);
    delay_us(200);
    _hd44780_write_nibble(cs, 0x03, true); // yes, really.
    delay_us(50);
    _hd44780_write_nibble(cs, 0x02, true);

    // Set small font (F=0), 2-line mode (N=1)
    _hd44780_write(cs, (1 << 5) | (1 << 3), true);
    delay_us(50);

    // Display on*/
    _hd44780_write(cs, (1 << 3), true);
    delay_us(50);

    clear(cs);

    // Entry point: I/D=1 (left-to-right script), SH=0 (no scroll)
    _hd44780_write(cs, (1 << 2) | (1 << 1), true);
    delay_us(50);

    // Display on, cursor off, cursor blink off
    config(cs, true, false, false);
    delay_us(50);
    cursor_pos(cs, 0, 0);
    delay_us(50);
}

pub fn clear(cs: &CriticalSection) {
    _hd44780_write(cs, 0x01, true);
    delay_ms(2);
}

/// A variety of common runtime config options
pub fn config(cs: &CriticalSection, display_active: bool, cursor_on: bool, cursor_blink: bool) {
    let mut content = 0x08;

    if display_active {
        content |= 1 << 2;
    }
    if cursor_on {
        content |= 1 << 1;
    }
    if cursor_blink {
        content |= 1;
    }

    _hd44780_write(cs, content, true);
}

/// Move the LCD cursor to specified row and col, counting from top left 0,0
pub fn cursor_pos(cs: &CriticalSection, row: u8, col: u8) {
    const ROW_OFFSETS: [u8; 4] = [0x00, 0x40, 0x14, 0x54];
    let mut row = row;
    let mut col = col;

    if row > 3 {
        row = 0;
    }
    if col > 39 {
        col = 0;
    }
    let address = ROW_OFFSETS[row as usize] + col;
    _hd44780_write(cs, (1 << 7) | address, true);
    delay_us(50);
}

/// Write a single character to the LCD
pub fn putc(cs: &CriticalSection, c: u8) {
    _hd44780_write(cs, c, false);
    delay_us(50);
}

/// Write a null-terminated string to the LCD
pub fn puts(cs: &CriticalSection, s: &str) {
    for b in s.bytes() {
        putc(cs, b);
    }
}

/// Write a command or data byte to the HD44780
fn _hd44780_write(cs: &CriticalSection, content: u8, command: bool) {
    _hd44780_write_nibble(cs, (content >> 4) & 0x0F, command);
    delay_us(10);
    _hd44780_write_nibble(cs, content & 0x0F, command);
    delay_us(10);
}

/// Write half of command or data byte to the HD44780
fn _hd44780_write_nibble(cs: &CriticalSection, content: u8, command: bool) {
    LCD_RS.set_bool(cs, !command);
    delay_us(1);
    LCD_E.set(cs);
    delay_us(1);

    LCD_DB4.set_bool(cs, content & (1 << 0) != 0);
    LCD_DB5.set_bool(cs, content & (1 << 1) != 0);
    LCD_DB6.set_bool(cs, content & (1 << 2) != 0);
    LCD_DB7.set_bool(cs, content & (1 << 3) != 0);

    // This delay should only need to be 1us or so, but this doesn't work.
    // The display goes crazy and displays weird characters.
    // Trial and error suggests 3us doesn't work but 4us delay here does work,
    // so using 8us to give some wiggle room.
    delay_us(8);

    LCD_E.clear(cs);
    delay_us(5);
}
