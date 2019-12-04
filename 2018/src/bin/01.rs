use std::collections::HashSet;

const INPUT: &str = include_str!("input/01.txt");

fn main() {
    let deltas = INPUT.lines().map(|l| l.parse::<i64>().unwrap()).collect::<Vec<i64>>();

    // Part 1
    let freq: i64 = deltas.iter().sum();

    // Answer with my input was 543
    println!("Part 1)\nFinal frequency: {}\n", freq);

    // Part 2
    let mut seen_freqs = HashSet::new();
    let mut freq = 0;
    let mut i = 0;
    loop {
        if seen_freqs.contains(&freq) {
            break;
        }

        seen_freqs.insert(freq);
        freq += deltas[i];
        i = (i+1) % (deltas.len());
    }

    // Answer with my input was 621
    println!("Part 2)\nFirst repeated frequency: {}", freq);
}
