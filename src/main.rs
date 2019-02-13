use bpm;
use std::process;

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

    bpm::run();
}
