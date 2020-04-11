use crate::memory;

pub fn add(state: memory::State, rs: &str, rt: &str, rd: &str) -> memory::State {
    println!("Add {} + {} -> {}", rs, rt, rd);
    return state.clone()
}

pub fn load_immediate(state: memory::State, rd: &str, imm: &str) -> memory::State {
    println!("Load {} -> {}", imm, rd);
    let new_state = memory::write_to_register(state, rd, imm.parse().unwrap());
    return new_state
}

pub fn catchall(state: memory::State) -> memory::State {
    println!("NOT_IMPLEMENTED");
    return state.clone();
}
