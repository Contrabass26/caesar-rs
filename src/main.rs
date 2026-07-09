use std::{array, thread};
use std::fs::File;
use std::io::Write;
use std::sync::mpsc;

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
const MONTHS: [u64; 12] = [JANUARY, FEBRUARY, MARCH, APRIL, MAY, JUNE, JULY, AUGUST, SEPTEMBER, OCTOBER, NOVEMBER, DECEMBER];

const fn day(i: usize) -> u64 { 1 << (i + 14) }

const MONDAY: u64 = 1 << 46;
const TUESDAY: u64 = 1 << 47;
const WEDNESDAY: u64 = 1 << 48;
const THURSDAY: u64 = 1 << 53;
const FRIDAY: u64 = 1 << 54;
const SATURDAY: u64 = 1 << 55;
const SUNDAY: u64 = 1 << 45;
const WEEKDAYS: [u64; 7] = [MONDAY, TUESDAY, WEDNESDAY, THURSDAY, FRIDAY, SATURDAY, SUNDAY];

const PIECES: [u64; 54] = [
    // T
    paint!(0, 1, 2, 8, 15),
    paint!(2, 7, 8, 9, 16),
    paint!(1, 8, 14, 15, 16),
    paint!(0, 7, 8, 9, 14),

    // Z
    paint!(0, 1, 8, 15, 16),
    paint!(2, 7, 8, 9, 14),
    paint!(1, 2, 8, 14, 15),
    paint!(0, 7, 8, 9, 16),

    // Vault
    paint!(0, 7, 8, 14, 15),
    paint!(0, 1, 2, 7, 8),
    paint!(0, 1, 7, 8, 15),
    paint!(1, 2, 7, 8, 9),
    paint!(1, 7, 8, 14, 15),
    paint!(0, 1, 7, 8, 9),
    paint!(0, 1, 7, 8, 14),
    paint!(0, 1, 2, 8, 9),

    // Tetris
    paint!(0, 7, 8, 15),
    paint!(1, 2, 7, 8),
    paint!(1, 7, 8, 14),
    paint!(0, 1, 8, 9),

    // Engineer
    paint!(0, 7, 14, 15, 16),
    paint!(0, 1, 2, 7, 14),
    paint!(0, 1, 2, 9, 16),
    paint!(2, 9, 14, 15, 16),

    // Long tetris
    paint!(0, 7, 8, 15, 22),
    paint!(2, 3, 7, 8, 9),
    paint!(0, 7, 14, 15, 22),
    paint!(1, 2, 3, 7, 8),
    paint!(1, 7, 8, 14, 21),
    paint!(0, 1, 2, 9, 10),
    paint!(1, 8, 14, 15, 21),
    paint!(0, 1, 8, 9, 10),

    // L
    paint!(0, 7, 14, 15),
    paint!(0, 1, 2, 7),
    paint!(0, 1, 8, 15),
    paint!(2, 7, 8, 9),
    paint!(1, 8, 14, 15),
    paint!(0, 7, 8, 9),
    paint!(0, 1, 7, 14),
    paint!(0, 1, 2, 9),

    // Long L
    paint!(0, 7, 14, 21, 22),
    paint!(0, 1, 2, 3, 7),
    paint!(0, 1, 8, 15, 22),
    paint!(3, 7, 8, 9, 10),
    paint!(1, 8, 15, 21, 22),
    paint!(0, 7, 8, 9, 10),
    paint!(0, 1, 7, 14, 21),
    paint!(0, 1, 2, 3, 10),

    // Enzyme
    paint!(0, 2, 7, 8, 9),
    paint!(0, 1, 7, 14, 15),
    paint!(0, 1, 2, 7, 9),
    paint!(0, 1, 8, 14, 15),

    // Finger
    paint!(0, 7, 14, 21),
    paint!(0, 1, 2, 3),
];

const NUM_PIECES: usize = 10;
const MAX_REGIONS: usize = 10;
const FIRST_COL: u64 = paint!(0, 7, 14, 21, 28, 35, 42, 49);
const LAST_COL: u64 = paint!(20, 27, 34, 41, 48);
const FAMILY_STARTS: [usize; NUM_PIECES] = [0, 4, 8, 16, 20, 24, 32, 40, 48, 52];
const FAMILY_SIZES: [usize; NUM_PIECES] = [4, 4, 8, 4, 4, 8, 8, 8, 4, 2];
const IS_FIVE: [bool; NUM_PIECES] = [true, true, true, false, true, true, false, true, true, false];

// Has 1s in the spaces we need to fill
const BASE_TO_FILL: u64 = (1 << 56) - 1 - (1 << 6) - (1 << 13) - (1 << 53) + (1 << 49);

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

fn print_solution(mut placements: [u64; NUM_PIECES]) {
    for y in 0..=7 {
        for i in 0..NUM_PIECES {
            for x in 0..=6 {
                if x == 6 && (y == 0 || y == 1) || y == 7 && x <= 3 {
                    print!(" ");
                } else if placements[i] & 1 == 1 {
                    print!("\u{25A0}");
                } else {
                    print!("\u{25A1}");
                }
                placements[i] >>= 1;
            }
            print!("    ");
        }
        println!();
    }
}

// Returns the number of ways in which regions[region_index..num_regions) can be filled with the pieces available
fn fill(regions: &[u64; MAX_REGIONS], num_regions: usize, region_index: usize, placements: &mut [u64; NUM_PIECES], num_placed: usize) -> usize {
    let spaces = regions[region_index];
    // Keep track of the number of ways we've found
    let mut num_ways = 0;
    // Choose a piece configuration to place
    let family_size = FAMILY_SIZES[num_placed];
    let family_start = FAMILY_STARTS[num_placed];
    for piece_index in family_start..(family_start + family_size) {
        let base_piece = PIECES[piece_index];
        // Choose a location for it
        let mut y_base = base_piece;
        // We don't actually need to try the bottom row: no piece can start there
        for _y in 0..=6 {
            let mut piece = y_base;
            loop {
                if piece | spaces == spaces {
                    // There is room for the piece here
                    placements[num_placed] = piece;
                    let new_num_placed = num_placed + 1;
                    // if flag { println!("Placed first piece {}", piece) }
                    if spaces & !piece == 0 {
                        // This region has been filled
                        if region_index + 1 >= num_regions {
                            // All regions have been filled
                            num_ways += 1;
                            if new_num_placed == 10 {
                                // println!("Found solution!");
                                // print_solution(*placements);
                            }
                        } else {
                            // Fill the next one
                            num_ways += fill(regions, num_regions, region_index + 1, placements, new_num_placed);
                        }
                    } else {
                        // Split the grid into regions
                        let mut new_regions = [0u64; MAX_REGIONS];
                        let mut region_sizes = [usize::MAX; MAX_REGIONS];
                        let mut new_num_regions = 0;
                        {
                            let mut covered = !spaces | piece; // the cells that can't be added to a new region
                            let mut start = 1;
                            let mut i = 0; // start = 2^i
                            'find_region: loop {
                                // Find a cell which isn't in a region yet
                                while covered & start != 0 {
                                    i += 1;
                                    start <<= 1;
                                    if i > 55 { break 'find_region; } // we've checked all the cells and they don't need a region
                                }
                                // Bucket-fill a region starting from here
                                // TODO: Better recursion-based (DFS) bucket fill?
                                let mut region = start;
                                covered |= start;
                                let mut size = 1;
                                {
                                    let mut changed = false;
                                    let mut candidate = 1;
                                    let mut forward = true;
                                    loop {
                                        if candidate & covered == 0 && ((candidate >> 7) & region != 0 || (candidate << 7) & region != 0 || candidate & FIRST_COL == 0 && (candidate >> 1) & region != 0 || candidate & LAST_COL == 0 && (candidate << 1) & region != 0) {
                                            region |= candidate;
                                            covered |= candidate;
                                            changed = true;
                                            size += 1;
                                        }
                                        if candidate == 1 && !forward || candidate == 1 << 55 && forward {
                                            forward = !forward;
                                            if !changed { break }
                                            changed = false;
                                        }
                                        candidate = if forward { candidate << 1 } else { candidate >> 1 }
                                    }
                                }
                                // Add this region to the array
                                if new_num_regions >= MAX_REGIONS {
                                    panic!("Region array isn't big enough")
                                }
                                new_regions[new_num_regions] = region;
                                region_sizes[new_num_regions] = size;
                                new_num_regions += 1;
                            }
                        }
                        // Find how many 4s and 5s there are
                        let mut four = 0;
                        for i in new_num_placed..NUM_PIECES {
                            if !IS_FIVE[i] { four += 1 }
                        }
                        let five = NUM_PIECES - new_num_placed - four;
                        // Check that all the region sizes can be made with this combination of piece sizes
                        let mut is_possible = true;
                        'size_check: for i in 0..new_num_regions {
                            let size = region_sizes[i];
                            for n in 0..=four {
                                let m = NUM_PIECES - new_num_placed - n;
                                if m > five { continue }
                                let total = n * 4 + m * 5;
                                if total == size { continue 'size_check }
                                if total > size {
                                    is_possible = false;
                                    break 'size_check;
                                }
                            }
                            is_possible = false;
                            break 'size_check;
                        }
                        if is_possible {
                            // Try all possible orders for filling the regions
                            fn permute(permutation: &mut [u64; MAX_REGIONS], is_used: u16, num_used: usize, regions: &[u64; MAX_REGIONS], num_regions: usize, placements: &mut [u64; NUM_PIECES], num_placed: usize, num_ways: &mut usize) {
                                // Find a region that isn't in the permutation yet
                                for i in 0..num_regions {
                                    if is_used & (1 << i) != 0 { continue }
                                    permutation[num_used] = regions[i];
                                    if num_used + 1 == num_regions {
                                        // Submit this permutation
                                        *num_ways += fill(permutation, num_regions, 0, placements, num_placed);
                                    } else {
                                        // Continue adding to the permutation
                                        permute(permutation, is_used | (1 << i), num_used + 1, regions, num_regions, placements, num_placed, num_ways);
                                    }
                                }
                            }
                            let mut sorted_regions = [0; MAX_REGIONS];
                            permute(&mut sorted_regions, 0, 0, &new_regions, new_num_regions, placements, new_num_placed, &mut num_ways);
                        }
                    }
                }
                piece <<= 1;
                if piece & FIRST_COL != 0 { break }
            }
            y_base <<= 7;
        }
    }
    num_ways
}

fn count_ways(weekday: u64, day: u64, month: u64) -> usize {
    let mut regions = [0; MAX_REGIONS];
    regions[0] = BASE_TO_FILL & !(weekday | day | month);
    let mut placements = [0; NUM_PIECES];
    fill(&regions, 1, 0, &mut placements, 0)
}

const NUM_THREADS: usize = 15;
const NUM_PUZZLES: usize = 7 * 31 * 12;
const PUZZLES_PER_THREAD: usize = NUM_PUZZLES / NUM_THREADS;

fn count_all() {
    #[derive(Copy, Clone)]
    struct PuzzleResult {
        i: usize,
        j: usize,
        k: usize,
        num_ways: usize,
    }

    let (tx, rx) = mpsc::channel();

    let worker_handles: [_; NUM_THREADS] = array::from_fn(|n| {
        let tx = tx.clone();
        thread::spawn(move || {
            let mut regions: [u64; MAX_REGIONS] = [0; MAX_REGIONS];
            let mut placements = [0u64; NUM_PIECES];
            let start = n * PUZZLES_PER_THREAD;
            let end = if n == (NUM_THREADS - 1) { NUM_PUZZLES } else { start + PUZZLES_PER_THREAD };
            let mut i = start / (31 * 12);
            let mut j = (start % (31 * 12)) / 12;
            let mut k = start % 12;
            for _input in start..end {
                regions[0] = BASE_TO_FILL & !(WEEKDAYS[i] | day(j) | MONTHS[k]);
                let num_ways = fill(&regions, 1, 0, &mut placements, 0);
                tx.send(PuzzleResult { i, j, k, num_ways }).expect("Failed to send result");
                k += 1;
                if k >= 12 {
                    k = 0;
                    j += 1;
                    if j >= 31 {
                        j = 0;
                        i += 1;
                    }
                }
            }
        })
    });

    let collector_handle = thread::spawn(move || {
        let mut file = File::create("../log.csv").expect("Failed to create log file");
        for _input in 0..NUM_PUZZLES {
            let result = rx.recv().expect("Failed to receive result");
            let message = format!("{},{},{},{}\n", result.i, result.j, result.k, result.num_ways);
            print!("{}", message);
            file.write(message.as_bytes()).expect("Failed to write to file");
        }
    });

    for handle in worker_handles {
        handle.join().expect("Failed to join worker thread");
    }
    collector_handle.join().expect("Failed to join collector thread");
}

fn main() {
    count_all();
}