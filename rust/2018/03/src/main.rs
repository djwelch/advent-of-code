extern crate regex;
use std::io::{BufRead, BufReader, Error, Read};
use regex::Regex;

fn main() {
    let f = std::fs::File::open("inputs/2018/03.txt").expect("Error: file not found");
    let input = parse_input(f).expect("Error: unable to parse input");
    println!("part1: {}", part1(&input).expect("Error: "));
    println!("part2: {}", part2(&input).expect("Error: "));
}

fn parse_input<R: Read>(io: R) -> Result<Vec<(i32,i32,i32,i32,i32)>, Error> {
    let re = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();
    let br = BufReader::new(io);
    let mut v = vec![];
    for line in br.lines() {
        let l = line.unwrap();
        let caps = re.captures(&l).unwrap();
        let n = caps.get(1).unwrap().as_str().parse().unwrap();
        let left = caps.get(2).unwrap().as_str().parse().unwrap();
        let top = caps.get(3).unwrap().as_str().parse().unwrap();
        let width = caps.get(4).unwrap().as_str().parse().unwrap();
        let height = caps.get(5).unwrap().as_str().parse().unwrap();
        v.push((left, top, width, height, n));
    }
    Ok(v)
}

fn part1(input: &Vec<(i32,i32,i32,i32,i32)>) -> Result<i32, Error> {
    let mut claims = std::collections::HashMap::new();

    for &(left, top, width, height, _) in input.iter() {
        for x in left..(left+width) {
            for y in top..(top+height) {
                *claims.entry((x,y)).or_insert(0) += 1;
            }
        }
    };

    let a = claims.values().filter(|&v| v > &1).count() as i32;
    Ok(a)
}

fn part2(input: &Vec<(i32,i32,i32,i32,i32)>) -> Result<i32, Error> {
    let mut claims = std::collections::HashMap::new();
    let mut overlapping = std::collections::HashSet::new();
    let mut nonoverlapping = std::collections::HashSet::new();

    for &(left, top, width, height, id) in input.iter() {
        for x in left..(left+width) {
            for y in top..(top+height) {
                let previd = *claims.entry((x,y)).or_insert(id);
                if previd != id {
                    overlapping.insert(previd);
                    overlapping.insert(id);
                } else {
                    nonoverlapping.insert(id);
                }
            }
        }
    };

    let i = nonoverlapping.difference(&overlapping).next().unwrap();
    Ok(*i)
}

