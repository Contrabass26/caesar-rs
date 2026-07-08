use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

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

const PIECE_FAMILIES: [u64; 54] = [
    0b1111 << 0,
    0b1111 << 0,
    0b1111 << 0,
    0b1111 << 0,

    0b1111 << 4,
    0b1111 << 4,
    0b1111 << 4,
    0b1111 << 4,

    0b11111111 << 8,
    0b11111111 << 8,
    0b11111111 << 8,
    0b11111111 << 8,
    0b11111111 << 8,
    0b11111111 << 8,
    0b11111111 << 8,
    0b11111111 << 8,

    0b1111 << 16,
    0b1111 << 16,
    0b1111 << 16,
    0b1111 << 16,

    0b1111 << 20,
    0b1111 << 20,
    0b1111 << 20,
    0b1111 << 20,

    0b11111111 << 24,
    0b11111111 << 24,
    0b11111111 << 24,
    0b11111111 << 24,
    0b11111111 << 24,
    0b11111111 << 24,
    0b11111111 << 24,
    0b11111111 << 24,

    0b11111111 << 32,
    0b11111111 << 32,
    0b11111111 << 32,
    0b11111111 << 32,
    0b11111111 << 32,
    0b11111111 << 32,
    0b11111111 << 32,
    0b11111111 << 32,

    0b11111111 << 40,
    0b11111111 << 40,
    0b11111111 << 40,
    0b11111111 << 40,
    0b11111111 << 40,
    0b11111111 << 40,
    0b11111111 << 40,
    0b11111111 << 40,

    0b1111 << 48,
    0b1111 << 48,
    0b1111 << 48,
    0b1111 << 48,

    0b11 << 52,
    0b11 << 52,
];

const NUM_PIECES: usize = 10;
const NUM_ALL_PIECES: usize = 54;
const MAX_REGIONS: usize = 10;
const FIRST_COL: u64 = paint!(0, 7, 14, 21, 28, 35, 42, 49);
const LAST_COL: u64 = paint!(20, 27, 34, 41, 48);
const FACTORIALS: [u128; MAX_REGIONS + 1] = [1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880, 3628800];

// Has 1s in the spaces we need to fill
const TO_FILL: u64 = (1 << 56) - 1 - (1 << 6) - (1 << 13) - (1 << 53) + (1 << 49) - (THURSDAY + day(2) + JULY);

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

#[derive(PartialEq, Eq, Hash)]
struct Exploration {
    region: u64, // we explored ways of covering `region`...
    is_placed: u64, // ...with pieces that hadn't been placed according to `is_placed`
}

// Returns the number of ways in which regions[region_index..num_regions) can be filled with the pieces available
fn fill(regions: [u64; MAX_REGIONS], num_regions: usize, region_index: usize, placements: &mut [u64; NUM_PIECES], is_placed: u64, num_placed: usize, cache: &mut HashMap<Exploration, u128>, flag: bool, file: &mut File) -> u128 {
    let spaces = regions[region_index];
    // Check whether this state has been explored already
    if let Some(n) = cache.get(&Exploration { region: spaces, is_placed }) {
        return *n;
    }
    // Keep track of the number of ways we've found
    let mut num_ways = 0;
    // Choose a piece to place
    for piece_index in 0..NUM_ALL_PIECES {
        if is_placed & (1 << piece_index) != 0 { continue }
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
                    if flag { println!("Placed first piece {}", piece) }
                    if spaces & !piece == 0 {
                        // This region has been filled
                        if region_index + 1 >= num_regions {
                            // All regions have been filled
                            num_ways += 1;
                            if num_placed == 9 {
                                let mut message = String::from("Found solution!");
                                for j in 0..num_placed + 1 {
                                    message = message + &" " + placements[j].to_string().as_str();
                                }
                                message += &"\n";
                                print!("{}", message);
                                file.write_all(message.as_bytes()).expect("Failed to write to file");
                            }
                        } else {
                            // Fill the next one
                            num_ways += fill(regions, num_regions, region_index + 1, placements, is_placed | PIECE_FAMILIES[piece_index], num_placed + 1, cache, false, file);
                        }
                    } else {
                        // Split the grid into regions
                        let mut new_regions = [0u64; MAX_REGIONS];
                        let mut region_sizes = [i32::MAX; MAX_REGIONS];
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
                        // Sort the regions by size
                        let mut indices: [usize; MAX_REGIONS] = core::array::from_fn(|i| i);
                        indices.sort_by_key(|i| region_sizes[*i]);
                        // Fill all these regions
                        let sorted_regions: [u64; MAX_REGIONS] = core::array::from_fn(|i| new_regions[indices[i]]);
                        num_ways += FACTORIALS[new_num_regions] * fill(sorted_regions, new_num_regions, 0, placements, is_placed | PIECE_FAMILIES[piece_index], num_placed + 1, cache, false, file);
                    }
                }
                piece <<= 1;
                if piece & FIRST_COL != 0 { break }
            }
            y_base <<= 7;
        }
    }
    // Write to the cache
    cache.insert(Exploration { region: spaces, is_placed }, num_ways);
    num_ways
}

fn main() {
    let mut regions: [u64; MAX_REGIONS] = [0; MAX_REGIONS];
    regions[0] = TO_FILL;
    let mut placements = [0u64; NUM_PIECES];
    let mut cache = HashMap::new();
    let mut file = File::create("log.txt").expect("Failed to create File");
    let num_ways = fill(regions, 1, 0, &mut placements, 0, 0, &mut cache, true, &mut file);
    println!("num_ways = {}", num_ways);
}