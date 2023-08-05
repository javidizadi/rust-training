use std::io;
fn main() {
    let mut user_input = String::new();
    println!("Enter n: ");
    io::stdin()
        .read_line(&mut user_input)
        .expect("failed to read input.");
    let n: u64 = user_input
        .trim()
        .parse()
        .expect("user input must be an unsigned integer.");
    println!("{}", fibonacci(n));
}
fn fibonacci(n: u64) -> u64 {
    let mut calc_base: [u64; 2] = [0, 1];
    if n <= 1 {
        calc_base[n as usize]
    } else {
        let mut is_odd = true;
        for _ in 0..n {
            if is_odd {
                calc_base[0] += calc_base[1]
            } else {
                calc_base[1] += calc_base[0];
            }
            is_odd = !is_odd;
        }
        calc_base[if is_odd { 0 } else { 1 }]
    }
}
