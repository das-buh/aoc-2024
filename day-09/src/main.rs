use std::iter;

fn main() {
    aoc::run_parts(one, two);
}

fn one(input: &str) -> u64 {
    let mut input = parse_input(input);

    let mut blocks = Vec::new();
    let mut id = 0;

    while let Some(file_len) = input.next() {
        let free_len = input.next().unwrap_or(0);
        blocks.extend(iter::repeat_n(Some(id), file_len));
        blocks.extend(iter::repeat_n(None, free_len));
        id += 1;
    }

    let mut checksum = 0;
    let mut pos = 0;

    while pos < blocks.len() {
        let id = blocks[pos].unwrap_or_else(|| loop {
            match blocks.pop().unwrap() {
                Some(id) => break id,
                None => continue,
            }
        });
        checksum += id * pos as u64;
        pos += 1;
    }

    checksum
}

fn two(input: &str) -> u64 {
    let mut input = parse_input(input);

    let mut blocks = Vec::new();
    let mut files = Vec::new();
    let mut pos = 0;
    let mut id = 0;

    while let Some(file_len) = input.next() {
        let free_len = input.next().unwrap_or(0);

        blocks.extend(iter::repeat_n(Some(id), file_len));
        blocks.extend(iter::repeat_n(None, free_len));
        files.push((id, pos, file_len));

        pos += file_len + free_len;
        id += 1;
    }

    'outer: while let Some((file_id, file_pos, file_len)) = files.pop() {
        let mut free_pos = 0;
        let mut free_len = 0;

        let free_pos = loop {
            match blocks[free_pos] {
                None => free_len += 1,
                Some(id) if id == file_id => continue 'outer,
                Some(_) => free_len = 0,
            }

            if free_len == file_len {
                break free_pos - free_len + 1;
            }

            free_pos += 1;
        };

        for i in 0..file_len {
            blocks[free_pos + i] = blocks[file_pos + i];
            blocks[file_pos + i] = None;
        }
    }

    let mut checksum = 0;

    for (pos, block) in blocks.into_iter().enumerate() {
        if let Some(id) = block {
            checksum += id * pos as u64;
        }
    }

    checksum
}

fn parse_input(input: &str) -> impl Iterator<Item = usize> + '_ {
    input
        .chars()
        .flat_map(|len| len.to_digit(10))
        .map(|len| len as usize)
}
