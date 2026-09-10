# Embedded Playground

Various projects to help learn embedded `no-std` Rust programming.

Currently examples using microbit v2

Based on the book here: [micro::bit v2 Embedded Discovery Book](https://docs.rust-embedded.org/discovery-mb2/index.html)

## Running Examples

For the non-embassy-rs projects, currently run them with

`cargo embed --target thumbv7em-none-eabihf`

## Projects

### embassy-ex

Try and port e.g. countdown example to use embassy-rs

### countdown

LEDs count down from 9 to 0 repeatedly

### random-leds

As name implies, light up random LED continuously
