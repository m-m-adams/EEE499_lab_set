# Starter template for EE499

# Objectives
We'll cover some final quality of life improvements for embedded Rust development.
First we'll add an allocator. This allows the use of heap allocated data structures like Vec and Box.
Next we'll add a serial connection. This lets us send messages seperately from the debug output.
Then we'll use a gatekeeper task on the serial connection to batch outputs together

# Part 1
We're going to use one of the allocators in embedded_alloc. https://docs.rs/embedded-alloc/0.6.0/embedded_alloc/index.html
Set it as the global allocator

## Part 2 - Gatekeeper
Use the uart task as a gatekeeper for serial output. Create a channel that other tasks can send strings to.
The uart task reads from that channel and writes the strings to the serial port.
Spawn 4 led controller tasks. Each task should blink an led at a different rate and send a message to the uart task each time it toggles the led.

## Part 3 - Clock (optional, alternatively implement your research project)

We will be building a digital clock. The serial line will be used for display
The below code will overwrite the same line repeatedly. This is good for a clock display as it will not flood the terminal with messages.
```angular2html
loop {
        let data = format!("testy test {:00}\r", count);
        count +=1;
        uart_tx.write(&data.as_bytes()).await.unwrap(); 
}
```

The digital watch should provide the following functionalities using the 4 buttons available on the board:
1. Showing Time: When the watch start, it sets the time to 00:00:00. then it tracks the time progress and
   shows it on watch display. 
2. Stopwatch: By pressing ‘0’ the watch should switch to stopwatch mode and return back to normal mode
   by pressing ‘0’ again. When watch switch to stop watch mode, the stopwatch value should be shown in
   the display. Pressing ‘1’ should start and repressing it should stop the stopwatch, and pressing ‘2’ should
   reset the stopwatch.
3. Alarm: By pressing ‘3’, the user can set an alarm by entering an hour and minute. When in this mode holding 0 should
scroll through hours, and holding 1 should scroll through seconds. 
Obviously the watch should start beeping when the alarms time is reached. Use the onboard buzzer

Your solution should include 5 tasks. 
1. Core task which provides the core services and controls the watch. 
2. TimeTracker task which tracks the time progress (at all times) and updates a global watch
3. Stopwatch task which provides the stopwatch functionalities.
4. Display task that works a gatekeeper for showing message in display. 
None of the tasks can write directly to the display and all requests should be sent to Display task. 
5. An input task which reads button inputs and publishes the events to other tasks.
6. An alarm task which buzzes when the alarm time is reached.

Write a readme that provides an overview of how the watch is structured