use aoc::{Grid, Regex};

fn main() {
    aoc::run_parts(one, two);
}

fn one(input: &str) -> u64 {
    let ((dim_x, dim_y), robots) = parse_input(input);
    let (mid_x, mid_y) = (dim_x / 2, dim_y / 2);

    let mut q = (0, 0, 0, 0);

    for (px, py, vx, vy) in robots {
        let px = (px + vx * 100).rem_euclid(dim_x);
        let py = (py + vy * 100).rem_euclid(dim_y);

        match (px - mid_x, py - mid_y) {
            (1.., 1..) => q.0 += 1,
            (..=-1, 1..) => q.1 += 1,
            (..=-1, ..=-1) => q.2 += 1,
            (1.., ..=-1) => q.3 += 1,
            _ => continue,
        }
    }

    q.0 * q.1 * q.2 * q.3
}

fn two(input: &str) -> &'static str {
    let ((dim_x, dim_y), mut robots) = parse_input(input);
    let mut bathroom = Grid::new_cloned(false, (dim_y as usize, dim_x as usize));

    for i in 1..101 * 103 {
        for (px, py, vx, vy) in &mut robots {
            *px = (*px + *vx).rem_euclid(dim_x);
            *py = (*py + *vy).rem_euclid(dim_y);
        }

        bathroom.tiles_mut().fill(false);

        for (px, py, ..) in &robots {
            bathroom[(*py as usize, *px as usize)] = true;
        }

        let mut max_consecutive = 0;

        for line in bathroom.lines() {
            let mut consecutive = 0;
            for tile in line {
                if *tile {
                    consecutive += 1;
                    if consecutive > max_consecutive {
                        max_consecutive = consecutive;
                    }
                } else {
                    consecutive = 0;
                }
            }
        }

        if max_consecutive > 10 {
            println!("jollyman #{i}");
            for line in bathroom.lines() {
                for tile in line {
                    if *tile {
                        print!("#");
                    } else {
                        print!(" ");
                    }
                }
                println!();
            }
        }
    }

    "those who are jolly"
}

fn parse_input(input: &str) -> ((i64, i64), Vec<(i64, i64, i64, i64)>) {
    let re = Regex::new(r"dim=(\d+),(\d+)").unwrap(); // not part of aoc input; included manually

    let (_, [dim_x, dim_y]) = re.captures(input).unwrap().extract();
    let dim = (dim_x.parse().unwrap(), dim_y.parse().unwrap());

    let re = Regex::new(r"p=(\d+),(\d+) v=(\-?\d+),(\-?\d+)").unwrap();

    let mut robots = Vec::new();
    for (_, groups) in re.captures_iter(input).map(|c| c.extract()) {
        let [px, py, vx, vy] = groups.map(|g| g.parse().unwrap());
        robots.push((px, py, vx, vy))
    }

    (dim, robots)
}
