// run with --release flag to occur int overflow.
fn main() {
    let mut a: u8 = 0;
    loop {
        a += 1;
        println!("{:?}", a);
        if a == 0 {
            break;
        };
    }
}
