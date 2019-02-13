use std::{
    io::{self, Read, Write},
    mem,
    time::SystemTime,
};

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
        let mut buffer = [0; 1];

        stdout.lock().flush().expect("Failed to flush stream");
        reader
            .read_exact(&mut buffer)
            .expect("Failed to read character");

        if buffer[0] == 32 {
            if vec.iter().count() >= 10 {
                vec.remove(0);
            }

            vec.push(now.elapsed().unwrap().subsec_millis());

            let median = if vec.len() == 1 {
                vec[0]
            } else {
                let mut sorted_vec = vec.clone();
                sorted_vec.sort();
                if sorted_vec.len() % 2 == 0 {
                    (sorted_vec[sorted_vec.len() / 2] + sorted_vec[(sorted_vec.len() / 2) - 1]) / 2
                } else {
                    sorted_vec[sorted_vec.len() / 2]
                }
            };

            println!("{} BPM", 60_000 / median);
        }
    }
}
