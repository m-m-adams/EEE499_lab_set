# Lab 1 - State Machines with Buttons and LEDs

## Stage 1 - Hello World!:
1. Add serial printing hello world to the main program. To do this you will use the log macro from defmt
2. Blink the onboard LED On and Off. This LED is GPIO 0 controlled by the wifi chip
3. Every second toggle the LED and print ON or OFF to the terminal

## Stage 2 - State Machines:
### Requirements:
1. Connect the buttons and LEDs to your GPIO pins. I suggest using GPIOs 2,3,4,5 for the buttons and 6,7,8,9 for the LEDs
2. Implement the following state machine for a Button/LED pair
   1. Your possible states are :
       1. Fade in and out over 3 seconds
       2. Blink every two seconds
       3. Toggle On button press
   2. Transitions are:
       1. While the LED is toggled ON, long pressing the button switches to a fade
       2. While the LED is toggled ON, Double pressing the button quickly switches to Blinking
       3. While the LED is Blinking or Fading, pressing the button switches back to toggle mode in the ON state
       4. While the LED is Blinking or Fading, holding the button switches to toggle mode in the Off state

### Hints:
1. There are 8 PWM slices on the pico, each can control two GPIOs. Slice 0 can control 1/2, 1 has 3/4 etc. You can split
the PWM controller into seperate channels that share the same PWM frequency but have seperate duty cycles by calling
`pwm.split()` (or `split_by_reference()` if you want to avoid moving the pwm object)
2. The state machine should be ran from an external function, not a method on the state. This will greatly simplify lifetimes
3. Use dependency injection to pass in the button and LEDs to the run function. This will allow you to test without hardware
4. The states should be enum variants which all implement an `LED_state` trait. Use the `enum dispatch` crate to minimize boiler plate
5. Split your state transitions into two functions, one handling button events and one handling time transitions
6. You can put the button and the LED state into two seperate tasks communicating over an async channel. This will allow the button 
handling to run in parallel with the lED time updates and behave the same in all states. Otherwise it is possible to make it 
work in a single task but the fading state logic will need to be more complex

## Stage 3 - Multiple Buttons and LEDs:

1. Make four copies of your button/LED state machine running in parallel
2. Make sure your code is expandable to any number of buttons
3. Make sure buttons and LEDs are not hardcoded to specific GPIOs
4. There are two plausible ways to do this:
    1. Make an input handling task that reads all buttons and passes them to the appropriate state machine, and an output
         handling task that takes LED commands from all state machines and updates the appropriate LED
    2. Make each state machine handle its own button and LED
    
   One will enable testing without hardware, two will be simpler to implement but harder to debug

