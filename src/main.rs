#![no_main]
#![no_std]

use cortex_m_rt::entry;
use microbit::board::Board;
use microbit::display::blocking::Display;
use microbit::hal::timer::Timer;
use microbit::pac::TIMER0;
use panic_halt as _;

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

    // infinite loop; just so we don't leave this stack frame
    loop {
        for c in 0..5 {
            led_matrix.show(0, c);
        }

        led_matrix.show(1, 4);
        led_matrix.show(2, 4);
        led_matrix.show(3, 4);

        for c in (0..5).rev() {
            led_matrix.show(4, c);
        }

        led_matrix.show(3, 0);
        led_matrix.show(2, 0);
        led_matrix.show(1, 0);
    }
}
