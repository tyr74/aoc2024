use std::collections::HashSet;
use std::fs;
use std::path::Path;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point(usize, usize);

impl Point {
    fn next_step(&mut self, dir: Dir) {
        match dir {
            Dir::North => self.0 = self.0.wrapping_sub(1),
            Dir::East => self.1 += 1,
            Dir::South => self.0 += 1,
            Dir::West => self.1 = self.1.wrapping_sub(1),
        }
    }

    fn move_back(&mut self, dir: Dir) {
        match dir {
            Dir::North => self.0 += 1,
            Dir::East => self.1 = self.1.wrapping_sub(1),
            Dir::South => self.0 = self.0.wrapping_sub(1),
            Dir::West => self.1 += 1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    North,
    East,
    South,
    West,
}

impl Dir {
    const fn turn_90(self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }
}

fn main() {
    let mut grid = parse_in(Path::new("day6.txt"));
    let total = find_loops(&mut grid);

    println!("Total: {total}");
}

#[allow(clippy::match_on_vec_items)]
fn find_loops(grid: &mut [Vec<char>]) -> usize {
    let start: Point = find_start(grid);
    let mut cur: Point = start;
    let mut set: HashSet<(Point, Dir)> = HashSet::new();
    let mut cur_dir: Dir = Dir::North;
    let mut count: usize = 0;

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            match grid[i][j] {
                '.' => grid[i][j] = 'O',
                _ => continue,
            }
            while let Some(ln) = grid.get(cur.0) {
                if let Some(&c) = ln.get(cur.1) {
                    match c {
                        '.' | '^' => {
                            cur.next_step(cur_dir);
                        }
                        '#' | 'O' => {
                            if !set.insert((cur, cur_dir)) {
                                count += 1;
                                break;
                            }
                            cur.move_back(cur_dir);
                            cur_dir = cur_dir.turn_90();
                            cur.next_step(cur_dir);
                        }
                        _ => panic!("Wrong character in grid"),
                    }
                } else {
                    break;
                }
            }
            grid[i][j] = '.';
            cur = start;
            cur_dir = Dir::North;
            set = HashSet::new();
        }
    }

    count
}

fn count_path(grid: &[Vec<char>]) -> usize {
    let mut cur: Point = find_start(grid);
    let mut set: HashSet<Point> = HashSet::new();
    let mut cur_dir: Dir = Dir::North;

    while let Some(ln) = grid.get(cur.0) {
        if let Some(&c) = ln.get(cur.1) {
            match c {
                '.' | '^' => {
                    let _ = set.insert(cur);
                    cur.next_step(cur_dir);
                }
                '#' => {
                    cur.move_back(cur_dir);
                    cur_dir = cur_dir.turn_90();
                    cur.next_step(cur_dir);
                }
                _ => panic!("Wrong character in grid"),
            }
        } else {
            break;
        }
    }

    set.len()
}

fn find_start(grid: &[Vec<char>]) -> Point {
    for (i, ln) in grid.iter().enumerate() {
        for (j, &c) in ln.iter().enumerate() {
            if c == '^' {
                return Point(i, j);
            }
        }
    }

    panic!("Couldn't find starting position");
}

fn parse_in(p: &Path) -> Vec<Vec<char>> {
    fs::read_to_string(p)
        .expect("File does not exist, fool")
        .lines()
        .map(|x| x.chars().collect())
        .collect()
}
