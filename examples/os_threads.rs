use std::thread::{self, sleep};

fn main() {
    println!("So, we start the program here!");

    let thread1 = thread::spawn(move || {
        sleep(std::time::Duration::from_millis(200));
        println!("First thread: finish last!");
    });

    let thread2 = thread::spawn(move || {
        sleep(std::time::Duration::from_millis(100));
        println!("Second thread");
        let thread3 = thread::spawn(move || {
            sleep(std::time::Duration::from_millis(50));
            println!("Third thread!");
        });

        thread3.join().unwrap();
    });

    thread1.join().unwrap();
    thread2.join().unwrap();
}
