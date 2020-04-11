use crate::memory;
use crate::utils;
use regex;

pub fn exec(state: memory::State, command: &str) -> memory::State {
    let command_fragments = utils::split_fragment(command);
    let new_state = exec_fragment(state, command_fragments);

    let initial_counter = state.get_program_counter();
    let new_counter = state.get_program_counter();

    let has_pc_changed = initial_counter != new_counter;

    if has_pc_changed {
        return new_state;
    } else {
        return memory::set_program_counter(new_state, new_counter + 4);
    }
}

fn exec_fragment(state: memory::State, fragment: regex::Captures) -> memory::State {
    match &fragment["op"] {
        "add" => add(state, &fragment["rd"], &fragment["rs"], &fragment["rt"]),
        "addi" => add_immediate(state, &fragment["rd"], &fragment["rs"], &fragment["imm"]),
        "li" => load_immediate(state, &fragment["rd"], &fragment["imm"]),
        "sub" => subtract(state, &fragment["rd"], &fragment["rs"], &fragment["rt"]),
        _ => catchall(state),
    }
}

fn parse_immediate(imm: &str) -> i32 {
    return imm.parse().unwrap();
}

fn add(state: memory::State, rd: &str, rs: &str, rt: &str) -> memory::State {
    let rs_value = state.get_register(rs);
    let rt_value = state.get_register(rt);
    let resulting_value = rs_value + rt_value;
    return memory::write_to_register(state, rd, resulting_value);
}

fn add_immediate(state: memory::State, rd: &str, rs: &str, imm: &str) -> memory::State {
    let rs_value = state.get_register(rs);
    let resulting_value = rs_value + parse_immediate(imm);
    return memory::write_to_register(state, rd, resulting_value);
}

fn load_immediate(state: memory::State, rd: &str, imm: &str) -> memory::State {
    return memory::write_to_register(state, rd, parse_immediate(imm));
}

fn subtract(state: memory::State, rd: &str, rs: &str, rt: &str) -> memory::State {
    let rs_value = state.get_register(rs);
    let rt_value = state.get_register(rt);
    let resulting_value = rs_value - rt_value;
    return memory::write_to_register(state, rd, resulting_value);
}

fn catchall(state: memory::State) -> memory::State {
    println!("NOT_IMPLEMENTED");
    return state.clone();
}
