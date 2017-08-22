use radio;
use lcd;
use firmware_common;
use control;
use io;
use libopencm3_sys;
use bare_metal::CriticalSection;

#[repr(C)]
#[derive(Debug)]
pub struct State {
    pub armed: bool,
    pub centre_frf: u32,
    pub ch1_status: ChannelStatus,
    pub ch2_status: ChannelStatus,
    pub ch3_status: ChannelStatus,
    pub ch4_status: ChannelStatus,
    pub beep_volume: u8,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ChannelStatus {
    Ok = 0,
    ContinuityError = 1,
    Fire = 2,
}

pub fn init(cs: &CriticalSection) {
    // Setup crystal oscillator and systick
    unsafe {
        libopencm3_sys::rcc_clock_setup_in_hsi_out_48mhz();
        firmware_common::utils::systick_init();
    }

    // Clock GPIOs, set pin modes
    io::init(cs);

    // Initialise radio and radio state
    radio::init(cs);
}

fn put_digit(cs: &CriticalSection, d: u8) {
    lcd::putc(cs, '0' as u8 + d);
}

/// Update display from local state
pub fn display_update(
    cs: &CriticalSection,
    state: &control::State,
    radio_state: &radio::State,
    lost_link: bool,
) {
    // Display format (link):
    //
    // ######################
    // #3.7V  DISARM SIG:99%#
    // #12.2V DISARM SIG:80%#
    // #002R      255R 000R #
    // # OK  FIRE ERR   OK  #
    // ######################

    // Display format (no link):
    //
    // ######################
    // #3.7V  DISARM SIG:0%V#
    // #      NO LINK!      #
    // #002R      255R 000R #
    // # OK  FIRE ERR   OK  #
    // ######################

    let adc_val = firmware_common::adc::adc_to_millivolts(io::BATT_MON.read(&cs));
    // Batt voltage comes via a 3k3 over 10k potential divider such that
    // V = Vbatt * 10/13.3
    // Vbatt = V * 133/100
    let mut control_batt_voltage = adc_val * 133 / 100;
    // We will round to tenths of a volt, AKA 100 millivolts
    // This bodge means the later implicit truncation acts like a proper round
    // Ignition is sent over pre-rounded so we don't need to do this for it
    if (control_batt_voltage / 10) % 10 >= 5 {
        control_batt_voltage += 10;
    }

    // Control battery voltage
    lcd::cursor_pos(cs, 0, 0);
    put_digit(cs, (control_batt_voltage / 1000) as u8);
    lcd::putc(cs, '.' as u8);
    put_digit(cs, ((control_batt_voltage / 100) % 10) as u8);
    lcd::puts(cs, &"V  ");

    // Control state
    lcd::cursor_pos(cs, 0, 6);
    if state.armed {
        lcd::puts(cs, &"ARMED  ");
    } else {
        lcd::puts(cs, &"DISARM ");
    }

    // Control signal level
    lcd::cursor_pos(cs, 0, 13);
    lcd::puts(cs, &"SIG:");
    put_digit(cs, (radio_state.packet_rssi / 10) as u8);
    put_digit(cs, (radio_state.packet_rssi % 10) as u8);
    lcd::putc(cs, '%' as u8);

    // Ignition's row:
    lcd::cursor_pos(cs, 1, 0);
    if !lost_link {
        // Battery voltage
        put_digit(cs, (radio_state.rx_voltage / 100) as u8);
        put_digit(cs, ((radio_state.rx_voltage / 10)) % 10 as u8);
        lcd::putc(cs, '.' as u8);
        put_digit(cs, (radio_state.rx_voltage % 10) as u8);
        lcd::puts(cs, &"V ");

        // State:
        if radio_state.rx_status & 0b11100000 != 0 {
            lcd::puts(cs, &"ERROR  ");
        } else {
            if radio_state.rx_status & (1 << 4) != 0 {
                lcd::puts(cs, &"ARMED  ");
            } else {
                lcd::puts(cs, &"DISARM ");
            }
        }

        // Signal level
        lcd::puts(cs, &"SIG:");
        put_digit(cs, (radio_state.rx_rssi / 10) as u8);
        put_digit(cs, (radio_state.rx_rssi % 10) as u8);
        lcd::putc(cs, '%' as u8);
    } else {
        lcd::puts(cs, &"      NO LINK!      ");
    }


    // Channel continuities
    lcd::cursor_pos(cs, 2, 0);
    if !lost_link {
        control_display_ch_cont(cs, radio_state.rx_cont1, state.ch1_status);
        control_display_ch_cont(cs, radio_state.rx_cont2, state.ch2_status);
        control_display_ch_cont(cs, radio_state.rx_cont3, state.ch3_status);
        control_display_ch_cont(cs, radio_state.rx_cont4, state.ch4_status);
    } else {
        lcd::puts(cs, &"                    ");
    }

    // Control channel status
    lcd::cursor_pos(cs, 3, 0);
    control_display_ch_status(cs, state.ch1_status, radio_state.rx_status & 0x01 != 0);
    control_display_ch_status(cs, state.ch2_status, radio_state.rx_status & 0x02 != 0);
    control_display_ch_status(cs, state.ch3_status, radio_state.rx_status & 0x04 != 0);
    control_display_ch_status(cs, state.ch4_status, radio_state.rx_status & 0x08 != 0);

}

/// Convert a channel status enum into a string and print to LCD
fn control_display_ch_status(cs: &CriticalSection, ch_status: ChannelStatus, remote_fire: bool) {
    if remote_fire {
        lcd::puts(cs, &"FIRE ");
        return;
    }

    match ch_status
    {
        ChannelStatus::Ok =>
            lcd::puts(cs, &" OK  "),
        ChannelStatus::ContinuityError =>
            lcd::puts(cs, &" ERR "),
        ChannelStatus::Fire =>
            // We are requesting fire but remote refuses/ignores
            lcd::puts(cs, &" NO  "),
        // _ =>
        //     lcd::puts(cs, &"     "),
    }
}

/// Display a continuity resistance to the LCD
fn control_display_ch_cont(cs: &CriticalSection, cont: u8, ch_status: ChannelStatus) {
    // Don't display invalid continuities for firing channels
    if ch_status == ChannelStatus::Fire {
        lcd::puts(cs, &"     ");
        return;
    }

    if cont == 255 {
        // 255 is a magic value meaning open
        lcd::puts(cs, &"  ");
        lcd::putc(cs, 0b11110011);
    } else if cont == 254 {
        // 254 is a magic value meaning >255 but not open
        lcd::puts(cs, &" hi");
    } else {
        put_digit(cs, cont / 100);
        put_digit(cs, (cont / 10) % 10);
        put_digit(cs, cont % 10);
    }
    lcd::putc(cs, 0b11110100); // Character code for ohms
    lcd::putc(cs, ' ' as u8);
}
