use core::arch::asm;
fn main() {
    let msg: &[u8] = b"Hello world :)\n";
    unsafe {
        asm!(
        "syscall",
            in("rax") 1,
            in("rdi") 1,
            in("rsi") msg.as_ptr(),
            in("rdx") msg.len(),
            options(nostack)
        );
    }
}
