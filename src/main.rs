use std::env;
use std::path::Path;

mod commands;
mod memory;
mod utils;

mod test_commands;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Invalid args");
        return;
    }

    let source_path = args[1].as_str();

    if !Path::new(source_path).exists() {
        println!("No such file! ({})", source_path);
        return;
    }

    let fragments = utils::read_source(source_path);
    let mut state = memory::setup_memory();

    for f in &fragments {
        println!("{}", f);
        let new_state = commands::exec(state, &f);
        new_state.dump();

        state = new_state
    }
}
