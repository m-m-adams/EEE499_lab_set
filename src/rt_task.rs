use alloc::vec::Vec;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::{Channel, Receiver, Sender};
use crate::led_states::PressType;

struct ThreadControlBlock {
    priority: u8,
    period: u32,
    max_time: u32,
    remaining_ticks: u32,
    run_function: fn(),
}

impl ThreadControlBlock {
    fn new(func: fn(), priority:u8 , period: u32, max_time: u32) -> ThreadControlBlock {
        ThreadControlBlock {
            priority,
            period,
            max_time,
            remaining_ticks: period,
            run_function: func,
        }
    }

    /// returns true if the function is ready to be ran
    fn tick(&mut self, ticks: u32) -> bool {
        if let Some(ticks) = self.remaining_ticks.checked_sub(ticks) {
            self.remaining_ticks = ticks;
             false
        }
        else {
            self.remaining_ticks = 0;
             true
        }
    }
    fn run(&mut self) {
        (self.run_function)();
        self.remaining_ticks = self.period;
    }
}

pub type TaskChannelReceiver<'a, const N:usize> = Receiver<'a, CriticalSectionRawMutex, ThreadControlBlock, N>;
pub type TaskChannelSender<'a, const N:usize> = Sender<'a, CriticalSectionRawMutex, ThreadControlBlock, N>;
pub type TaskChannel<const N:usize> = Channel<CriticalSectionRawMutex, ThreadControlBlock, N>;

pub struct Scheduler<const max_tasks:usize, const priority_levels:usize> {
    ready: [ThreadControlBlock; max_tasks],
    blocked: [ThreadControlBlock; max_tasks],
    executors: [TaskChannel<max_tasks>; priority_levels],
    ticks_to_next_change: usize
}

impl<const max_tasks:usize, const priority_levels:usize> Scheduler<max_tasks, priority_levels> {
    pub fn tick(&mut self, num: usize) {
        self.ticks_to_next_change = if let Some(ticks) = self.ticks_to_next_change.checked_sub(num) {
            ticks
        } else {
            // this should be 0 in which case we need to schedule it to run so probably use a for loop for that
            self.blocked.iter().enumerate().min_by_key(|(_, t)| t.remaining_ticks).unwrap().0
        };
    }
}
