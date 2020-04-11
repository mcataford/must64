mod utils;
mod commands;
mod memory;
mod constants;

fn main() {
    let source_path = "./test.s";
    let fragments = utils::read_source(source_path);
    let mut state = memory::setup_memory();
    for f in &fragments {
        let command_fragments = utils::split_fragment(&f);
        let new_state = exec(state, command_fragments);
        new_state.dump();
        
        state = new_state
    }
    println!("{:?}", fragments);
}

fn exec(state: memory::State, command: Vec<&str>) -> memory::State{
    match command[0] {
        "add" => commands::add(state, command[1], command[2], command[3]),
        "li" => commands::load_immediate(state, command[1], command [2]),
        _ => commands::catchall(state),
    }
}


