use bpm;
use std::{io, process};

fn main() {
    setup_terminal().unwrap_or_else(|err| {
        println!("An error occured: {}", err);
        process::exit(1);
    });

    bpm::run();
}

fn setup_terminal() -> io::Result<()> {
    let mut termios = termios::Termios::from_fd(libc::STDIN_FILENO)?;
    termios.c_lflag &= !(termios::ICANON | termios::ECHO);
    termios::tcsetattr(0, termios::TCSANOW, &termios)?;
    Ok(())
}
