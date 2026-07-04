use std::process::exit;

macro_rules! paint {
    ($s:expr) => {
        (1 << $s)
    };

    ($s:expr, $($ss:expr),+) => {
        paint!($s) | paint!($($ss),+)
    };
}

const JANUARY: u64 = 1 << 0;
const FEBRUARY: u64 = 1 << 1;
const MARCH: u64 = 1 << 2;
const APRIL: u64 = 1 << 3;
const MAY: u64 = 1 << 4;
const JUNE: u64 = 1 << 5;
const JULY: u64 = 1 << 7;
const AUGUST: u64 = 1 << 8;
const SEPTEMBER: u64 = 1 << 9;
const OCTOBER: u64 = 1 << 10;
const NOVEMBER: u64 = 1 << 11;
const DECEMBER: u64 = 1 << 12;

const fn day(i: usize) -> u64 { 1 << (i + 13) }

const MONDAY: u64 = 1 << 46;
const TUESDAY: u64 = 1 << 47;
const WEDNESDAY: u64 = 1 << 48;
const THURSDAY: u64 = 1 << 53;
const FRIDAY: u64 = 1 << 54;
const SATURDAY: u64 = 1 << 55;
const SUNDAY: u64 = 1 << 45;

// Has 1s in the spaces we're allowed to fill
const BOUNDS: u64 = (1 << 56) - 1 - (1 << 6) - (1 << 13) - (1 << 53) + (1 << 49) - (THURSDAY + day(2) + JULY);

// T
const P_0_0: u64 = paint!(0, 1, 2, 8, 15);
const P_0_1: u64 = paint!(2, 7, 8, 9, 16);
const P_0_2: u64 = paint!(1, 8, 14, 15, 16);
const P_0_3: u64 = paint!(0, 7, 8, 9, 14);

// Z
const P_1_0: u64 = paint!(0, 1, 8, 15, 16);
const P_1_1: u64 = paint!(2, 7, 8, 9, 14);
const P_1_2: u64 = paint!(1, 2, 8, 14, 15);
const P_1_3: u64 = paint!(0, 7, 8, 9, 16);

// Vault
const P_2_0: u64 = paint!(0, 7, 8, 14, 15);
const P_2_1: u64 = paint!(0, 1, 2, 7, 8);
const P_2_2: u64 = paint!(0, 1, 7, 8, 15);
const P_2_3: u64 = paint!(1, 2, 7, 8, 9);
const P_2_4: u64 = paint!(1, 7, 8, 14, 15);
const P_2_5: u64 = paint!(0, 1, 7, 8, 9);
const P_2_6: u64 = paint!(0, 1, 7, 8, 14);
const P_2_7: u64 = paint!(0, 1, 2, 8, 9);

// Tetris
const P_3_0: u64 = paint!(0, 7, 8, 15);
const P_3_1: u64 = paint!(1, 2, 7, 8);
const P_3_2: u64 = paint!(1, 7, 8, 14);
const P_3_3: u64 = paint!(0, 1, 8, 9);

// Engineer
const P_4_0: u64 = paint!(0, 7, 14, 15, 16);
const P_4_1: u64 = paint!(0, 1, 2, 7, 14);
const P_4_2: u64 = paint!(0, 1, 2, 9, 16);
const P_4_3: u64 = paint!(2, 9, 14, 15, 16);

// Long tetris
const P_5_0: u64 = paint!(0, 7, 8, 15, 22);
const P_5_1: u64 = paint!(2, 3, 7, 8, 9);
const P_5_2: u64 = paint!(0, 7, 14, 15, 22);
const P_5_3: u64 = paint!(1, 2, 3, 7, 8);
const P_5_4: u64 = paint!(1, 7, 8, 14, 21);
const P_5_5: u64 = paint!(0, 1, 2, 9, 10);
const P_5_6: u64 = paint!(1, 8, 14, 15, 21);
const P_5_7: u64 = paint!(0, 1, 8, 9, 10);

// L
const P_6_0: u64 = paint!(0, 7, 14, 15);
const P_6_1: u64 = paint!(0, 1, 2, 7);
const P_6_2: u64 = paint!(0, 1, 8, 15);
const P_6_3: u64 = paint!(2, 7, 8, 9);
const P_6_4: u64 = paint!(1, 8, 14, 15);
const P_6_5: u64 = paint!(0, 7, 8, 9);
const P_6_6: u64 = paint!(0, 1, 7, 14);
const P_6_7: u64 = paint!(0, 1, 2, 9);

// Long L
const P_7_0: u64 = paint!(0, 7, 14, 21, 22);
const P_7_1: u64 = paint!(0, 1, 2, 3, 7);
const P_7_2: u64 = paint!(0, 1, 8, 15, 22);
const P_7_3: u64 = paint!(3, 7, 8, 9, 10);
const P_7_4: u64 = paint!(1, 8, 15, 21, 22);
const P_7_5: u64 = paint!(0, 7, 8, 9, 10);
const P_7_6: u64 = paint!(0, 1, 7, 14, 21);
const P_7_7: u64 = paint!(0, 1, 2, 3, 10);

// Enzyme
const P_8_0: u64 = paint!(0, 2, 7, 8, 9);
const P_8_1: u64 = paint!(0, 1, 7, 14, 15);
const P_8_2: u64 = paint!(0, 1, 2, 7, 9);
const P_8_3: u64 = paint!(0, 1, 8, 14, 15);

// Finger
const P_9_0: u64 = paint!(0, 7, 14, 21);
const P_9_1: u64 = paint!(0, 1, 2, 3);

const NUM_PIECES: usize = 10;
const FIRST_COL: u64 = paint!(0, 7, 14, 21, 28, 35, 42, 49);

fn print_grid(mut grid: u64) {
    for y in 0..=7 {
        for x in 0..=6 {
            if x == 6 && (y == 0 || y == 1) || y == 7 && x <= 3 {
                print!(" ");
            } else if grid & 1 == 1 {
                print!("\u{25A0}");
            } else {
                print!("\u{25A1}");
            }
            grid >>= 1;
        }
        println!();
    }
}

fn add_piece(i: usize, grid: u64, placements: &mut [u64; NUM_PIECES]) {
    macro_rules! attempt {
        ($base_piece:expr) => {
            let mut y_base = $base_piece;
            // We don't actually need to iterate over the bottom row: no piece can start there
            for _y in 0..=6 {
                let mut piece = y_base;
                loop {
                    if piece | BOUNDS == BOUNDS && piece & grid == 0 {
                        // There is room for the piece here
                        placements[i] = piece;
                        if i == 9 {
                            println!("Found solution!");
                            for j in 0..NUM_PIECES {
                                println!("Placement {} = {}", j, placements[j]);
                                print_grid(placements[j]);
                            }
                            exit(0);
                        } else {
                            // Place the next piece
                            add_piece(i + 1, grid | piece, placements);
                        }
                    }
                    piece <<= 1;
                    if piece & FIRST_COL != 0 { break }
                }
                y_base <<= 7;
            }
        };
    }

    match i {
        0 => {
            attempt!(P_0_0);
            attempt!(P_0_1);
            attempt!(P_0_2);
            attempt!(P_0_3);
        },
        1 => {
            attempt!(P_1_0);
            attempt!(P_1_1);
            attempt!(P_1_2);
            attempt!(P_1_3);
        },
        2 => {
            attempt!(P_2_0);
            attempt!(P_2_1);
            attempt!(P_2_2);
            attempt!(P_2_3);
            attempt!(P_2_4);
            attempt!(P_2_5);
            attempt!(P_2_6);
            attempt!(P_2_7);
        },
        3 => {
            attempt!(P_3_0);
            attempt!(P_3_1);
            attempt!(P_3_2);
            attempt!(P_3_3);
        },
        4 => {
            attempt!(P_4_0);
            attempt!(P_4_1);
            attempt!(P_4_2);
            attempt!(P_4_3);
        },
        5 => {
            attempt!(P_5_0);
            attempt!(P_5_1);
            attempt!(P_5_2);
            attempt!(P_5_3);
            attempt!(P_5_4);
            attempt!(P_5_5);
            attempt!(P_5_6);
            attempt!(P_5_7);
        },
        6 => {
            attempt!(P_6_0);
            attempt!(P_6_1);
            attempt!(P_6_2);
            attempt!(P_6_3);
            attempt!(P_6_4);
            attempt!(P_6_5);
            attempt!(P_6_6);
            attempt!(P_6_7);
        },
        7 => {
            attempt!(P_7_0);
            attempt!(P_7_1);
            attempt!(P_7_2);
            attempt!(P_7_3);
            attempt!(P_7_4);
            attempt!(P_7_5);
            attempt!(P_7_6);
            attempt!(P_7_7);
        },
        8 => {
            attempt!(P_8_0);
            attempt!(P_8_1);
            attempt!(P_8_2);
            attempt!(P_8_3);
        },
        9 => {
            attempt!(P_9_0);
            attempt!(P_9_1);
        },
        _ => panic!("Expected i in 0..=9, got {}", i)
    };
}

fn main() {
    let mut placements = [0u64; NUM_PIECES];
    add_piece(0, 0, &mut placements);
}
