use std::{
    io::{self, Read, Write},
    process,
    time::SystemTime,
};

fn main() {
    let mut termios = termios::Termios::from_fd(libc::STDIN_FILENO).unwrap_or_else(|err| {
        println!("An error occured: {}", err);
        process::exit(1);
    });
    termios.c_lflag &= !(termios::ICANON | termios::ECHO);
    termios::tcsetattr(0, termios::TCSANOW, &termios).unwrap_or_else(|err| {
        println!("An error occured: {}", err);
        process::exit(1);
    });

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
