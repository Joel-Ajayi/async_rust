use std::arch::asm;

fn main() {
    let msg = "Hello, Async Programs: Raw Syscall!\n".to_string();
    syscall(msg);
}

#[cfg(target_os = "linux")]
#[inline(never)] // we never want this function to be inlined during optimization
/*  Inlining is when the compiler omits the function
call and simply copies the body of the function instead of calling it. */

fn syscall(msg: String) {
    let msg_ptr = msg.as_ptr() as usize;
    let msg_len = msg.len();

    unsafe {
        asm!(
            "mov rax, 1",      // system call 1 is write on Linux
            "mov rdi, 1",      // file handle 1 is stdout
            "syscall",         // call kernel, software interrupt
            in("rsi") msg_ptr, // address of string to output
            in("rdx") msg_len,     // number of bytes
            out("rax") _, out("rdi") _, lateout("rsi") _, lateout("rdx") _
        );
    }
}
