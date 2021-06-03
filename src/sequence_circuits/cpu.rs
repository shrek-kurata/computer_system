use crate::{
    adders::alu,
    gates::{and, mux, mux16},
};

use super::{counter_circuit::Counter, register::Register, word::Word};

pub struct CPU {
    a_register: Register,
    d_register: Register,
    out_memory: Word,
    write_dist: Word,
    pc: Counter,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            a_register: Register::new(),
            d_register: Register::new(),
            out_memory: [false; 16],
            write_dist: [false; 16],
            pc: Counter::new(),
        }
    }

    pub fn cycle(&mut self, in_memory: Word, instruction: Word, reset: bool) {
        let is_c_command = instruction[15]; // A命令かC命令か
        let use_m = and::calc(is_c_command, instruction[3]);
        let zero_d = and::calc(is_c_command, instruction[4]);
        let negate_d = and::calc(is_c_command, instruction[5]);
        let zero_a = and::calc(is_c_command, instruction[6]);
        let negate_a = and::calc(is_c_command, instruction[7]);
        let f = and::calc(is_c_command, instruction[8]);
        let negate_alu = and::calc(is_c_command, instruction[9]);
        let d = self.d_register.out();
        let a = mux16::calc(self.a_register.out(), in_memory, use_m);
        alu::calc(d, a, zero_d, negate_d, zero_a, negate_a, f, negate_alu);
    }
    /**
    出力 (i xx a cccccc ddd jjj) \
    i: 命令の種類 false=>A命令, true=>C命令 \
    C命令の場合、以下のように読み取られる。A命令の場合、定数値として解釈される。\
    a,c => comp領域 \
    d => dest領域 \
    j => jump領域
    */
    pub fn decode(instruction: Word) -> (bool, [bool; 2], bool, [bool; 6], [bool; 3], [bool; 3]) {
        (
            instruction[0],
            [instruction[1], instruction[2]],
            instruction[3],
            [
                instruction[4],
                instruction[5],
                instruction[6],
                instruction[7],
                instruction[8],
                instruction[9],
            ],
            [instruction[10], instruction[11], instruction[12]],
            [instruction[13], instruction[14], instruction[15]],
        )
    }
}
