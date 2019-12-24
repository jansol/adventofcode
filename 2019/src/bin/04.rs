const INPUT: &str = include_str!("input/04.txt");

fn main() {
    let limits = INPUT.trim().split('-').collect::<Vec<&str>>();
    let limits = [limits[0].parse().unwrap(), limits[1].parse().unwrap()];

    // Part 1
    let cnt: u64 = (limits[0]..limits[1]).map(|n: u64| is_valid(n.to_string().as_bytes()) as u64).sum();

    // Answer with my input was 966
    println!("Part 1) Valid passwords: {}\n", cnt);
    


    // Part 2
    let cnt: u64 = (limits[0]..limits[1]).map(|n: u64| is_valid2(n.to_string().as_bytes()) as u64).sum();

    // Answer with my input was 628
    println!("Part 2) Valid passwords: {}\n", cnt);
}

fn bruteforce(remaining_digits: i64,  upper: i64) -> usize {
   0
}

fn is_valid(pw: &[u8]) -> bool {
    let flags = pw.iter().map(|&d| d as i8).fold((-1i8, false, false),
        |(prev, has_pair, decreases), digit|
            (digit, digit == prev || has_pair, digit < prev || decreases));

    /* has_pair && !decreases */
    flags.1 && !flags.2
}

fn is_valid2(pw: &[u8]) -> bool {
    let flags = pw.iter().map(|&d| d as i8).fold((-1i8, -1i8, false, None, false),
    |(prev2, prev, pair_prev, confirmed_pair_digit, decreases), digit|
        (prev, digit,
        digit == prev && digit != prev2,
        if pair_prev && digit != prev { Some(prev) } else { confirmed_pair_digit },
        digit < prev || decreases)
    );
    
    /* confirmed earlier pair digit or last two digits were a pair && !decreases */
    (flags.3.is_some() || flags.2) && !flags.4
}

#[cfg(test)]
mod tests {
    use super::{is_valid, is_valid2};

    #[test]
    fn day04_part1_example_1() {
        assert_eq!(is_valid(b"111111"), true);
    }

    #[test]
    fn day04_part1_example_2() {
        assert_eq!(is_valid(b"223450"), false);
    }

    #[test]
    fn day04_part1_example_3() {
        assert_eq!(is_valid(b"123789"), false);
    }
    
    #[test]
    fn day04_part2_example_1() {
        assert_eq!(is_valid2(b"112233"), true);
    }
    
    #[test]
    fn day04_part2_example_2() {
        assert_eq!(is_valid2(b"123444"), false);
    }
    
    #[test]
    fn day04_part2_example_3() {
        assert_eq!(is_valid2(b"111122"), true);
    }
    
    #[test]
    fn day04_part2_example_4() {
        assert_eq!(is_valid2(b"112222"), true);
    }
}
