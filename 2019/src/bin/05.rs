use aoc2019::intcode;

const INPUT: &str = include_str!("input/05.txt");

fn main() {
    let program = INPUT.trim().split_terminator(",").map(|l| l.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    // Part 1
    let mut ram = program.clone();
    let output = intcode::exec(&mut ram[..], &[1]);
    assert!(output[..output.len()-1].iter().all(|&x| x == 0));

    // Answer with my input was 15386262
    println!("Part 1) Program diagnostic code: {:?}\n", output.last().unwrap());



    // Part 2
    let mut ram = program.clone();
    let output = intcode::exec(&mut ram[..], &[5]);
    assert!(output.len() == 1);

    // Answer with my input was 10376124
    println!("Part 2) Program diagnostic code: {}\n", output.first().unwrap());
}

#[cfg(test)]
mod tests {
    use aoc2019::intcode;

    #[test]
    fn day05_part1_example_0() {
        let mut program = [3,0,4,0,99];
        let inputs = [42];
        let expected = &inputs;
        let output = intcode::exec(&mut program[..], &inputs);
        assert_eq!(output[..], expected[..]);
    }
    
    #[test]
    fn day05_part1_example_1() {
        let mut program = [1002,4,3,4,33];
        let _ = intcode::exec(&mut program[..], &[]);
        assert_eq!(program[4], 99);
    }
    
    #[test]
    fn day05_part1_example_2() {
        let mut program = [1101,100,-1,4,0];
        let _ = intcode::exec(&mut program[..], &[]);
        assert_eq!(program[4], 99);
    }
    
    #[test]
    fn day05_part2_example_1() {
        let program = [3,9,8,9,10,9,4,9,99,-1,8];

        let mut ram = program.clone();
        let output = intcode::exec(&mut ram[..], &[0]);
        assert_eq!(output, [0]);

        let mut ram = program.clone();
        let output = intcode::exec(&mut ram[..], &[8]);
        assert_eq!(output, [1]);
    }
    
    #[test]
    fn day05_part2_example_2() {
        let program = [3,9,7,9,10,9,4,9,99,-1,8];
        
        let mut ram = program.clone();
        let output = intcode::exec(&mut ram[..], &[7]);
        assert_eq!(output, [1]);
        
        let mut ram = program.clone();
        let output = intcode::exec(&mut ram[..], &[8]);
        assert_eq!(output, [0]);
    }
    
    #[test]
    fn day05_part2_example_3() {
        let program = [3,3,1108,-1,8,3,4,3,99];
        
        let mut ram = program.clone();
        let output = intcode::exec(&mut ram[..], &[0]);
        assert_eq!(output, [0]);
        
        let mut ram = program.clone();
        let output = intcode::exec(&mut ram[..], &[8]);
        assert_eq!(output, [1]);
    }
    
    #[test]
    fn day05_part2_example_4() {
        let program = [3,3,1107,-1,8,3,4,3,99];
        
        let mut ram = program.clone();
        let output = intcode::exec(&mut ram[..], &[7]);
        assert_eq!(output, [1]);
        
        let mut ram = program.clone();
        let output = intcode::exec(&mut ram[..], &[8]);
        assert_eq!(output, [0]);
    }
    
    #[test]
    fn day05_part2_example_5() {
        let program = [3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9];
        
        let mut ram = program.clone();
        let output = intcode::exec(&mut ram[..], &[42]);
        assert_eq!(output, [1]);
        
        let mut ram = program.clone();
        let output = intcode::exec(&mut ram[..], &[0]);
        assert_eq!(output, [0]);
    }
    
    #[test]
    fn day05_part2_example_6() {
        let program = [3,3,1105,-1,9,1101,0,0,12,4,12,99,1];
        
        let mut ram = program.clone();
        let output = intcode::exec(&mut ram[..], &[42]);
        assert_eq!(output, [1]);
        
        let mut ram = program.clone();
        let output = intcode::exec(&mut ram[..], &[0]);
        assert_eq!(output, [0]);
    }
    
    #[test]
    fn day05_part2_example_7() {
        let program = [3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
        1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
        999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99];
        
        let mut ram = program.clone();
        let output = intcode::exec(&mut ram[..], &[7]);
        assert_eq!(output, [999]);
        
        let mut ram = program.clone();
        let output = intcode::exec(&mut ram[..], &[8]);
        assert_eq!(output, [1000]);
        
        let mut ram = program.clone();
        let output = intcode::exec(&mut ram[..], &[9]);
        assert_eq!(output, [1001]);
    }
}
