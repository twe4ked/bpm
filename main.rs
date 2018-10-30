extern crate libc;

use std::mem;
use std::io;
use std::io::Read;
use std::io::Write;
use std::time::SystemTime;

fn main() {
    let fd = 0;
    let mut termios = unsafe { mem::uninitialized() };
    unsafe { libc::tcgetattr(fd, &mut termios) };
    termios.c_lflag &= !(libc::ICANON | libc::ECHO);
    unsafe { libc::tcsetattr(fd, libc::TCSAFLUSH, &termios) };

    let mut vec = Vec::with_capacity(10);

    loop {
        let now = SystemTime::now();
        let stdout = io::stdout();
        let mut reader = io::stdin();
        let mut buffer = [0;1];

        stdout.lock().flush().unwrap();
        reader.read_exact(&mut buffer).unwrap();

        if buffer[0] == 32 {
            if vec.iter().count() >= 10 {
                vec.remove(0);
            }

            vec.push(now.elapsed().unwrap().subsec_millis());

            let mut sorted_vec = vec.clone();
            sorted_vec.sort();
            let median = sorted_vec[sorted_vec.len() / 2];

            println!("{} BPM", 60000 / median);
        }
    }
}
