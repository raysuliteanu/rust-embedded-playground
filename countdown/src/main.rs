#![no_main]
#![no_std]

use cortex_m_rt::entry;
use microbit::board::Board;
use microbit::display::blocking::Display;
use microbit::hal::timer::Timer;
use microbit::pac::TIMER0;
use panic_halt as _;

const DELAY: u32 = 250;

struct LedMatrix {
    timer: Timer<TIMER0>,
    display: Display,
    matrix: [[[u8; 5]; 5]; 10],
}

impl LedMatrix {
    fn new(board: Board) -> Self {
        let timer = Timer::new(board.TIMER0);
        let display = Display::new(board.display_pins);
        LedMatrix {
            timer,
            display,
            matrix: [
                // 0
                [
                    [0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0],
                ],
                // 1
                [
                    [0, 0, 1, 0, 0],
                    [0, 1, 1, 0, 0],
                    [0, 0, 1, 0, 0],
                    [0, 0, 1, 0, 0],
                    [0, 1, 1, 1, 0],
                ],
                // 2
                [
                    [0, 0, 1, 1, 0],
                    [0, 1, 0, 1, 0],
                    [0, 0, 1, 0, 0],
                    [0, 1, 0, 0, 0],
                    [0, 1, 1, 1, 0],
                ],
                // 3
                [
                    [0, 1, 1, 1, 0],
                    [0, 0, 0, 1, 0],
                    [0, 1, 1, 1, 0],
                    [0, 0, 0, 1, 0],
                    [0, 1, 1, 1, 0],
                ],
                // 4
                [
                    [0, 1, 0, 1, 0],
                    [0, 1, 0, 1, 0],
                    [0, 1, 1, 1, 0],
                    [0, 0, 0, 1, 0],
                    [0, 0, 0, 1, 0],
                ],
                // 5
                [
                    [0, 1, 1, 1, 0],
                    [0, 1, 0, 0, 0],
                    [0, 1, 1, 1, 0],
                    [0, 0, 0, 1, 0],
                    [0, 1, 1, 1, 0],
                ],
                // 6
                [
                    [0, 1, 1, 1, 0],
                    [0, 1, 0, 0, 0],
                    [0, 1, 1, 1, 0],
                    [0, 1, 0, 1, 0],
                    [0, 1, 1, 1, 0],
                ],
                // 7
                [
                    [0, 1, 1, 1, 0],
                    [0, 0, 0, 1, 0],
                    [0, 0, 1, 0, 0],
                    [0, 1, 0, 0, 0],
                    [1, 0, 0, 0, 0],
                ],
                // 8
                [
                    [0, 1, 1, 1, 0],
                    [0, 1, 0, 1, 0],
                    [0, 1, 1, 1, 0],
                    [0, 1, 0, 1, 0],
                    [0, 1, 1, 1, 0],
                ],
                // 9
                [
                    [0, 1, 1, 1, 0],
                    [0, 1, 0, 1, 0],
                    [0, 1, 1, 1, 0],
                    [0, 0, 0, 1, 0],
                    [0, 0, 0, 1, 0],
                ],
            ],
        }
    }

    fn show(&mut self, num: usize) {
        self.display.show(&mut self.timer, self.matrix[num], DELAY);
    }
}

#[entry]
fn main() -> ! {
    let board = Board::take().unwrap();
    let mut led_matrix = LedMatrix::new(board);

    // infinite loop; just so we don't leave this stack frame
    loop {
        for n in (0..10).rev() {
            led_matrix.show(n);
        }
    }
}
