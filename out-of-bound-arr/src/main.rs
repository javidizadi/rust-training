// index out of bounds in run-time
fn main() {
    let arr: [u8; 4] = [22, 2, 2, 2];
    for i in 0..5 {
        println!("{:?}", arr[i]);
    }
}
