pub fn exec(ram: &mut [i64]) {
    let mut pc = 0;
    let mut halt = false;
    while !halt {
        let op = ram[pc];
        match op {
            1 => /* ADD a, b, dst */ {
                let a = ram[(pc+1) as usize] as usize;
                let b = ram[(pc+2) as usize] as usize;
                let dst = ram[(pc+3) as usize] as usize;
                ram[dst] = ram[a] + ram[b];
            },
            2 => /* MUL a, b, dst */ {
                let a = ram[(pc+1) as usize] as usize;
                let b = ram[(pc+2) as usize] as usize;
                let dst = ram[(pc+3) as usize] as usize;
                ram[dst] = ram[a] * ram[b];
            },
            99 => /* HALT */ {
                halt = true
            },
            x => /* ERROR */ {
                panic!("INVALID OPCODE: {}, AT ADDR: {}", x, pc);
            }
        }
        pc += 4;
    }
}
