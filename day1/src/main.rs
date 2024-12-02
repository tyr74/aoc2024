use std::fs;
use std::iter::{zip, Zip};
use std::path::Path;

fn main() {
    let mut sum = 0usize;
    let mut count = 0usize;
    let (l, r) = parse_input_both(Path::new("day1.txt"));

    for (i, j) in parse_input(Path::new("day1.txt")) {
        sum += i.abs_diff(j);
    }

    for i in l {
        count += i * r
            .iter()
            .fold(0, |acc, x| if *x == i { acc + 1 } else { acc });
    }

    println!("Sum: {sum}");
    println!("Count: {count}");
}

fn parse_input_both(p: &Path) -> (Vec<usize>, Vec<usize>) {
    let file = fs::read_to_string(p).expect("File does not exist, fool");
    let mut l: Vec<usize> = Vec::new();
    let mut r: Vec<usize> = Vec::new();

    for ln in file.lines() {
        let mut split = ln
            .split("   ")
            .map(str::parse::<usize>)
            .map(|x| x.expect("Number not parsed correctly"));
        l.push(split.next().expect("Line empty"));
        r.push(split.next().expect("Line only has one item"));
    }

    (l, r)
}

fn parse_input(p: &Path) -> Zip<std::vec::IntoIter<usize>, std::vec::IntoIter<usize>> {
    let file = fs::read_to_string(p).expect("File does not exist, fool");
    let mut l: Vec<usize> = Vec::new();
    let mut r: Vec<usize> = Vec::new();

    for ln in file.lines() {
        let mut split = ln
            .split("   ")
            .map(str::parse::<usize>)
            .map(|x| x.expect("Number not parsed correctly"));
        l.push(split.next().expect("Line empty"));
        r.push(split.next().expect("Line only has one item"));
    }
    l.sort_unstable();
    r.sort_unstable();

    zip(l, r)
}
