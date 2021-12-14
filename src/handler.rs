
pub fn match_input(value: &str) -> (String, String) {
    let file = read_db("/usr/local/bin/qic_dir/qic_db");
    for line in file.lines() {
        let mut chunk = line.splitn(3, ':');
        let key = chunk.next().expect("Key Missing!");
        let command = chunk.next().expect("Command Missing!");
        let parameter = chunk.next().expect("Missing Parameters!");
        if key == value {
            return (command.to_string(), parameter.to_string())
        }
    }
    ("No Value".to_string(), "Error".to_string())
}

pub fn read_db(path: &str) -> String {
    let contents = std::fs::read_to_string(path)
        .expect("Database couldn't be found.");
        return contents
}
