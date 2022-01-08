mod core;

fn main() {
    core::command_line::execute().expect("Error whilst trying to parse the file.");
}
