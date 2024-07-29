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
    let mut exec = Command::new(command);
    exec.output().expect("The command failed");
}
