use std::env;

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

fn main() {
    let args: Vec<String> = env::args().collect();
    let grid: u64 = args[1].parse().unwrap();
    print_grid(grid);
}
