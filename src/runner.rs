use std::process::Command;

pub fn run_command(command: String, parameter: String) {

//    println!("{} {}", command, parameter); //For debugging
    Command::new(command).arg(parameter).status().expect("Run Failed!");
}
