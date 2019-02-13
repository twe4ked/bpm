use bpm;
use std::{io::Result, process};

fn main() {
    setup_terminal().unwrap_or_else(|err| {
        println!("An error occured: {}", err);
        process::exit(1);
    });

    bpm::run().unwrap_or_else(|err| {
        println!("An error occured: {}", err);
        process::exit(1);
    });
}

fn setup_terminal() -> Result<()> {
    let mut termios = termios::Termios::from_fd(libc::STDIN_FILENO)?;
    termios.c_lflag &= !(termios::ICANON | termios::ECHO);
    termios::tcsetattr(0, termios::TCSANOW, &termios)?;
    Ok(())
}
