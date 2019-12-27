use std::convert::TryInto;

#[derive(Debug)]
enum Instruction {
    Add(i64, i64, usize),
    Mul(i64, i64, usize),
    Input(usize),
    Output(i64),
    Halt,
    Unknown(i64),
}
use Instruction::*;

pub fn exec(ram: &mut [i64], inputs: &[i64]) -> Vec<i64> {
    let mut pc = 0;
    let mut outputs = Vec::new();
    let mut inputs = inputs;

    loop {
        let (op, advance) = decode(ram, pc);

        match op {
            Add(a, b, dst) => {
                ram[dst] = a + b;
            },
            Mul(a, b, dst) => {
                ram[dst] = a * b;
            },
            Input(dst) => {
                ram[dst] = inputs[0];
                inputs = &inputs[1..];
            },
            Output(a) => {
                outputs.push(a);
            },
            Halt => {
                return outputs
            },
            Unknown(x) => {
                panic!("INVALID OPCODE: {}, AT ADDR: {}", x, pc);
            }
        }
        pc += advance;
    }
}

fn decode(mem: &[i64], pc: usize) -> (Instruction, usize) {
    macro_rules! param {
        ($mode:ident, $offset:expr) => {
            match $mode {
                0x30 => {
                    let addr: usize = mem[pc+$offset].try_into().unwrap();
                    mem[addr]
                },
                0x31 => mem[pc+$offset],
                _ => panic!("INVALID PARAMETER MODE"),
            }
        }
    }

    let tmp = format!("{:05}", mem[pc]);
    let mode_1 = tmp.as_bytes()[2];
    let mode_2 = tmp.as_bytes()[1];
    let mode_3 = tmp.as_bytes()[0];
    let code: i64 = tmp[3..5].parse().unwrap();

    match code {
        1 => (Add(param!(mode_1, 1), param!(mode_2, 2), mem[pc+3].try_into().unwrap()), 4),
        2 => (Mul(param!(mode_1, 1), param!(mode_2, 2), mem[pc+3].try_into().unwrap()), 4),
        3 => (Input(mem[pc+1].try_into().unwrap()), 2),
        4 => (Output(param!(mode_1, 1)), 2),
        99 => (Halt, 0),
        x => (Unknown(x), 1),
    }
}
