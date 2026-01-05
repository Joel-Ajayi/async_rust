use std::arch::asm;

fn main() {
    let _t = 100;
    let t_ptr = 99999999999999 as *const usize;
    let x = dereference(t_ptr);
    println!("{}", x);
}

fn dereference(ptr: *const usize) -> usize {
    let res: usize;
    unsafe { asm!("mov {0}, [{1}]", out(reg) res, in(reg) ptr) }
    res
}
