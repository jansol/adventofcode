const INPUT: &str = include_str!("input/02.txt");

fn main() {
    let lines = INPUT.lines().collect::<Vec<&str>>();

    // Part 1
    let checksum = lines.iter().filter(exactly_two).count() * lines.iter().filter(exactly_three).count();

    // Answer with my input was 6150
    println!("Part 1)\nChecksum: {}\n", checksum);

    // Part 2
    let (a, b) = id_pair(&lines);
    let letters = lines[a].as_bytes().iter().zip(lines[b].as_bytes().iter())
        .map(|(&a,&b)| if a == b { Some(a) } else {None})
        .flat_map(|a| a).collect::<Vec<u8>>();
    let letters = String::from_utf8(letters).unwrap();

    // Answer with my input was rteotyxzbodglnpkudawhijsc
    println!("Part 2)\nCommon letters: {}", letters);
}

fn exactly_two(s: &&&str) -> bool {
    let mut chars = [0u8; 256];

    for &b in s.as_bytes() {
        chars[b as usize] += 1;
    }

    chars.iter().filter(|&&x| x == 2).count() > 0
}

fn exactly_three(s: &&&str) -> bool {
    let mut chars = [0u8; 256];

    for &b in s.as_bytes() {
        chars[b as usize] += 1;
    }

    chars.iter().filter(|&&x| x == 3).count() > 0
}

fn different_chars(a: &str, b: &str) -> usize {
    assert_eq!(a.len(), b.len());

    a.as_bytes().iter().zip(b.as_bytes().iter()).map(|(&a,&b)| a != b).filter(|&x| x).count()
}

fn id_pair(lines: &Vec<&str>) -> (usize, usize) {
    for a in 0 .. lines.len()-1 {
        for b in a..lines.len() {
            if different_chars(lines[a], lines[b]) == 1 {
                return (a, b);
            }
        }
    };

    (0, 0)
}
