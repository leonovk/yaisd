use std::process::Command;
use std::thread;

mod config;

pub fn run() {
    let commands = config::commands();
    let mut streams = Vec::new();

    for command in commands {
        let handle = thread::spawn(|| {
            execute(command);
        });

        streams.push(handle);
    }

    for compute in streams {
        compute.join().expect("error 3");
    }
}

fn execute(command: String) {
    let mut execuitble = Vec::new();

    for word in command.split_whitespace() {
        execuitble.push(word.to_string());
    }

    let mut exec = Command::new(&execuitble[0]);

    for arg in execuitble.into_iter().skip(1) {
        exec.arg(arg);
    }

    exec.output().expect("The command failed");
}
