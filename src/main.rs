
mod runner;
mod handler;

fn main() {
    let mut arguments = std::env::args().skip(1);
    let value = arguments.next().expect("Value Not Provided!");

    let m = handler::match_input(&value);
    runner::run_command(m.0, m.1)
}


