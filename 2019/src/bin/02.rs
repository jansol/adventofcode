use aoc2019::intcode::exec as exec_intcode;

const INPUT: &str = include_str!("input/02.txt");

fn main() {
    let program = INPUT.trim().split_terminator(",").map(|l| l.parse::<i64>().unwrap()).collect::<Vec<i64>>();

    // Part 1
    let mut ram = program.clone();
    // "restore program to its state before the 1202 alarm"
    ram[1] = 12;
    ram[2] = 2;
    exec_intcode(&mut ram[..]);

    // Answer with my input was 4714701
    println!("Part 1) Value at position 0: {}\n", ram[0]);



    // Part 2
    let mut result: i64 = 0;
    let target: i64 = 19690720;
    let mut noun: i64 = -1;
    let mut verb: i64 = -1;
    while result != target && noun < 99 {
        noun += 1;
        verb = -1;

        while result != target && verb < 99 {
            verb += 1;

            ram = program.clone();
            ram[1] = noun;
            ram[2] = verb;
            exec_intcode(&mut ram);
            result = ram[0];
        }
    }

    // Answer with my input was 5121
    println!("Part 2) 100*noun({}) + verb({}) = {}", noun, verb, 100*noun+verb);
}

#[cfg(test)]
mod tests {
    use super::exec_intcode;

    #[test]
    fn day02_part1_example_0() {
        let mut program = vec![1,9,10,3,2,3,11,0,99,30,40,50];
        exec_intcode(&mut program[..]);
        assert_eq!(program, vec![3500,9,10,70,2,3,11,0,99,30,40,50])
    }

    #[test]
    fn day02_part1_example_1() {
        let mut program = vec![1,0,0,0,99];
        exec_intcode(&mut program[..]);
        assert_eq!(program, vec![2,0,0,0,99]);
    }

    #[test]
    fn day02_part1_example_2() {
        let mut program = vec![2,3,0,3,99];
        exec_intcode(&mut program[..]);
        assert_eq!(program, vec![2,3,0,6,99]);
    }

    #[test]
    fn day02_part1_example_3() {
        let mut program = vec![2,4,4,5,99,0];
        exec_intcode(&mut program[..]);
        assert_eq!(program, vec![2,4,4,5,99,9801]);
    }

    #[test]
    fn day02_part1_example_4() {
        let mut program = vec![1,1,1,4,99,5,6,0,99];
        exec_intcode(&mut program[..]);
        assert_eq!(program, vec![30,1,1,4,2,5,6,0,99]);
    }
}
