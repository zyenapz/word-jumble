#[macro_export]
macro_rules! clear_console {
    () => {
        print!("\x1B[2J\x1B[1;1H");
    };
}

#[macro_export]
macro_rules! pause_console {
    ($msg: expr) => {
        let mut stdout = std::io::stdout();
        stdout.write($msg).unwrap();
        stdout.flush().unwrap();
        io::stdin().read(&mut [0]).unwrap();
    };
}

#[macro_export]
macro_rules! display_prompt {
    () => {
        print!("> ");
        std::io::stdout()
            .flush()
            .expect("Error encountered when flushing stdout!");
    };
}

pub fn display_notice(msg: &str) {
    if !msg.is_empty() {
        println!(": {}", msg);
    }
}
