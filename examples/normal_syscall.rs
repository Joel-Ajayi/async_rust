use std::io;

fn main() {
    let msg = "Hello, Async Programs: Normal Syscall!\n".to_string();
    syscall(msg).unwrap();
}

// Foreign Function Interface (FFI)
#[cfg(target_os = "linux")]
#[link(name = "c")]
unsafe extern "C" {
    fn write(fd: u32, buf: *const u8, count: usize) -> i32;
}

#[cfg(target_family = "unix")]
fn syscall(message: String) -> io::Result<()> {
    let msg_ptr = message.as_ptr();
    let len = message.len();
    let res = unsafe { write(1, msg_ptr, len) };

    if res == -1 {
        return Err(io::Error::last_os_error());
    }

    Ok(())
}
