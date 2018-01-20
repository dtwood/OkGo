#![no_std]
#![feature(const_fn)]
#![feature(lang_items)]
#![feature(never_type)]
#![feature(proc_macro)]
#![feature(unwind_attributes)]

extern crate bare_metal;
extern crate cortex_m_rtfm as rtfm;
extern crate embedded_hal;
extern crate firmware_common;
extern crate hmac;
extern crate md5;
extern crate nb;
extern crate rfm95w;
extern crate stm32f0xx;
extern crate stm32f0xx_hal as f0;

// mod beeper;
mod board;
mod radio;
// mod io;
mod utils;

use radio::RadioExt;
use rtfm::Threshold;
use rtfm::app;
use f0::gpio::GpioExt;
use firmware_common::rfm;
use f0::prelude::*;

/// Drop delay in ms
const PACKET_DROP_DELAY: u32 = 5000;

app! {
    device: stm32f0xx,

    resources: {
        // static BEEPER: beeper::Beeper = beeper::Beeper::new();
        static LAST_PACKET: u32 = 0;
        static MILLIS: u32 = 0;

        static LED_GREEN: board::LedGreen;
        static LED_YELLOW: board::LedYellow;
        static LED_ARM: board::LedArm;
        static LED_DISARM: board::LedDisarm;

        static RELAY_UPSTREAM: board::RelayUpstream;
        static RELAY_CHANNEL_1: board::RelayChannel1;
        static RELAY_CHANNEL_2: board::RelayChannel2;
        static RELAY_CHANNEL_3: board::RelayChannel3;
        static RELAY_CHANNEL_4: board::RelayChannel4;

        // static BATT_MON: Adc = Adc::new(Port::A, 0, 0);
        // static RELAY_SENSE: Adc = Adc::new(Port::B, 1, 9);
        // static CONT_CH1: Adc = Adc::new(Port::B, 0, 8);
        // static CONT_CH2: Adc = Adc::new(Port::A, 7, 7);
        // static CONT_CH3: Adc = Adc::new(Port::A, 6, 6);
        // static CONT_CH4: Adc = Adc::new(Port::A, 5, 5);

        static RADIO: board::Radio;
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
                LAST_PACKET, MILLIS,
                LED_GREEN, LED_YELLOW, LED_ARM, LED_DISARM,
                RELAY_UPSTREAM, RELAY_CHANNEL_1, RELAY_CHANNEL_2, RELAY_CHANNEL_3, RELAY_CHANNEL_4,
                // BATT_MON, RELAY_SENSE, CONT_CH1, CONT_CH2, CONT_CH3, CONT_CH4, // ADCs
                RADIO, // Radio
            ],
        },
    }
}

// The initialization phase.
//
// This runs first and within a *global* critical section. Nothing can preempt
// this function.
fn init(p: init::Peripherals, _r: init::Resources) -> init::LateResources {
    // Setup crystal oscillator and systick
    // unsafe {
    //     libset_highcm3_sys::rcc_clock_setup_in_hsi_out_48mhz();
    //     libset_highcm3_sys::systick_init();
    // }

    let mut rcc = p.device.RCC.constrain();
    let mut flash = p.device.FLASH.constrain();
    let mut ahb = rcc.ahb;
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut gpioa = p.device.GPIOA.split(&mut ahb);
    let mut gpiob = p.device.GPIOB.split(&mut ahb);
    let spi = f0::spi::Spi::spi1(
        p.device.SPI1,
        (
            gpioa.pa5.into_af5(&mut gpioa.moder, &mut gpioa.afrl),
            gpioa.pa6.into_af5(&mut gpioa.moder, &mut gpioa.afrl),
            gpiob.pb5.into_af5(&mut gpiob.moder, &mut gpiob.afrl),
        ),
        embedded_hal::spi::Mode {
            polarity: embedded_hal::spi::Polarity::IdleLow,
            phase: embedded_hal::spi::Phase::CaptureOnFirstTransition,
        },
        f0::time::MegaHertz(1),
        clocks,
        &mut rcc.apb2,
    );
    let nss = gpioa
        .pa15
        .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);
    let mut radio =
        rfm95w::Rfm95w::new(spi, nss, rfm::Frf::Frf868, radio::RADIO_POWER_DBM).unwrap();

    radio.receive_async(radio::REQ_PACKET_LEN).unwrap();

    // r.BEEPER.init(p.DAC, Port::A, 4);

    // Upstream relay and firing channels, default all set_low

    // /* 48MHz / 8 => 6,000,000 counts per second */
    // systick_set_clocksource(STK_CSR_CLKSOURCE_AHB);
    //
    // /* 6,000,000/6000 = 1000 overflows per second - every 1ms one interrupt */
    // /* SysTick interrupt every N clock pulses: set reload to N-1 */
    // systick_set_reload(47999);
    // // systick_interrupt_enable();
    // systick_counter_enable();

    init::LateResources {
        RADIO: radio,
        LED_GREEN: gpiob
            .pb13
            .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper),
        LED_YELLOW: gpiob
            .pb12
            .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper),
        LED_ARM: gpiob
            .pb8
            .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper),
        LED_DISARM: gpiob
            .pb9
            .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper),
        RELAY_UPSTREAM: gpioa
            .pa10
            .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper),
        RELAY_CHANNEL_1: gpioa
            .pa9
            .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper),
        RELAY_CHANNEL_2: gpioa
            .pa8
            .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper),
        RELAY_CHANNEL_3: gpiob
            .pb15
            .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper),
        RELAY_CHANNEL_4: gpiob
            .pb14
            .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper),
    }
}

fn idle() -> ! {
    loop {
        rtfm::wfi();
    }
}

fn sys_tick(_t: &mut rtfm::Threshold, mut r: SYS_TICK::Resources) {
    *r.MILLIS += 1;
}

fn fire(r: &mut TIM2_IRQ::Resources, fire_1: bool, fire_2: bool, fire_3: bool, fire_4: bool) {
    r.LED_DISARM.set_low();
    r.LED_ARM.set_high();
    if fire_1 {
        r.RELAY_CHANNEL_1.set_low()
    };
    if fire_2 {
        r.RELAY_CHANNEL_2.set_low()
    };
    if fire_3 {
        r.RELAY_CHANNEL_3.set_low()
    };
    if fire_4 {
        r.RELAY_CHANNEL_4.set_low()
    };
    r.RELAY_UPSTREAM.set_low();
}

fn disarm(r: &mut TIM2_IRQ::Resources) {
    r.RELAY_CHANNEL_1.set_high();
    r.RELAY_CHANNEL_2.set_high();
    r.RELAY_CHANNEL_3.set_high();
    r.RELAY_CHANNEL_4.set_high();
    r.RELAY_UPSTREAM.set_high();
    r.LED_ARM.set_low();
    r.LED_DISARM.set_high();
}

fn timer_tick(t: &mut rtfm::Threshold, mut r: TIM2_IRQ::Resources) {
    match r.RADIO.receive_packet() {
        Ok((rssi, packet)) => {
            *r.LAST_PACKET = utils::get_millis(t, &r.MILLIS);

            // r.BEEPER.set_rate(if packet.armed {
            //     beeper::Rate::Fast
            // } else {
            //     beeper::Rate::Slow
            // });
            // r.BEEPER.set_volume(match packet.beep_volume {
            //     0 => beeper::Volume::Silent,
            //     1 => beeper::Volume::Low,
            //     2 => beeper::Volume::Loud,
            //     _ => beeper::Volume::Medium,
            // });

            if packet.armed {
                fire(
                    &mut r,
                    packet.fire_ch1,
                    packet.fire_ch2,
                    packet.fire_ch3,
                    packet.fire_ch4,
                )
            } else {
                disarm(&mut r);
            }

            utils::delay_ms(t, &r.MILLIS, 10);
            r.RADIO.transmit_packet(&radio::CfmPacket {
                received_rssi: rssi,
                battery_voltage: unimplemented!(), // utils::adc_to_battery_voltage(r.BATT_MON.read(t, &r.ADC)),
                armed: packet.armed,
                fire_ch1: packet.fire_ch1,
                fire_ch2: packet.fire_ch2,
                fire_ch3: packet.fire_ch3,
                fire_ch4: packet.fire_ch4,
                cont_ch1: unimplemented!(), // utils::adc_to_ohms(r.CONT_CH1.read(t, &r.ADC)),
                cont_ch2: unimplemented!(), // utils::adc_to_ohms(r.CONT_CH2.read(t, &r.ADC)),
                cont_ch3: unimplemented!(), // utils::adc_to_ohms(r.CONT_CH3.read(t, &r.ADC)),
                cont_ch4: unimplemented!(), // utils::adc_to_ohms(r.CONT_CH4.read(t, &r.ADC)),
            });
            r.RADIO.receive_async(radio::REQ_PACKET_LEN).unwrap();

            // r.GPIOB.claim(t, |gpiob, _t| {
            // hal::leds::GREEN.toggle(gpiob);
            // hal::leds::YELLOW.toggle(gpiob);
            // });
        }
        Err(nb::Error::WouldBlock) => {
            if utils::get_millis(t, &r.MILLIS) - *r.LAST_PACKET > PACKET_DROP_DELAY {
                disarm(&mut r);
            };
        }
        Err(nb::Error::Other(_)) => {
            disarm(&mut r);
        }
    }

    // r.BEEPER.do_beep(t, &r.MILLIS, &r.DAC);
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[no_mangle]
pub extern "C" fn rust_begin_unwind(
    _fmt: &core::fmt::Arguments,
    _file_line: &(&'static str, usize),
) -> ! {
    loop {}
}
