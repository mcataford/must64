use crate::constants;
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

fn exec_fragment(state: memory::State, fg: regex::Captures) -> memory::State {
    let op = &fg["op"];
    let rd = if fg.get(2) != None {
        fg.get(2).unwrap().as_str()
    } else {
        ""
    };
    let rs = if fg.get(3) != None {
        fg.get(3).unwrap().as_str()
    } else {
        ""
    };
    let rt = if fg.get(4) != None {
        fg.get(4).unwrap().as_str()
    } else {
        ""
    };
    let imm = if fg.get(5) != None {
        fg.get(5).unwrap().as_str()
    } else {
        ""
    };

    if constants::R_TYPES.contains(&op) {
        return exec_r_type(state, op, rd, rs, rt);
    } else if constants::I_TYPES.contains(&op) {
        return exec_i_type(state, op, rd, rs, imm);
    } else {
        return state;
    }
}

fn parse_immediate(imm: &str) -> i32 {
    return imm.parse().unwrap();
}

fn exec_r_type(state: memory::State, op: &str, rd: &str, rs: &str, rt: &str) -> memory::State {
    let rs_value = state.get_register(rs);
    let rt_value = state.get_register(rt);

    let resulting_value = match op {
        "add" => rs_value + rt_value,
        "and" => rs_value & rt_value,
        "or" => rs_value | rt_value,
        "nor" => !(rs_value | rt_value),
        "sub" => rs_value - rt_value,
        _ => state.get_register(rd),
    };

    return memory::write_to_register(state, rd, resulting_value);
}

fn exec_i_type(state: memory::State, op: &str, rd: &str, rs: &str, imm: &str) -> memory::State {
    let rs_value = state.get_register(rs);
    let imm_value = parse_immediate(imm);

    let resulting_value = match op {
        "addi" => rs_value + imm_value,
        "andi" => rs_value & imm_value,
        "ori" => rs_value | imm_value,
        "li" => imm_value,
        _ => state.get_register(rd),
    };

    return memory::write_to_register(state, rd, resulting_value);
}
