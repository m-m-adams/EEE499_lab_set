#![no_std]
#![no_main]
extern crate alloc;

#[allow(unused)]
mod controllers;
mod led_states;
mod pending;
mod wifi;

use alloc::format;
use alloc::string::String;
use core::cell::RefCell;
use cortex_m::peripheral::NVIC;
use critical_section::CriticalSection;
use defmt::*;
use embassy_executor::{InterruptExecutor, Spawner};
use embassy_rp::interrupt::{Interrupt, InterruptExt, Priority};
use embassy_rp::pac::pwm::regs::Intr;
use embassy_rp::peripherals::{PIO0, UART0};
use embassy_rp::pio::InterruptHandler;
use embassy_rp::{bind_interrupts, interrupt, pac};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::blocking_mutex::Mutex;
use embassy_time::{Duration, Timer};
use embassy_rp::uart;
use embassy_rp::uart::{Async, BufferedInterruptHandler, BufferedUart, BufferedUartTx, UartTx};
use embassy_sync::channel::Channel;
use embedded_alloc::TlsfHeap as Heap;
use embedded_alloc;
use static_cell::StaticCell;

#[allow(unused)]
use {defmt_rtt as _, panic_probe as _};



#[global_allocator]
static HEAP: Heap = Heap::empty();

// interrupts exist in what's called a vector table, which is a table of addresses that point to functions that are called when an interrupt occurs.
// This creates function called PIO0_IRQ_0 and sets it as an interrupt handler. That function will call InterruptHandler::on_interrupt.
// This is used in rust at compile time to prove to peripherals that interrupts they require are registered

bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
    UART0_IRQ => BufferedInterruptHandler<UART0>;
});

pub type UartChannel = Channel<CriticalSectionRawMutex, String, 10>;

#[embassy_executor::task]
pub async fn uart_sender(mut uart_tx: UartTx<'static, Async>) {
    let mut count = 0;
    let clear = "\x1Bc".as_bytes();
    info!("clear {:x}", clear);

    uart_tx.write(clear).await.unwrap();
    loop {
        let data = format!("testy test {:00}\r", count);
        count +=1;
        info!("TX {:x}", data.as_bytes());
        uart_tx.write(&data.as_bytes()).await.unwrap();
        Timer::after_secs(1).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 131072; //can get dynamically from linkerscript but this should be fine
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
    }
    let p = embassy_rp::init(Default::default());
    let mut conf = uart::Config::default();
    conf.baudrate = 115200;
    let mut uart_tx = UartTx::<Async>::new(p.UART0, p.PIN_0, p.DMA_CH0, conf);
    spawner.spawn(uart_sender(uart_tx)).unwrap();

}