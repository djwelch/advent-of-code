use std::io::{BufRead, BufReader, Error, Read};

fn main() {
    let f = std::fs::File::open("inputs/2018/02.txt").expect("Error: file not found");
    let input = parse_input(f).expect("Error: unable to parse input");
    println!("part1: {}", part1(&input).expect("Error: "));
    println!("part2: {}", part2(&input).expect("Error: "));
}

fn parse_input<R: Read>(io: R) -> Result<Vec<String>, Error> {
    let br = BufReader::new(io);
    let mut v = vec![];
    for line in br.lines() {
        v.push(line.unwrap());
    }
    Ok(v)
}

fn part1(input: &Vec<String>) -> Result<i32, Error> {
    let mut n2 = 0;
    let mut n3 = 0;
    for line in input {
        let mut numbers = std::collections::HashMap::new();
        line.chars()
            .for_each(|n| *numbers.entry(n).or_insert(0) += 1);
        if numbers.values().any(|&n| n == 2) {
            n2 += 1;
        }
        if numbers.values().any(|&n| n == 3) {
            n3 += 1;
        }
    }
    Ok(n2 * n3)
}

fn part2(input: &Vec<String>) -> Result<String, Error> {
    let mut result: String = "".to_string();
    for line1 in input {
        for line2 in input {
            if line1 == line2 {
                continue;
            }
            let diff = line1
                .chars()
                .zip(line2.chars())
                .map(|(x, y)| if x == y { 0 } else { 1 });
            let diff_sum: i32 = diff.clone().sum();
            if diff_sum == 1 {
                result = line1.chars()
                    .zip(diff)
                    .filter(|(_, d)| { d == &0 })
                    .map(|(x,_)| { x })
                    .collect();
            }
        }
    }

    Ok(result)
}

