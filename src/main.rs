#![no_std]
#![no_main]

mod controllers;
mod led_states;
mod pending;
mod wifi;

use crate::controllers::run_led_state_machine;
use cyw43::Control;
use cyw43_pio::{PioSpi, DEFAULT_CLOCK_DIVIDER};
use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::gpio::{Input, Level, Output};
use embassy_rp::peripherals::{DMA_CH0, PIN_23, PIN_24, PIN_25, PIN_29, PIO0};
use embassy_rp::pio::{InterruptHandler, Pio};
use embassy_rp::{bind_interrupts, pwm, Peri};
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
});

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    //unwrap!(spawner.spawn(blinky(ctrl)));
    let input1 = Input::new(p.PIN_6, embassy_rp::gpio::Pull::Up);
    let input2 = Input::new(p.PIN_7, embassy_rp::gpio::Pull::Up);
    let input3 = Input::new(p.PIN_8, embassy_rp::gpio::Pull::Up);
    let input4 = Input::new(p.PIN_9, embassy_rp::gpio::Pull::Up);
    let conf = pwm::Config::default();
    let (pwm_a, pwm_b) =
        pwm::Pwm::new_output_ab(p.PWM_SLICE1, p.PIN_2, p.PIN_3, conf.clone()).split();
    let (pwm_c, pwm_d) = pwm::Pwm::new_output_ab(p.PWM_SLICE2, p.PIN_4, p.PIN_5, conf).split();
    spawner
        .spawn(run_led_state_machine(input1, pwm_a.unwrap()))
        .unwrap();
    spawner
        .spawn(run_led_state_machine(input2, pwm_b.unwrap()))
        .unwrap();
    spawner
        .spawn(run_led_state_machine(input3, pwm_c.unwrap()))
        .unwrap();
    spawner
        .spawn(run_led_state_machine(input4, pwm_d.unwrap()))
        .unwrap();
}
