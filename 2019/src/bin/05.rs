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
    // let mut ram = program.clone();
    // let output = intcode::exec(&mut ram[..], &[5]);
    // assert!(output.len() == 1);

    // Answer with my input was 42
    //println!("Part 2) Final result: {}\n", output.first().unwrap());
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
}
