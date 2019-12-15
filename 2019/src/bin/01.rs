const INPUT: &str = include_str!("input/01.txt");

fn main() {
    let masses = INPUT.lines().map(|l| l.parse::<i64>().unwrap()).collect::<Vec<i64>>();

    // Part 1
    let sum: i64 = masses.iter().map(fuel).sum();

    // Answer with my input was 3348430
    println!("Part 1) Total fuel: {}\n", sum);



    // Part 2
    let sum: i64 = masses.iter().map(fuel_rec).sum();

    // Answer with my input was 5019767
    println!("Part 2) Total fuel: {}\n", sum);
}

fn fuel(x: &i64) -> i64 {
    (x / 3) - 2
}

fn fuel_rec(x: &i64) -> i64 {
    if *x > 0 {
        let tmp = ((*x / 3 ) - 2).max(0);
        tmp + fuel_rec(&tmp)
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::{fuel, fuel_rec};

    #[test]
    fn day01_part1_example_1() {
        let mass = 12;
        let res = 2;
        let f = fuel(&mass);
        assert_eq!(f, res);
    }

    #[test]
    fn day01_part1_example_2() {
        let mass = 14;
        let res = 2;
        let f = fuel(&mass);
        assert_eq!(f, res);
    }

    #[test]
    fn day01_part1_example_3() {
        let mass = 1969;
        let res = 654;
        let f = fuel(&mass);
        assert_eq!(f, res);
    }

    #[test]
    fn day01_part1_example_4() {
        let mass = 100756;
        let res = 33583;
        let f = fuel(&mass);
        assert_eq!(f, res);
    }

    #[test]
    fn day01_part2_example_1() {
        let mass = 14;
        let res = 2;
        let f = fuel_rec(&mass);
        assert_eq!(f, res);
    }

    #[test]
    fn day01_part2_example_2() {
        let mass = 1969;
        let res = 966;
        let f = fuel_rec(&mass);
        assert_eq!(f, res);
    }

    #[test]
    fn day01_part2_example_3() {
        let mass = 100756;
        let res = 50346;
        let f = fuel_rec(&mass);
        assert_eq!(f, res);
    }
}
