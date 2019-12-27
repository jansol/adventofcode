use std::collections::HashMap;

const INPUT: &str = include_str!("input/06.txt");

fn main() {
    // Part 1
    let orbits = orbit_map(INPUT);
    let sum: u64 = orbits.iter().map(|(satellite, _)| orbit_count_rec(&orbits, satellite)).sum();

    // Answer with my input was 247089
    println!("Part 1) Total number of orbits: {}\n", sum);



    // Part 2
    let dist = dist_between(&orbits, "YOU", "SAN");

    // Answer with my input was 442
    println!("Part 2) Orbital hops to Santa: {}\n", dist);
}

fn orbit_map(input: &str) -> HashMap<String, String> {
    input.trim().lines().map(|l| {
        let s: Vec<&str> = l.split(')').collect();
        (s[1].to_owned(), s[0].to_owned())
    }).collect()
}

fn orbit_count_rec(map: &HashMap<String, String>, start: &str) -> u64 {
    if start == "COM" {
        return 0;
    } else {
        return orbit_count_rec(map, map.get(start).unwrap()) + 1;
    }
}

fn path_to_com<'a>(map: &'a HashMap<String, String>, start: &'a str) -> Vec<&'a str> {
    let mut ptr = start;
    let mut path = Vec::new();
    while ptr != "COM" {
        ptr = map.get(ptr).unwrap();
        path.push(ptr);
    }
    path
}

fn dist_between(map: &HashMap<String, String>, a: &str, b: &str) -> u64 {
    let path_a = path_to_com(&map, a);
    let path_b = path_to_com(&map, b);
    let mut dist_a = 0;
    let mut dist_total = None;
    for a in &path_a {
        let mut dist_b = 0;
        let mut hit = None;
        for b in &path_b {
            if a == b {
                hit = Some(b);
                break;
            }
            dist_b += 1;
        }

        if hit.is_some() {
            dist_total = Some(dist_a + dist_b);
            break;
        }
        dist_a += 1;
    }

    dist_total.unwrap()
}

#[cfg(test)]
mod tests {
    use super::{orbit_map, orbit_count_rec, dist_between};

    #[test]
    fn day06_part1_example_1() {
        let input = r#"
COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
"#;

        let orbits = orbit_map(input);
        let sum: u64 = orbits.iter().map(|(satellite, _)| orbit_count_rec(&orbits, satellite)).sum();

        assert_eq!(sum, 42);
    }

    #[test]
    fn day06_part2_example_1() {
        let input = r#"
COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN
"#;
        
        let orbits = orbit_map(input);
        let dist_total = dist_between(&orbits, "YOU", "SAN");

        assert_eq!(dist_total, 4);
    }
}
