use aoc::Regex;

fn main() {
    aoc::run_parts(one, two);
}

fn one(input: &str) -> u64 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    re.captures_iter(input)
        .map(|c| c.extract().1.map(|s| s.parse::<u64>().unwrap()))
        .map(|[x, y]| x * y)
        .sum()
}

fn two(input: &str) -> u64 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();
    re.captures_iter(input)
        .fold((0, true), |(sum, enabled), c| match &c[0] {
            "do()" => (sum, true),
            "don't()" => (sum, false),
            _ if enabled => {
                let [x, y] = [&c[1], &c[2]].map(|s| s.parse::<u64>().unwrap());
                (sum + x * y, true)
            }
            _ => (sum, false),
        })
        .0
}
