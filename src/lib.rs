use std::{
    io::{self, Read, Write},
    time::SystemTime,
};

pub fn run() {
    let mut vec = Vec::with_capacity(10);

    loop {
        let now = SystemTime::now();

        if get_char() == ' ' {
            if vec.iter().count() >= 10 {
                vec.remove(0);
            }

            vec.push(now.elapsed().unwrap().subsec_millis());

            let mut median = if vec.len() == 1 {
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

            if median == 0 {
                median = 1;
            }

            println!("{} BPM", 60_000 / median);
        }
    }
}

fn get_char() -> char {
    let stdout = io::stdout();
    let mut reader = io::stdin();
    let mut buffer = [0; 1];

    stdout.lock().flush().expect("Failed to flush stream");
    reader
        .read_exact(&mut buffer)
        .expect("Failed to read character");
    buffer[0] as char
}
