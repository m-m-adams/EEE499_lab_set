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