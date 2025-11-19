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
Implement the alarm clock shown in the moodle lab guide. Use serial communication to/from the computer to handle keyboard input