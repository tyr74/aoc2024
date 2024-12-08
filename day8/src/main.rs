use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point(usize, usize);

fn main() {
    let grid = parse_in(Path::new("day8.txt"));
    let map = find_antennas(&grid);
    let total = count_antinodes(&grid, &map);

    println!("Final count: {total}");
}

#[allow(clippy::cast_possible_wrap, clippy::needless_continue)]
fn count_antinodes(grid: &[Vec<char>], map: &HashMap<char, Vec<Point>>) -> usize {
    let mut antinodes: HashSet<Point> = HashSet::new();
    let i_max = grid.len() - 1;
    let j_max = grid[0].len() - 1;
    let mut cur_i: usize;
    let mut cur_j: usize;
    let mut i_diff: isize;
    let mut j_diff: isize;

    for points in map.values() {
        for (idx, &pt) in points.iter().enumerate() {
            for (jdx, &other) in points.iter().enumerate() {
                if idx == jdx {
                    continue;
                }
                i_diff = other.0 as isize - pt.0 as isize;
                j_diff = other.1 as isize - pt.1 as isize;
                for i in 0..200 {
                    if let Ok(a) = usize::try_from(other.0 as isize + i * i_diff) {
                        cur_i = a;
                    } else {
                        continue;
                    }
                    if let Ok(a) = usize::try_from(other.1 as isize + i * j_diff) {
                        cur_j = a;
                    } else {
                        continue;
                    }

                    if (0..=i_max).contains(&cur_i) && (0..=j_max).contains(&cur_j) {
                        antinodes.insert(Point(cur_i, cur_j));
                    }
                }
            }
        }
    }

    antinodes.len()
}

fn find_antennas(grid: &[Vec<char>]) -> HashMap<char, Vec<Point>> {
    let mut map: HashMap<char, Vec<Point>> = HashMap::new();

    for (i, ln) in grid.iter().enumerate() {
        for (j, &c) in ln.iter().enumerate() {
            if c != '.' {
                map.entry(c)
                    .and_modify(|x| x.push(Point(i, j)))
                    .or_insert_with(|| vec![Point(i, j)]);
            }
        }
    }

    map
}

fn parse_in(p: &Path) -> Vec<Vec<char>> {
    fs::read_to_string(p)
        .expect("File does not exist, fool")
        .lines()
        .map(|x| x.chars().collect())
        .collect()
}
