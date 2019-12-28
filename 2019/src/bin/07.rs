use aoc2019::intcode;

const INPUT: &str = include_str!("input/07.txt");

fn main() {
    // Part 1
    let program = INPUT.trim().split_terminator(",").map(|l| l.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    let sgn = max_signal(&program);

    // Answer with my input was 116680
    println!("Part 1) Max thruster signal: {}\n", sgn);

    // Part 2
    let res = 42;

    // Answer with my input was 42
    println!("Part 2) Final result: {}\n", res);
}

/// Generate all possible permutations of input using Heap's algorithm
fn permute_rec(a: &mut [i64; 5], k: usize, out: &mut Vec<[i64; 5]>) {
    if k == 1 {
        out.push(a.clone());
    } else {
        permute_rec(a, k - 1, out);

        for i in 0..k - 1 {
            if k & 1 == 0 {
                a.swap(i, k - 1);
            } else {
                a.swap(0, k - 1);
            }

            permute_rec(a, k - 1, out);
        }
    }
}

fn max_signal(program: &[i64]) -> i64 {
    let mut perms = Vec::new();
    permute_rec(&mut [0, 1, 2, 3, 4], 5, &mut perms);
    perms
        .iter()
        .map(|seq| {
            seq.iter().fold(0, |signal, &phase| {
                let mut ram = program.to_owned();
                *intcode::exec(&mut ram, &[phase, signal]).first().unwrap()
            })
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use aoc2019::intcode;
    use super::max_signal;

    #[test]
    fn day07_part1_example_1() {
        let program = [
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ];
        let reference = 43210;
        assert_eq!(max_signal(&program), reference);
    }

    #[test]
    fn day07_part1_example_2() {
        let program = [
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0,
        ];
        let reference = 54321;
        assert_eq!(max_signal(&program), reference);
    }

    #[test]
    fn day07_part1_example_3() {
        let program = [
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
            33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
        ];
        let reference = 65210;
        assert_eq!(max_signal(&program), reference);
    }
}
