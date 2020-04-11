mod utils;
mod commands;
mod memory;
mod constants;

fn main() {
    let source_path = "./test.s";
    let fragments = utils::read_source(source_path);
    let mut state = memory::setup_memory();
    for f in &fragments {
        println!("{}", f);
        let new_state = commands::exec(state, &f);
        new_state.dump();
        
        state = new_state
    }
}
