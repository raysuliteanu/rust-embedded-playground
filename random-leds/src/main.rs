#![no_main]
#![no_std]

use cortex_m_rt::entry;
use microbit::board::Board;
use microbit::display::blocking::Display;
use microbit::hal::timer::Timer;
use microbit::pac::TIMER0;
use panic_halt as _;
use rand::prelude::*;

const DELAY: u32 = 100;

struct LedMatrix {
    timer: Timer<TIMER0>,
    display: Display,
    matrix: [[u8; 5]; 5],
}

impl LedMatrix {
    fn new(board: Board) -> Self {
        let timer = Timer::new(board.TIMER0);
        let display = Display::new(board.display_pins);
        LedMatrix {
            timer,
            display,
            matrix: [
                [0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0],
            ],
        }
    }

    fn show(&mut self, row: usize, col: usize) {
        self.matrix[row][col] = 1;
        self.display.show(&mut self.timer, self.matrix, DELAY);
        self.matrix[row][col] = 0;
    }
}

#[entry]
fn main() -> ! {
    let board = Board::take().unwrap();
    let mut led_matrix = LedMatrix::new(board);

    let mut rng = SmallRng::seed_from_u64(0xdeadbeef);

    // infinite loop; just so we don't leave this stack frame
    loop {
        let r: usize = rng.random_range(0..5);
        let c: usize = rng.random_range(0..5);

        led_matrix.show(r, c);
    }
}
