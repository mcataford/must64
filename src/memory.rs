use crate::constants;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct State {
    program_counter: i32,
    registers: [i32; 32],
}

impl State {
    pub fn dump(&self) {
        println!("PC: 0x{:08X?}", self.program_counter);
        println!("{:?}", self.registers);
    }

    pub fn get_register(&self, register: &str) -> i32 {
        let index = map_register_to_index(register);
        self.registers[index]
    }

    pub fn get_program_counter(&self) -> i32 {
        return self.program_counter;
    }
}

pub fn setup_memory() -> State {
    return State {
        program_counter: constants::TEXT_START_ADDR,
        registers: [0; 32],
    };
}

pub fn write_to_register(initial_state: State, register: &str, value: i32) -> State {
    let register_index = map_register_to_index(register);
    let mut new_regs = initial_state.registers.clone();
    new_regs[register_index] = value;
    return State {
        registers: new_regs,
        program_counter: initial_state.program_counter.clone(),
    };
}

pub fn set_program_counter(state: State, value: i32) -> State {
    return State {
        registers: state.registers.clone(),
        program_counter: value,
    };
}

fn map_register_to_index(register: &str) -> usize {
    match register {
        "$zero" => 0,
        "$at" => 1,
        "$v0" => 2,
        "$v1" => 3,
        "$a0" => 4,
        "$a1" => 5,
        "$a2" => 6,
        "$a3" => 7,
        "$t0" => 8,
        "$t1" => 9,
        "$t2" => 10,
        "$t3" => 11,
        "$t4" => 12,
        "$t5" => 13,
        "$t6" => 14,
        "$t7" => 15,
        "$s0" => 16,
        "$s1" => 17,
        "$s2" => 18,
        "$s3" => 19,
        "$s4" => 20,
        "$s5" => 21,
        "$s6" => 22,
        "$s7" => 23,
        "$t8" => 24,
        "$t9" => 25,
        "$k1" => 26,
        "$k2" => 27,
        "$gp" => 28,
        "$sp" => 29,
        "$fp" => 30,
        "$ra" => 31,
        &_ => 0,
    }
}
