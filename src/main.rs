
mod runner;
mod handler;

fn main() {
    let mut arguments = std::env::args().skip(1);
    let value = arguments.next().unwrap_or("".to_string());

    let (m, p) = handler::match_input(&value);
    runner::run_command(m, p)
}


