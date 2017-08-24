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
mod hal;

use rtfm::app;
use rtfm::Resource;
use f0::output::Output;
use f0::gpio::{Gpio, Port};
use f0::adc::Adc;
use f0::spi::Spi;
use firmware_common::rfm;
use hal::{Led, Relay};

/// Drop delay in ms
const PACKET_DROP_DELAY: u32 = 5000;

app! {
    device: stm32f0xx,

    resources: {
        static BEEPER: beeper::Beeper = beeper::Beeper::new();
        static LAST_PACKET: u32 = 0;
        static MILLIS: u32 = 0;

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
            resources: [MILLIS, ADC, DAC],
            priority: 2,
        },

        TIM2_IRQ: {
            path: timer_tick,
            resources: [
                ADC, DAC, GPIOA, GPIOB, // Hardware
                BEEPER, LAST_PACKET, MILLIS,
                BATT_MON, RELAY_SENSE, CONT_CH1, CONT_CH2, CONT_CH3, CONT_CH4, // ADCs
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

    r.BEEPER.init(p.DAC, Port::A, 4);

    // Upstream relay and firing channels, default all off
    hal::relays::UPSTREAM.open(p.GPIOA);
    hal::relays::CHANNEL_1.open(p.GPIOA);
    hal::relays::CHANNEL_2.open(p.GPIOA);
    hal::relays::CHANNEL_3.open(p.GPIOB);
    hal::relays::CHANNEL_4.open(p.GPIOB);

    hal::relays::UPSTREAM.init(p.GPIOA, p.RCC);
    hal::relays::CHANNEL_1.init(p.GPIOA, p.RCC);
    hal::relays::CHANNEL_2.init(p.GPIOA, p.RCC);
    hal::relays::CHANNEL_3.init(p.GPIOB, p.RCC);
    hal::relays::CHANNEL_4.init(p.GPIOB, p.RCC);


    hal::leds::ARM.on(p.GPIOB);
    hal::leds::DISARM.off(p.GPIOB);
    hal::leds::ARM.init(p.GPIOB, p.RCC);
    hal::leds::DISARM.init(p.GPIOB, p.RCC);


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

fn fire(gpioa: &stm32f0xx::GPIOA, gpiob: &stm32f0xx::GPIOB, fire_1: bool, fire_2: bool, fire_3: bool, fire_4: bool)
{
    hal::leds::DISARM.off(gpiob);
    hal::leds::ARM.on(gpiob);
    hal::relays::CHANNEL_1.set_bool(gpioa, fire_1);
    hal::relays::CHANNEL_2.set_bool(gpioa, fire_2);
    hal::relays::CHANNEL_3.set_bool(gpiob, fire_3);
    hal::relays::CHANNEL_4.set_bool(gpiob, fire_4);
    hal::relays::UPSTREAM.close(gpioa);
}

fn disarm(gpioa: &stm32f0xx::GPIOA, gpiob: &stm32f0xx::GPIOB)
{
    hal::relays::UPSTREAM.open(gpioa);
    hal::relays::CHANNEL_1.open(gpioa);
    hal::relays::CHANNEL_2.open(gpioa);
    hal::relays::CHANNEL_3.open(gpiob);
    hal::relays::CHANNEL_4.open(gpiob);
    hal::leds::ARM.off(gpiob);
    hal::leds::DISARM.on(gpiob);
}

fn timer_tick(t: &mut rtfm::Threshold, r: TIM2_IRQ::Resources) {
    match radio::receive_async(t, r.RADIO) {
        Ok((rssi, packet)) => {
            **r.LAST_PACKET = utils::get_millis(t, &r.MILLIS);

            r.BEEPER.set_rate(
                if packet.armed {
                    beeper::Rate::Fast
                } else {
                    beeper::Rate::Slow
                }
            );
            r.BEEPER.set_volume(
                match packet.beep_volume {
                    0 => beeper::Volume::Silent,
                    1 => beeper::Volume::Low,
                    2 => beeper::Volume::Loud,
                    _ => beeper::Volume::Medium,
                },
            );

            if packet.armed {
                fire(r.GPIOA, r.GPIOB, packet.fire_ch1, packet.fire_ch2, packet.fire_ch3, packet.fire_ch4)
            } else {
                disarm(r.GPIOA, r.GPIOB);
            }

            utils::delay_ms(t, &r.MILLIS, 10);
            radio::transmit(
                t,
                r.RADIO,
                &radio::CfmPacket {
                    received_rssi: rssi,
                    battery_voltage: utils::adc_to_battery_voltage(r.BATT_MON.read(t, &r.ADC)),
                    armed: packet.armed,
                    fire_ch1: packet.fire_ch1,
                    fire_ch2: packet.fire_ch2,
                    fire_ch3: packet.fire_ch3,
                    fire_ch4: packet.fire_ch4,
                    cont_ch1: utils::adc_to_ohms(r.CONT_CH1.read(t, &r.ADC)),
                    cont_ch2: utils::adc_to_ohms(r.CONT_CH2.read(t, &r.ADC)),
                    cont_ch3: utils::adc_to_ohms(r.CONT_CH3.read(t, &r.ADC)),
                    cont_ch4: utils::adc_to_ohms(r.CONT_CH4.read(t, &r.ADC)),
                },
            );
            r.RADIO.receive_async(radio::REQ_PACKET_LEN);

            r.GPIOB.claim(t, |gpiob, _t| {
                hal::leds::GREEN.toggle(gpiob);
                hal::leds::YELLOW.toggle(gpiob);
            });
        }
        Err(nb::Error::WouldBlock) => {
            if utils::get_millis(t, &r.MILLIS) - **r.LAST_PACKET > PACKET_DROP_DELAY {
                disarm(r.GPIOA, r.GPIOB);
            };
        }
        Err(nb::Error::Other(_)) => {
            disarm(r.GPIOA, r.GPIOB);
        }
    }

    r.BEEPER.do_beep(t, &r.MILLIS, &r.DAC);
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}
