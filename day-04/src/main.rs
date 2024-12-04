use aoc::ArrayVec;

fn main() {
    aoc::run_parts(one, two);
}

fn one(input: &str) -> u64 {
    let lines = input
        .lines()
        .map(|line| line.as_bytes())
        .collect::<Vec<_>>();
    let width = lines[0].len();
    let height = lines.len();

    let mut count = 0;

    for line in &lines {
        count += line
            .windows(4)
            .filter(|window| window == b"XMAS" || window == b"SAMX")
            .count() as u64;
    }

    for i in 0..width {
        count += lines
            .windows(4)
            .filter(|window| {
                let vert = window
                    .iter()
                    .map(|line| line[i])
                    .collect::<ArrayVec<_, 4>>();
                let vert = vert.as_slice();
                vert == b"XMAS" || vert == b"SAMX"
            })
            .count() as u64;
    }

    for i in 0..height - 3 {
        for j in 0..width - 3 {
            let mut forward = [0; 4];
            let mut back = [0; 4];

            for k in 0..4 {
                forward[k] = lines[i + k][j + k];
                back[k] = lines[i + k][j + 3 - k];
            }

            count += [forward, back]
                .into_iter()
                .filter(|diag| diag == b"XMAS" || diag == b"SAMX")
                .count() as u64;
        }
    }

    count
}

fn two(input: &str) -> u64 {
    let lines = input
        .lines()
        .map(|line| line.as_bytes())
        .collect::<Vec<_>>();

    let mut count = 0;

    for vert in lines.windows(3) {
        let mut vert = vert
            .iter()
            .map(|line| line.windows(3))
            .collect::<ArrayVec<_, 3>>();

        while let Some(square) = vert
            .iter_mut()
            .map(|line| line.next())
            .collect::<Option<ArrayVec<_, 3>>>()
        {
            let square = square.as_slice();
            if let [[b'M', _, b'M'], [_, b'A', _], [b'S', _, b'S']]
            | [[b'M', _, b'S'], [_, b'A', _], [b'M', _, b'S']]
            | [[b'S', _, b'S'], [_, b'A', _], [b'M', _, b'M']]
            | [[b'S', _, b'M'], [_, b'A', _], [b'S', _, b'M']] = square
            {
                count += 1;
            }
        }
    }

    count
}
