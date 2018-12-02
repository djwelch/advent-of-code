extern crate reqwest;
use std::env;
use std::fs;
use std::io;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let day = args[1].parse::<i32>().unwrap();
    let url = format!("https://adventofcode.com/2018/day/{}/input", day);
    let session = env::var("AOC_SESSION").expect("Error: AOC_SESSION");

    let mut response = reqwest::Client::new()
        .get(&url)
        .header(reqwest::header::COOKIE, format!("session={}", session))
        .send()
        .expect("Error: could not send request");

    match response.status() {
        reqwest::StatusCode::OK => {
            let file_path = format!("inputs/2018/{:02}.txt", day);
            let path = Path::new(&file_path);
            let dir = path.parent().unwrap();
            fs::create_dir_all(dir).expect(&format!(
                    "Error: could not create directory: {}",
                    dir.display()
                    ));

            let mut file = fs::File::create(path)
                .expect(&format!("Error: could not create file: {}", path.display()));

            io::copy(&mut response, &mut file).expect(&format!(
                    "Error: could not copy to file: {}",
                    path.display()
                    ));
        }
        sc => println!("Error: {}", sc),
    };
    Ok(())
}

