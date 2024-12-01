#[derive(Debug)]
pub struct Pair {
    pub va: Vec<u32>,
    pub vb: Vec<u32>,
}

impl std::fmt::Display for Pair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} : {:?}", self.va, self.vb)
    }
}

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Pair {
    let mut va: Vec<u32> = Vec::new();
    let mut vb: Vec<u32> = Vec::new();

    input.lines().for_each(|l| {
        let mut s = l.trim().split("   ").map(|d| d.parse().unwrap());
        va.push(s.next().unwrap());
        vb.push(s.next().unwrap());
    });

    va.sort();
    vb.sort();

    Pair { va, vb }
}

#[aoc(day1, part1)]
pub fn solve_pt1(input: &Pair) -> u32 {
    let mut total: u32 = 0;

    for i in 0..input.va.len() {
        let (a, b) = (input.va.get(i).unwrap(), input.vb.get(i).unwrap());
        if a > b {
            total += a - b;
        } else {
            total += b - a;
        }
    }

    total
}

#[aoc(day1, part2)]
pub fn solve_pt2(input: &Pair) -> u32 {
    let va = &input.va;
    let vb = &input.vb;

    let mut p1 = 0;
    let mut p2 = 0;

    let mut score = 0;

    while p1 != va.len() - 1 {
        let check = va.get(p1).unwrap();

        let mut count = 0;

        while p2 != vb.len() - 1 {
            let sign = vb.get(p2).unwrap();
            if sign > check {
                break;
            } else if sign == check {
                count += 1;
                p2 += 1;
                continue;
            } else {
                p2 += 1;
                continue;
            }
        }

        score += check * count;

        p1 += 1;
        p2 = 0;
    }

    score
}
