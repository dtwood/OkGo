use lcd;
use io;
use stm32f0xx;
use bare_metal::CriticalSection;
use firmware_common::utils::delay_ms;


/// Display an appropriate splash screen depending on state
pub extern "C" fn splash(cs: &CriticalSection) {
    // If reset caused by windowed/independant watchdog:
    let csr = &stm32f0xx::RCC.borrow(cs).csr;
    if csr.read().wwdgrstf().bit_is_set() | csr.read().iwdgrstf().bit_is_set() {
        // Reset wdt signal
        csr.write(|w| w.rmvf().set_bit());

        splash_wdt(cs);
    } else if !io::SW_KEY.get_bool(cs) {
        splash_armed(cs);
    } else {
        splash_normal(cs);
    }
}

/// Display the normal CUSF OkGo2 splash screen
pub extern "C" fn splash_normal(cs: &CriticalSection) {
    lcd::cursor_pos(cs, 0, 0);
    lcd::puts(cs, &"Cambridge    OKGO 2 ");
    lcd::cursor_pos(cs, 1, 0);
    lcd::puts(cs, &"University  by dwt27");
    lcd::cursor_pos(cs, 2, 0);
    lcd::puts(cs, &"Space       Sept '15");
    lcd::cursor_pos(cs, 3, 0);
    lcd::puts(cs, &"Flight         devel");
    // puts(FIRMWARE_VERSION); // TODO
    delay_ms(2000);

    lcd::cursor_pos(cs, 0, 0);
    lcd::puts(cs, &"Buzzer settings     ");
    lcd::cursor_pos(cs, 1, 0);
    lcd::puts(cs, &"Hold on boot:       ");
    lcd::cursor_pos(cs, 2, 0);
    lcd::puts(cs, &"CH1: off  CH2: low  ");
    lcd::cursor_pos(cs, 3, 0);
    lcd::puts(cs, &"CH3: med  CH4: high ");
    delay_ms(2000);
}

/// Display the WDT reset splash
pub extern "C" fn splash_wdt(cs: &CriticalSection) {
    lcd::cursor_pos(cs, 0, 0);
    lcd::puts(cs, &"      WARNING!      ");
    lcd::cursor_pos(cs, 1, 0);
    lcd::puts(cs, &"   INTERNAL ERROR   ");
    lcd::cursor_pos(cs, 2, 0);
    lcd::puts(cs, &"WATCHDOG TIMER RESET");
    delay_ms(1000);
}

/// Hang in the turned-on-while-armed splash
pub extern "C" fn splash_armed(cs: &CriticalSection) {
    lcd::cursor_pos(cs, 0, 0);
    lcd::puts(cs, &"REFUSING TO POWER  ");
    lcd::cursor_pos(cs, 1, 0);
    lcd::puts(cs, &"UP WHILE ARMED! TURN");
    lcd::cursor_pos(cs, 2, 0);
    lcd::puts(cs, &"OFF, DISARM, THEN   ");
    lcd::cursor_pos(cs, 3, 0);
    lcd::puts(cs, &"TURN BACK ON AGAIN! ");
    loop {
        // iwdg_reset(); // TODO
    }
}
