use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug)]
struct PageOrdering {
    num: usize,
    before: Vec<usize>,
}

impl PartialEq for PageOrdering {
    fn eq(&self, other: &Self) -> bool {
        self.num == other.num
    }
}

impl Eq for PageOrdering {}

impl PartialOrd for PageOrdering {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PageOrdering {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.before.contains(&other.num) {
            Ordering::Greater
        } else if other.before.contains(&self.num) {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
}

fn main() {
    let (map, lists) = parse_in(Path::new("day5.txt"));
    let mut sum: usize = 0;

    for mut l in lists {
        sum += med_of_sorted(&map, &mut l);
    }

    println!("Sum: {sum}");
}

fn med_of_sorted(map: &HashMap<usize, PageOrdering>, list: &mut [usize]) -> usize {
    if list.is_sorted_by(|x, y| {
        map.get(x)
            .expect("Number is not in list")
            .lt(map.get(y).expect("Number is not in list"))
    }) {
        0
    } else {
        list.sort_by(|x, y| {
            map.get(x)
                .expect("Number is not in list")
                .cmp(map.get(y).expect("Number is not in list"))
        });
        list[list.len() / 2]
    }
}

fn parse_in(p: &Path) -> (HashMap<usize, PageOrdering>, Vec<Vec<usize>>) {
    let file = fs::read_to_string(p).expect("File does not exist, fool");
    let mut rules: bool = true;
    let mut map: HashMap<usize, PageOrdering> = HashMap::new();
    let mut lists: Vec<Vec<usize>> = Vec::new();

    for ln in file.lines() {
        if ln.is_empty() {
            rules = false;
            continue;
        }
        if rules {
            let split: Vec<usize> = ln
                .split('|')
                .map(|x| x.parse().expect("Num not parsed correctly"))
                .collect();
            map.entry(split[0]).or_insert(PageOrdering {
                num: split[0],
                before: Vec::new(),
            });
            map.entry(split[1])
                .and_modify(|x| x.before.push(split[0]))
                .or_insert_with(|| PageOrdering {
                    num: split[1],
                    before: vec![split[0]],
                });
        } else {
            lists.push(
                ln.split(',')
                    .map(|x| x.parse().expect("Number not parsed correctly"))
                    .collect(),
            );
        }
    }

    (map, lists)
}
