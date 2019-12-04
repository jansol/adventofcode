enum Step {
    L(i32),
    R(i32),
}
use Step::*;

fn left(orientation: (i32, i32)) -> (i32, i32) {
    let (x,y) = orientation;
    (-y,x)
}

fn right(orientation: (i32, i32)) -> (i32, i32) {
    let (x,y) = orientation;
    (y,-x)
}

fn main() {
    println!("Hello, advent!");

    let steps = vec![
        R(2), L(3), R(2), R(4), L(2), L(1), R(2), R(4), R(1), L(4), L(5), R(5), R(5), R(2), R(2),
        R(1), L(2), L(3), L(2), L(1), R(3), L(5), R(187), R(1), R(4), L(1), R(5), L(3), L(4),
        R(50), L(4), R(2), R(70), L(3), L(2), R(4), R(3), R(194), L(3), L(4), L(4), L(3), L(4),
        R(4), R(5), L(1), L(5), L(4), R(1), L(2), R(4), L(5), L(3), R(4), L(5), L(5), R(5), R(3),
        R(5), L(2), L(4), R(4), L(1), R(3), R(1), L(1), L(2), R(2), R(2), L(3), R(3), R(2), R(5),
        R(2), R(5), L(3), R(2), L(5), R(1), R(2), R(2), L(4), L(5), L(1), L(4), R(4), R(3), R(1),
        R(2), L(1), L(2), R(4), R(5), L(2), R(3), L(4), L(5), L(5), L(4), R(4), L(2), R(1), R(1),
        L(2), L(3), L(2), R(2), L(4), R(3), R(2), L(1), L(3), L(2), L(4), L(4), R(2), L(3), L(3),
        R(2), L(4), L(3), R(4), R(3), L(2), L(1), L(4), R(4), R(2), L(4), L(4), L(5), L(1), R(2),
        L(5), L(2), L(3), R(2), L(2)
    ];

    let mut pos = (0i32, 0i32);
    let mut dir = (0, 1);

    for s in steps {
        let count = match s {
            R(x) => x,
            L(x) => x,
        };
        dir = match s {
            L(_) => left(dir),
            R(_) => right(dir),
        };

        pos = (pos.0 + (dir.0 * count), pos.1 + (dir.1 * count));
    }

    println!("{:?} {}", pos, pos.0.abs()+pos.1.abs());



    let mut pos = (0i32, 0i32);
    let mut visits = HashMap::new();
    let mut dir = (0, 1);
    let mut revisit = (0i32, 0i32);
    let mut found = false;

    for s in steps {
        let count = match s {
            R(x) => x,
            L(x) => x,
        };
        dir = match s {
            L(_) => left(dir),
            R(_) => right(dir),
        };

        for i in 1..count+1 {
            let dest = (pos.0 + (dir.0 * i), pos.1 + (dir.1 * i));
            if !visits.contains_key(&dest) {
                visits.insert(dest, 1);
            } else if !found {
                revisit = dest;
                found = true;
            }
        }
        pos = (pos.0 + (dir.0 * count), pos.1 + (dir.1 * count));
    }

    println!("{:?} {}", revisit, revisit.0.abs()+revisit.1.abs());
}
