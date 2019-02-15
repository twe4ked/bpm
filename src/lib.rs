use std::{
    io::{self, Read, Result, Write},
    time::SystemTime,
};

const SAMPLE_COUNT: usize = 10;

pub fn run() -> Result<()> {
    setup_terminal()?;

    let mut samples = Vec::with_capacity(SAMPLE_COUNT);

    loop {
        let now = SystemTime::now();

        if get_char()? == ' ' {
            if samples.iter().count() >= SAMPLE_COUNT {
                samples.remove(0);
            }
            samples.push(now.elapsed().unwrap().subsec_millis());

            let median = median(&samples);
            if median == 0 {
                println!("??? BPM");
            } else {
                println!("{} BPM", 60_000 / median);
            }
        }
    }
}

fn get_char() -> Result<char> {
    let stdout = io::stdout();
    let mut reader = io::stdin();
    let mut buffer = [0; 1];

    stdout.lock().flush()?;
    reader.read_exact(&mut buffer)?;
    Ok(buffer[0] as char)
}

fn median(vec: &[u32]) -> u32 {
    if vec.len() == 1 {
        vec[0]
    } else {
        let mut vec = vec.to_owned();
        vec.sort();
        if vec.len() % 2 == 0 {
            (vec[vec.len() / 2] + vec[(vec.len() / 2) - 1]) / 2
        } else {
            vec[vec.len() / 2]
        }
    }
}

fn setup_terminal() -> Result<()> {
    let mut termios = termios::Termios::from_fd(libc::STDIN_FILENO)?;
    termios.c_lflag &= !(termios::ICANON | termios::ECHO);
    termios::tcsetattr(0, termios::TCSANOW, &termios)?;
    Ok(())
}
