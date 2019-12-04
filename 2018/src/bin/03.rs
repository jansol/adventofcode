const INPUT: &str = include_str!("input/03.txt");

#[derive(PartialEq, Eq, Clone, Copy)]
enum ClaimState {
    Unclaimed,
    Unique(usize),
    Conflicting,
}
use self::ClaimState::*;

#[derive(Clone, Copy)]
struct Rect {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

struct Claim {
    id: usize,
    area: Rect,
}

fn main() {
    let mut fabric = vec![vec![Unclaimed; 1000]; 1000];
    let claims = INPUT.lines().map(parse_claim).collect::<Vec<Claim>>();
    for c in &claims {
        claim_region(&mut fabric, c.id, c.area);
    }

    // Part 1
    let result: usize = fabric.iter().map(|row| row.iter().filter(|x| **x == Conflicting).count()).sum();

    // Answer with my input was 113576
    println!("Part 1)\nConflicting square inches: {}\n", result);

    // Part 2
    let id = claims.iter().filter(|c| !has_overlap(&fabric, c.area)).next().unwrap().id;

    // Answer with my input was 825
    println!("Part 2)\nNon-overlapping claim: {}", id);
}

fn parse_claim(line: &str) -> Claim {
    let words = line.split(' ').collect::<Vec<&str>>();
    let id = words[0].trim_start_matches('#').parse().unwrap();
    let (x, y) = {
        let parts = words[2].trim_end_matches(':').split(',').map(|x| x.parse().unwrap()).collect::<Vec<usize>>();
        (parts[0], parts[1])
    };
    let (width, height) = {
        let parts = words[3].split('x').map(|x| x.parse().unwrap()).collect::<Vec<usize>>();
        (parts[0], parts[1])
    };

    Claim {
        id,
        area: Rect {
            x, y, width, height
        }
    }
}

fn claim_region(fabric: &mut Vec<Vec<ClaimState>>, id: usize, area: Rect) {
    let len_y = fabric.len();
    fabric.extend(vec![vec![Unclaimed; 1000]; len_y - (area.y+area.height).max(len_y)]);
    for y in area.y .. area.y+area.height {
        let len_x = fabric[y].len();
        fabric[y].extend(vec![Unclaimed; len_x - (area.y+area.height).max(len_x)]);

        for x in area.x .. area.x+area.width {
            fabric[y][x] = match fabric[y][x] {
                Unclaimed => Unique(id),
                _ => Conflicting,
            };
        }
    }
}

fn has_overlap(fabric: &Vec<Vec<ClaimState>>, area: Rect) -> bool {
    for y in area.y .. area.y+area.height {
        for x in area.x .. area.x+area.width {
            if fabric[y][x] == Conflicting {
                return true
            }
        }
    }

    return false
}
