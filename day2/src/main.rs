use std::fs;
use std::path::Path;

fn main() {
    let count = count_valid(Path::new("day2.txt"));

    println!("Count: {count}");
}

fn count_valid(p: &Path) -> usize {
    let reports = parse_file(p);
    let mut count: usize = 0;

    for rep in reports {
        if check_report(&rep) || check_damped(&rep) {
            count += 1;
        }
    }

    count
}

fn check_damped(report: &[isize]) -> bool {
    let mut diffs: Vec<isize> = Vec::new();

    for skip in 0..report.len() {
        diffs.clear();
        let mut temp: Vec<isize> = report.to_vec();
        let _ = temp.remove(skip);
        for i in 0..(temp.len() - 1) {
            diffs.push(temp[i + 1] - temp[i]);
        }

        if (diffs.iter().all(|&x| x < 0) && diffs.iter().all(|&x| x >= -3))
            || (diffs.iter().all(|&x| x > 0) && diffs.iter().all(|&x| x <= 3))
        {
            return true;
        }
    }
    false
}

fn check_report(report: &[isize]) -> bool {
    let mut diffs: Vec<isize> = Vec::new();

    for i in 0..(report.len() - 1) {
        diffs.push(report[i + 1] - report[i]);
    }

    if diffs.iter().any(|&x| x == 0) {
        false
    } else if diffs.iter().all(|&x| x < 0) {
        diffs.iter().all(|&x| x >= -3)
    } else if diffs.iter().all(|&x| x > 0) {
        diffs.iter().all(|&x| x <= 3)
    } else {
        false
    }
}

fn parse_file(p: &Path) -> Vec<Vec<isize>> {
    let file = fs::read_to_string(p).expect("File does not exist, fool");
    let mut out: Vec<Vec<isize>> = Vec::new();

    for ln in file.lines() {
        out.push(
            ln.split(' ')
                .map(|x| x.parse().expect("Number parsed incorrectly"))
                .collect(),
        );
    }

    out
}
