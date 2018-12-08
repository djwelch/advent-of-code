extern crate regex;
extern crate itertools;
use std::io::{BufRead, BufReader, Error, Read};
use regex::Regex;
use itertools::Itertools;

fn main() {
    let f = std::fs::File::open("inputs/2018/04.txt").expect("Error: file not found");
    let input = parse_input(f).expect("Error: unable to parse input");
    println!("part1: {}", part1(&input).expect("Error: "));
    println!("part2: {}", part2(&input).expect("Error: "));
}

fn parse_input<R: Read>(io: R) -> Result<Vec<(i32,i32,i32)>, Error> {
    let reGuard = Regex::new(r"\[\d{4}-\d{2}-\d{2} (\d{2}):(\d{2})\] Guard #(\d+) begins shift$").unwrap();
    let reSleep = Regex::new(r"\[\d{4}-\d{2}-\d{2} (\d{2}):(\d{2})\] falls asleep$").unwrap();
    let reWake = Regex::new(r"\[\d{4}-\d{2}-\d{2} (\d{2}):(\d{2})\] wakes up$").unwrap();
    let br = BufReader::new(io);
    let mut lines:Vec<_> = br.lines().map(|l| l.unwrap()).collect();
    lines.sort();
    let mut v = vec![];
    let mut guard:Option<i32> = None;
    let mut sleep:Option<i32> = None;
    let mut wake:Option<i32> = None;
    for l in lines {
        if let Some(cap) = reGuard.captures(&l) {
            guard = Some(cap.get(3).unwrap().as_str().parse().unwrap());
            sleep = None;
            wake = None;
        }
        if let Some(cap) = reSleep.captures(&l) {
            sleep = Some(cap.get(2).unwrap().as_str().parse().unwrap());
            wake = None;
        }
        if let Some(cap) = reWake.captures(&l) {
            wake = Some(cap.get(2).unwrap().as_str().parse().unwrap());
        }
        // let n = caps.get(1).unwrap().as_str().parse().unwrap();
        if let (Some(guard), Some(sleep), Some(wake)) = (guard, sleep, wake) {
            v.push((guard, sleep, wake));
        }
    }
    Ok(v)
}

fn part1(input: &Vec<(i32,i32,i32)>) -> Result<i32, Error> {
    let mut guards = vec![];
    for (guard, times) in &input.iter().group_by(|(guard, _, _)| guard) {
        guards.push((guard, times.map(|(_, s, e)| e-s).sum::<i32>()));
    }
    guards.sort_by(|(_, sleep1), (_, sleep2)| sleep2.cmp(sleep1));

    let (sleepyGuard, sleepyMinutes) = guards[0];
    let mut minutes = std::collections::HashMap::new();

    for (_, s, w) in input.iter().filter(|(guard, _, _)| guard == sleepyGuard) {
        for i in *s..*w {
            *minutes.entry(i).or_insert(0) += 1;
        }
    }
    
    let mut sorted: Vec<_> = minutes.iter().collect();
    sorted.sort_by(|(_, m1), (_, m2)| m2.cmp(m1));
    let m = *sorted[0].0;
    Ok(m*sleepyGuard)
}

fn part2(input: &Vec<(i32,i32,i32)>) -> Result<i32, Error> {
    let mut minutes = std::collections::HashMap::new();
    for (guard, times) in &input.iter().group_by(|(guard, _, _)| guard) {
        for (_, s, w) in times {
            for i in *s..*w {
                *minutes.entry((guard, i)).or_insert(0) += 1;
            }
        }
    }
    let mut sorted: Vec<_> = minutes.into_iter().collect();
    sorted.sort_by(|(_, m1), (_, m2)| m2.cmp(m1));
    // println!("{:?}", sorted);
    let ((guard, m), s) = sorted[0];
    Ok(guard*m)
}

