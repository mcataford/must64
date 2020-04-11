use crate::memory;
use crate::utils;
use regex;

pub fn exec(state: memory::State, command: &str) -> memory::State {
    let command_fragments = utils::split_fragment(command);
    return exec_fragment(state, command_fragments);
}

fn exec_fragment(state: memory::State, fragment: regex::Captures) -> memory::State {
    match &fragment["op"] {
        "add" => add(state, &fragment["rs"], &fragment["rt"], &fragment["rd"]),
        "li" => load_immediate(state, &fragment["rd"], &fragment["imm"]),
        _ => catchall(state),
    }
}

fn add(state: memory::State, rs: &str, rt: &str, rd: &str) -> memory::State {
    let rs_value = state.get_register(rs);
    let rt_value = state.get_register(rt);
    let resulting_value = rs_value + rt_value;
    return memory::write_to_register(state, rd, resulting_value);
}

fn load_immediate(state: memory::State, rd: &str, imm: &str) -> memory::State {
    return memory::write_to_register(state, rd, imm.parse().unwrap());
}

fn catchall(state: memory::State) -> memory::State {
    println!("NOT_IMPLEMENTED");
    return state.clone();
}
