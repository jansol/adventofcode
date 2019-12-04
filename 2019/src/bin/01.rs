const INPUT: &str = include_str!("input/01.txt");

fn main() {
    let masses = INPUT.lines().map(|l| l.parse::<i64>().unwrap()).collect::<Vec<i64>>();

    // Part 1
    let sum: i64 = masses.iter().map(|x| (x / 3) - 2).sum();

    // Answer with my input was 3348430
    println!("Part 1) Total fuel: {}\n", sum);



    // Part 2
    fn fuel_rec(x: &i64) -> i64 {
        if *x > 0 {
            let tmp = ((*x / 3 ) - 2).max(0);
            tmp + fuel_rec(&tmp)
        } else {
            0
        }
    };
    let sum: i64 = masses.iter().map(fuel_rec).sum();

    // Answer with my input was 5019767
    println!("Part 2) Total fuel: {}\n", sum);
}
