mod core;

use std::env::args;

fn main() {
    let arguments = args().skip(1).collect::<Vec<String>>();
    core::command_line::execute(arguments);
}
