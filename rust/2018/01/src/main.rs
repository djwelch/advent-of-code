use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

fn main() {
    let f = std::fs::File::open("inputs/2018/01.txt").expect("Error: file not found");
    let input = parse_input(f).expect("Error: unable to parse input");
    println!("part1: {}", part1(&input).expect("Error: "));
    println!("part2: {}", part2(&input).expect("Error: "));

}

fn parse_input<R: Read>(io: R) -> Result<Vec<i32>, Error> {
    let br = BufReader::new(io);
    let mut v = vec![];
    for line in br.lines() {
        v.push(line.unwrap().parse::<i32>().map_err(|e| Error::new(ErrorKind::InvalidData, e))?);
    }
    Ok(v)
}

fn part1(input: &Vec<i32>) -> Result<i32, Error> {
    Ok(input.iter().fold(0, |sum, &n| sum+n))
}

fn part2(input: &Vec<i32>) -> Result<i32, Error> {
    let mut iter = input.iter().cycle();
    let mut numbers = std::collections::HashSet::new();
    let mut sum = 0;
    let value = loop {
        let n = iter.next().unwrap();
        sum += n;
        if numbers.contains(&sum) {
            break sum;
        }
        numbers.insert(sum);
    };

    return Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_it() {
        assert_eq!(3, 2)
    }
}
