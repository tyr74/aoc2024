use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Copy)]
struct Point(usize, usize);

#[derive(Debug, PartialEq, Clone, Copy)]
enum Dir {
    N,
    S,
    E,
    W,
    NE,
    SE,
    NW,
    SW,
    All,
}

impl Dir {
    fn i(self, pt: &Point) -> usize {
        match self {
            Self::N | Self::NE | Self::NW => pt.0.wrapping_sub(1),
            Self::S | Self::SE | Self::SW => pt.0 + 1,
            Self::All => panic!("Don't use this func with all"),
            _ => pt.0,
        }
    }

    fn j(self, pt: &Point) -> usize {
        match self {
            Self::W | Self::SW | Self::NW => pt.1 + 1,
            Self::E | Self::SE | Self::NE => pt.1.wrapping_sub(1),
            Self::All => panic!("Don't use this func with all"),
            _ => pt.1,
        }
    }
}

#[derive(Debug)]
struct DirPoint {
    pt: Point,
    dir: Dir,
}

impl DirPoint {
    fn i(&self) -> usize {
        self.dir.i(&self.pt)
    }
    fn j(&self) -> usize {
        self.dir.j(&self.pt)
    }
}

fn main() {
    let sum = do_thing(Path::new("day4.txt"));

    println!("Sum: {sum}");
}

fn do_thing(p: &Path) -> usize {
    let grid = parse_in(p);
    let starts = find_a(&grid);
    let mut sum: usize = 0;

    for s in starts {
        sum += process_x(&grid, &s);
    }

    sum
}

fn process_x(grid: &[Vec<char>], pt: &Point) -> usize {
    let mut num_m: usize = 0;
    let mut num_s: usize = 0;
    let dirs = [Dir::NE, Dir::NW, Dir::SE, Dir::SW];

    for d in dirs {
        if let Some(ln) = grid.get(d.i(pt)) {
            if let Some(&c) = ln.get(d.j(pt)) {
                if c == 'M' {
                    num_m += 1;
                } else if c == 'S' {
                    num_s += 1;
                } else {
                    return 0;
                }
            } else {
                return 0;
            }
        } else {
            return 0;
        }
    }

    if num_m == 2
        && num_s == 2
        && (grid[Dir::NE.i(pt)][Dir::NE.j(pt)] != grid[Dir::SW.i(pt)][Dir::SW.j(pt)]
            && grid[Dir::SE.i(pt)][Dir::SE.j(pt)] != grid[Dir::NW.i(pt)][Dir::NW.j(pt)])
    {
        return 1;
    }
    0
}

fn process(grid: &[Vec<char>], dpt: &DirPoint, letter: char) -> usize {
    let mut output: usize = 0;
    if dpt.dir == Dir::All {
        let dirs = [
            Dir::N,
            Dir::NE,
            Dir::E,
            Dir::SE,
            Dir::S,
            Dir::SW,
            Dir::W,
            Dir::NW,
        ];
        for d in dirs {
            output += process(grid, &DirPoint { pt: dpt.pt, dir: d }, 'M');
        }
        return output;
    }
    if let Some(ln) = grid.get(dpt.i()) {
        if let Some(&c) = ln.get(dpt.j()) {
            if c == letter {
                if c == 'S' {
                    return 1;
                }
                return process(
                    grid,
                    &DirPoint {
                        pt: Point(dpt.i(), dpt.j()),
                        dir: dpt.dir,
                    },
                    next_letter(letter),
                );
            }
        }
    }

    0
}

fn next_letter(letter: char) -> char {
    match letter {
        'X' => 'M',
        'M' => 'A',
        'A' => 'S',
        _ => panic!("Letter should't be on board"),
    }
}

fn find_a(grid: &[Vec<char>]) -> Vec<Point> {
    let mut output: Vec<Point> = Vec::new();

    for (i, ln) in grid.iter().enumerate() {
        for (j, &c) in ln.iter().enumerate() {
            if c == 'A' {
                output.push(Point(i, j));
            }
        }
    }

    output
}

fn find_starts(grid: &[Vec<char>]) -> Vec<DirPoint> {
    let mut output: Vec<DirPoint> = Vec::new();

    for (i, ln) in grid.iter().enumerate() {
        for (j, &c) in ln.iter().enumerate() {
            if c == 'X' {
                output.push(DirPoint {
                    pt: Point(i, j),
                    dir: Dir::All,
                });
            }
        }
    }

    output
}

fn parse_in(p: &Path) -> Vec<Vec<char>> {
    fs::read_to_string(p)
        .expect("File does not exist, fool")
        .lines()
        .map(|x| x.chars().collect())
        .collect()
}
