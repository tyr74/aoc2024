use std::fs;
use std::path::Path;

fn main() {
    let sum = do_thing(Path::new("day3.txt"));

    println!("Sum: {sum}");
}

fn do_thing(p: &Path) -> usize {
    let mut sum: usize = 0;
    let cmds = parse_in(p);
    let mut enabled: bool = true;
    for s in cmds {
        sum += calc_mul(&s, &mut enabled);
    }

    sum
}

fn calc_mul(s: &str, enabled: &mut bool) -> usize {
    let mut sum: usize = 0;

    'i: for (idx, c) in s.chars().enumerate() {
        if c == 'd' {
            if s[idx..idx + 4] == *"do()" {
                *enabled = true;
            }
            if s[idx..idx + 7] == *"don't()" {
                *enabled = false;
            }
        }
        if *enabled && c == 'm' {
            if s[idx..idx + 4] != *"mul(" {
                continue;
            }
            let mut prod: usize = 1;
            let mut cur: String = String::new();
            let mut comma: bool = false;
            let mut second: bool = false;
            for c in s[idx + 4..].chars() {
                if c.is_numeric() {
                    cur.push(c);
                    if comma {
                        second = true;
                    }
                } else if c == ',' {
                    prod *= cur.parse().unwrap_or(0);
                    if prod == 0 {
                        continue 'i;
                    }
                    comma = true;
                    cur = String::new();
                } else if c == ')' {
                    if !second {
                        continue 'i;
                    }
                    prod *= cur.parse().unwrap_or(0);
                    if prod == 0 {
                        continue 'i;
                    }
                    break;
                } else {
                    continue 'i;
                }
            }
            sum += prod;
        }
    }

    sum
}

fn parse_in(p: &Path) -> Vec<String> {
    fs::read_to_string(p)
        .expect("File does not exist, fool")
        .lines()
        .map(String::from)
        .collect()
}
