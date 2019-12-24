const INPUT: &str = include_str!("input/03.txt");

#[derive(Debug)]
struct Point {
    pub x: i64,
    pub y: i64,
}

#[derive(Debug)]
struct Segment {
    pub start: Point,
    pub end: Point,
}

fn main() {
    let wires = INPUT.lines().map(|l| parse_wires(l)).collect::<Vec<Vec<(i64, i64)>>>();
    let segments = [
        acc_segments(&wires[0][..]),
        acc_segments(&wires[1][..]),
    ];

    // Part 1
    let min_dist = compute_min_dist(&segments);

    // Answer with my input was 3229
    println!("Part 1) Manhattan distance: {}\n", min_dist);

    // Part 2
    let min_steps = compute_min_steps(&segments);
    
    // Answer with my input was 32132
    println!("Part 2) Least steps taken: {}\n", min_steps);
    
}

fn parse_wires(l: &str) -> Vec<(i64, i64)> {
    l.split_terminator(",").map(|w| {
        let dir = w.chars().nth(0).unwrap();
        let num = w[1..].parse::<i64>().unwrap();
        match dir {
            'L' => (-num, 0),
            'R' => (num, 0),
            'U' => (0, num),
            'D' => (0, -num),
            _ => panic!(),
        }
    }).collect::<Vec<(i64, i64)>>()
}

fn acc_segments(dirs: &[(i64, i64)]) -> Vec<Segment> {
    let mut pos = (0, 0);
    let mut segments = Vec::with_capacity(dirs.len());
    for i in 0..dirs.len() {
        let end = (pos.0 + dirs[i].0, pos.1 + dirs[i].1);
        segments.push(Segment {start: Point{x: pos.0, y: pos.1}, end: Point{x: end.0, y: end.1}});
        pos = end;
    }
    
    segments
}

fn compute_min_dist(segments: &[Vec<Segment>; 2]) -> i64 {
    let mut md = std::i64::MAX;

    for a in segments[0].iter() {
        for b in segments[1].iter() {
            if let Some((x, y)) = intersect(a, b) {
                md = md.min(x.abs()+y.abs());
            }
        }
    }

    md
}

fn compute_min_steps(segments: &[Vec<Segment>; 2]) -> i64 {
    let mut steps_a = 0;
    let mut steps_min = std::i64::MAX;

    for a in segments[0].iter() {
        
        let mut steps_b = 0;
        for b in segments[1].iter() {
            if let Some((x,y)) = intersect(a, b) {
                steps_min = steps_min.min(
                    steps_a + steps_b +
                    (a.start.x - x).abs() + (a.start.y - y).abs() +
                    (b.start.x - x).abs() + (b.start.y - y).abs()
                );
            }

            steps_b += (b.start.x - b.end.x).abs() + (b.start.y - b.end.y).abs();
        }

        steps_a += (a.start.x - a.end.x).abs() + (a.start.y - a.end.y).abs();
    }
    
    steps_min
}

fn intersect(a: &Segment, b: &Segment) -> Option<(i64, i64)> {
    // First segments don't count
    if (a.start.x == 0 && a.start.y == 0) || (b.start.x == 0 && b.start.y == 0) {
        return None;
    }

    // sort start & end points to simplify comparisons
    let a_start = Point{x: a.start.x.min(a.end.x), y: a.start.y.min(a.end.y)};
    let a_end = Point{x: a.start.x.max(a.end.x), y: a.start.y.max(a.end.y)};
    let b_start = Point{x: b.start.x.min(b.end.x), y: b.start.y.min(b.end.y)};
    let b_end = Point{x: b.start.x.max(b.end.x), y: b.start.y.max(b.end.y)};
    
    // Assuming lines are perpendicular
    if a_start.x <= b_end.x && a_end.x >= b_start.x
    && a_start.y <= b_end.y && a_end.y >= b_start.y {
        let intersection = Point {
            // lines are axis aligned so just check which one is which
            x: if a_start.x == a_end.x {a_start.x} else {b_start.x},
            y: if a_start.y == a_end.y {a_start.y} else {b_start.y},
        };
        
        Some((intersection.x, intersection.y))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::{compute_min_dist, compute_min_steps, acc_segments, parse_wires};

    #[test]
    fn day03_part1_example_0() {
        let input = ["R8,U5,L5,D3", "U7,R6,D4,L4"];
        let answer = 6;
        
        let wires = [parse_wires(input[0]), parse_wires(input[1])];
        let segments = [
            acc_segments(&wires[0][..]),
            acc_segments(&wires[1][..]),
        ];
        dbg!(&segments);
        
        assert_eq!(compute_min_dist(&segments), answer);
    }

    #[test]
    fn day03_part1_example_1() {
        let input = ["R75,D30,R83,U83,L12,D49,R71,U7,L72", "U62,R66,U55,R34,D71,R55,D58,R83"];
        let answer = 159;

        let wires = [parse_wires(input[0]), parse_wires(input[1])];
        let segments = [
            acc_segments(&wires[0][..]),
            acc_segments(&wires[1][..]),
        ];

        assert_eq!(compute_min_dist(&segments), answer);
    }
    
    #[test]
    fn day03_part1_example_2() {
        let input = ["R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51", "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"];
        let answer = 135;

        let wires = [parse_wires(input[0]), parse_wires(input[1])];
        let segments = [
            acc_segments(&wires[0][..]),
            acc_segments(&wires[1][..]),
        ];

        assert_eq!(compute_min_dist(&segments), answer);
    }
    
    #[test]
    fn day03_part2_example_0() {
        let input = ["R8,U5,L5,D3", "U7,R6,D4,L4"];
        let answer = 30;
        
        let wires = [parse_wires(input[0]), parse_wires(input[1])];
        let segments = [
        acc_segments(&wires[0][..]),
        acc_segments(&wires[1][..]),
        ];
        dbg!(&segments);
        
        assert_eq!(compute_min_steps(&segments), answer);
    }
    
    #[test]
    fn day03_part2_example_1() {
        let input = ["R75,D30,R83,U83,L12,D49,R71,U7,L72", "U62,R66,U55,R34,D71,R55,D58,R83"];
        let answer = 610;
        
        let wires = [parse_wires(input[0]), parse_wires(input[1])];
        let segments = [
        acc_segments(&wires[0][..]),
        acc_segments(&wires[1][..]),
        ];
        
        assert_eq!(compute_min_steps(&segments), answer);
    }
    
    #[test]
    fn day03_part2_example_2() {
        let input = ["R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51", "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"];
        let answer = 410;
        
        let wires = [parse_wires(input[0]), parse_wires(input[1])];
        let segments = [
        acc_segments(&wires[0][..]),
        acc_segments(&wires[1][..]),
        ];
        
        assert_eq!(compute_min_steps(&segments), answer);
    }
}
