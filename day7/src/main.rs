use std::collections::VecDeque;
use std::fs;
use std::path::Path;

fn main() {
    let mut sum: usize = 0;
    for ln in parse_in(Path::new("day7.txt")) {
        sum += get_valid(ln.0, &ln.1);
    }

    println!("Sum: {sum}");
}

#[allow(clippy::cast_possible_truncation)]
fn get_valid(sol: usize, nums: &[usize]) -> usize {
    let mut nums: VecDeque<usize> = nums.to_vec().into();
    let start = nums.pop_front().expect("Nums is empty");
    let mut res = start;
    for mut i in 0..2_usize.pow(nums.len() as u32) {
        for &n in &nums {
            if i % 2 == 1 {
                res *= n;
            } else {
                res += n;
            }
            i /= 2;
        }
        if res == sol {
            return sol;
        }
        res = start;
    }

    0
}

fn parse_in(p: &Path) -> Vec<(usize, Vec<usize>)> {
    fs::read_to_string(p)
        .expect("File does not exist, fool")
        .lines()
        .map(|x| -> (&str, &str) { x.split_at(x.find(':').expect("Line malformed")) })
        .map(|x| -> (usize, Vec<usize>) {
            (
                x.0.parse().expect("Result malformed"),
                x.1[1..]
                    .trim()
                    .split(' ')
                    .map(|x| x.parse().expect("Operand malformed"))
                    .collect(),
            )
        })
        .collect()
}
