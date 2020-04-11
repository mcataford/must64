use crate::memory;
use crate::utils;

pub fn exec(state: memory::State, command: &str) -> memory::State {
    let command_fragments = utils::split_fragment(command);
    return exec_fragment(state, command_fragments)
}

fn exec_fragment(state: memory::State, fragment: Vec<&str>) -> memory::State{
    match fragment[0] {
        "add" => add(state, fragment[1], fragment[2], fragment[3]),
        "li" => load_immediate(state, fragment[1], fragment[2]),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let rs = "$t0";
        let rt = "$t1";
        let rd = "$t2";
        let rs_value = 1;
        let rt_value = 2;

        let state = memory::setup_memory();
        let state_1 = memory::write_to_register(state, rs, rs_value);
        let state_2 = memory::write_to_register(state_1, rt, rt_value);
        let post_add = exec(state_2, "add $t0, $t1, $t2");

        assert_eq!(post_add.get_register(rd), rs_value + rt_value);
    }
}
