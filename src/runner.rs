use std::process::Command;

pub fn run_command(command: String, parameter: String) {

    Command::new(command).arg(parameter).status().expect("Run Failed!");
}
