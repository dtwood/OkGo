#![no_std]
#![feature(const_fn)]
#![feature(lang_items)]
#![feature(never_type)]
#![feature(proc_macro)]
#![feature(unwind_attributes)]

extern crate bare_metal;
extern crate cortex_m_rtfm as rtfm;
extern crate f0;
extern crate firmware_common;
extern crate hmac;
extern crate md_5;
extern crate nb;
extern crate stm32f0xx;

mod beeper;
mod radio;
mod io;
mod utils;

use rtfm::app;
use f0::output::Output;
use f0::gpio::{Gpio, Port};
use f0::adc::Adc;
use f0::dac::Dac;
use f0::spi::Spi;
use rtfm::Resource;
use firmware_common::rfm;

/// Drop delay in ms
const PACKET_DROP_DELAY: u32 = 5000;

app! {
    device: stm32f0xx,

    resources: {
        static BEEPER: beeper::Beeper = beeper::Beeper::new(Dac::new(Port::A, 4));
        static LAST_PACKET: u32 = 0;
        static MILLIS: u32 = 0;

        static LED_GREEN: Output = Output::new(Port::B, 13);
        static LED_YELLOW: Output = Output::new(Port::B, 12);
        static LED_ARM: Output = Output::new(Port::B, 8);
        static LED_DISARM: Output = Output::new(Port::B, 9);

        static UPSTREAM_RELAY: Output = Output::new(Port::A, 10);
        static FIRE_CH1: Output = Output::new(Port::A, 9);
        static FIRE_CH2: Output = Output::new(Port::A, 8);
        static FIRE_CH3: Output = Output::new(Port::B, 15);
        static FIRE_CH4: Output = Output::new(Port::B, 14);

        static BATT_MON: Adc = Adc::new(Port::A, 0, 0);
        static RELAY_SENSE: Adc = Adc::new(Port::B, 1, 9);
        static CONT_CH1: Adc = Adc::new(Port::B, 0, 8);
        static CONT_CH2: Adc = Adc::new(Port::A, 7, 7);
        static CONT_CH3: Adc = Adc::new(Port::A, 6, 6);
        static CONT_CH4: Adc = Adc::new(Port::A, 5, 5);

        static RADIO: firmware_common::rfm::Radio = firmware_common::rfm::Radio::new(
            Output::new(Port::A, 15),
            Spi {
                sck: Gpio {
                    port: Port::B,
                    pin: 3,
                },
                miso: Gpio {
                    port: Port::B,
                    pin: 4,
                },
                mosi: Gpio {
                    port: Port::B,
                    pin: 5,
                },
            }
        );
    },

    tasks: {
        SYS_TICK: {
            path: sys_tick,
            resources: [MILLIS],
            priority: 2,
        },

        TIM2_IRQ: {
            path: timer_tick,
            resources: [
                BEEPER, LAST_PACKET, MILLIS,
                LED_GREEN, LED_YELLOW, LED_ARM, LED_DISARM, // Status LEDs
                UPSTREAM_RELAY, FIRE_CH1, FIRE_CH2, FIRE_CH3, FIRE_CH4, // Firing Channels
                ADC, BATT_MON, RELAY_SENSE, CONT_CH1, CONT_CH2, CONT_CH3, CONT_CH4, // ADCs
                RADIO, // Radio
            ],
        },
    }
}

// The initialization phase.
//
// This runs first and within a *global* critical section. Nothing can preempt
// this function.
fn init(p: init::Peripherals, r: init::Resources) {
    // Setup crystal oscillator and systick
    // unsafe {
    //     libopencm3_sys::rcc_clock_setup_in_hsi_out_48mhz();
    //     libopencm3_sys::systick_init();
    // }

    // Clock GPIOs, set pin modes
    io::init(&p, &r);
    // Initialise radio and local state variables, read stored config
    r.RADIO
        .init(p.RCC, p.SPI1, rfm::Frf::Frf868, radio::RADIO_POWER_DBM);

    r.RADIO.receive_async(radio::REQ_PACKET_LEN);

    // /* 48MHz / 8 => 6,000,000 counts per second */
    // systick_set_clocksource(STK_CSR_CLKSOURCE_AHB);
    //
    // /* 6,000,000/6000 = 1000 overflows per second - every 1ms one interrupt */
    // /* SysTick interrupt every N clock pulses: set reload to N-1 */
    // systick_set_reload(47999);
    // // systick_interrupt_enable();
    // systick_counter_enable();
}

fn idle() -> ! {
    loop {
        rtfm::wfi();
    }
}

fn sys_tick(_t: &mut rtfm::Threshold, r: SYS_TICK::Resources) {
    **r.MILLIS += 1;
}

fn timer_tick(t: &mut rtfm::Threshold, r: TIM2_IRQ::Resources) {
    match radio::receive_async(t, r.RADIO) {
        Ok((rssi, packet)) => {
            **r.LAST_PACKET = **r.MILLIS.borrow(t);

            r.BEEPER.set_state(
                if packet.armed {
                    beeper::Rate::Fast
                } else {
                    beeper::Rate::Slow
                },
                match packet.beep_volume {
                    0 => beeper::Volume::Silent,
                    1 => beeper::Volume::Low,
                    2 => beeper::Volume::Loud,
                    _ => beeper::Volume::Medium,
                },
            );

            if packet.armed {
                r.LED_ARM.set();
                r.UPSTREAM_RELAY.set();
                r.FIRE_CH1.set_bool(packet.fire_ch1);
                r.FIRE_CH2.set_bool(packet.fire_ch2);
                r.FIRE_CH3.set_bool(packet.fire_ch3);
                r.FIRE_CH4.set_bool(packet.fire_ch4);
                r.LED_DISARM.clear();
            } else {
                r.LED_DISARM.set();
                r.UPSTREAM_RELAY.clear();
                r.FIRE_CH1.clear();
                r.FIRE_CH2.clear();
                r.FIRE_CH3.clear();
                r.FIRE_CH4.clear();
                r.LED_ARM.clear();
            }

            utils::delay_ms(t, &r.MILLIS, 10);
            radio::transmit(
                t,
                r.RADIO,
                &radio::CfmPacket {
                    received_rssi: rssi,
                    battery_voltage: utils::adc_to_battery_voltage(r.BATT_MON.read()),
                    armed: packet.armed,
                    fire_ch1: packet.fire_ch1,
                    fire_ch2: packet.fire_ch2,
                    fire_ch3: packet.fire_ch3,
                    fire_ch4: packet.fire_ch4,
                    cont_ch1: utils::adc_to_ohms(r.CONT_CH1.read()),
                    cont_ch2: utils::adc_to_ohms(r.CONT_CH2.read()),
                    cont_ch3: utils::adc_to_ohms(r.CONT_CH3.read()),
                    cont_ch4: utils::adc_to_ohms(r.CONT_CH4.read()),
                },
            );
            r.RADIO.receive_async(radio::REQ_PACKET_LEN);

            r.LED_GREEN.toggle();
            r.LED_YELLOW.toggle();
        }
        Err(nb::Error::WouldBlock) => {
            if **r.MILLIS.borrow(t) - **r.LAST_PACKET > PACKET_DROP_DELAY {
                r.LED_DISARM.set();
                r.UPSTREAM_RELAY.clear();
                r.FIRE_CH1.clear();
                r.FIRE_CH2.clear();
                r.FIRE_CH3.clear();
                r.FIRE_CH4.clear();
                r.LED_ARM.clear();
            };
        }
        Err(nb::Error::Other(_)) => {
            r.LED_DISARM.set();
            r.UPSTREAM_RELAY.clear();
            r.FIRE_CH1.clear();
            r.FIRE_CH2.clear();
            r.FIRE_CH3.clear();
            r.FIRE_CH4.clear();
            r.LED_ARM.clear();
        }
    }

    r.BEEPER.do_beep(t, &r.MILLIS);
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}
