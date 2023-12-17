use nalgebra::DMatrix;
use std::{fs::File, io::{BufReader, BufRead}, collections::{HashSet, HashMap}, cmp::max};

const MIRROR_RIGHT: char = '/';
const MIRROR_LEFT: char = '\\';
const SPLITTER_VERTICAL: char = '|';
const SPLITTER_HORIZONTAL: char = '-';

type Visits = HashMap<(i32, i32), Vec<Direction>>;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn travel_forward(&self, location: &mut (i32, i32)) {
        match *self {
            Direction::Up => location.0 -= 1,
            Direction::Down => location.0 += 1,
            Direction::Left => location.1 -= 1,
            Direction::Right => location.1 += 1,
        }
    }
}

fn build_matrix(filename: &str) -> DMatrix<char> {
    let file = File::open(filename).unwrap();
    let mut data = Vec::new();
    let mut nrows = 0;
    for line in BufReader::new(file).lines() {
        let line = line.unwrap();
        data.extend(line.chars());
        nrows += 1;
    }
    DMatrix::from_row_slice(nrows, data.len() / nrows, &data)
}

fn traverse(matrix: &DMatrix<char>, start: (i32, i32), direction: Direction, visited: Option<&mut Visits>) -> HashSet<(i32, i32)> {
    let mut direction = direction;
    let mut set = HashSet::new();
    let mut binding = HashMap::new();
    let mut visited: &mut Visits = visited.unwrap_or(&mut binding);
    let mut current = start;
    let nrows = matrix.nrows() as i32;
    let ncols = matrix.ncols() as i32;
    while current.0 < nrows && current.0 > -1 && current.1 < ncols && current.1 > -1 {
        if visited.contains_key(&current) && visited[&current].contains(&direction) {
            break;
        }
        visited.entry(current).or_insert(Vec::new()).push(direction.clone());
        set.insert(current);
        match matrix[(current.0 as usize, current.1 as usize)] {
            MIRROR_RIGHT => {
                match direction {
                    Direction::Up => { current.1 += 1; direction = Direction::Right; },
                    Direction::Down => { current.1 -= 1; direction = Direction::Left; },
                    Direction::Left => { current.0 += 1; direction = Direction::Down; },
                    Direction::Right => { current.0 -= 1; direction = Direction::Up; },
                }
            }
            MIRROR_LEFT => {
                match direction {
                    Direction::Up => { current.1 -= 1; direction = Direction::Left; },
                    Direction::Down => { current.1 += 1; direction = Direction::Right; },
                    Direction::Left => { current.0 -= 1; direction = Direction::Up; },
                    Direction::Right => { current.0 += 1; direction = Direction::Down; },
                }
            }
            SPLITTER_VERTICAL => {
                if direction == Direction::Up || direction == Direction::Down {
                    direction.travel_forward(&mut current);
                } else {
                    set.extend(traverse(matrix, (current.0 - 1, current.1), Direction::Up, Some(&mut visited)));
                    set.extend(traverse(matrix, (current.0 + 1, current.1), Direction::Down, Some(&mut visited)));
                    break;
                }
            }
            SPLITTER_HORIZONTAL => {
                if direction == Direction::Right || direction == Direction::Left {
                    direction.travel_forward(&mut current);
                } else {
                    set.extend(traverse(matrix, (current.0, current.1 - 1), Direction::Left, Some(&mut visited)));
                    set.extend(traverse(matrix, (current.0, current.1 + 1), Direction::Right, Some(&mut visited)));
                    break;
                }
            }
            _ => {
                direction.travel_forward(&mut current);
            }
        }
    }
    set
}

fn solution(filename: &str) -> usize {
    let matrix = build_matrix(filename);
    traverse(&matrix, (0, 0), Direction::Right, None).len()
}

fn solution_2(filename: &str) -> usize { 
    let matrix = build_matrix(filename);
    let nrows = matrix.nrows() as i32;
    let ncols = matrix.ncols() as i32;
    let mut largest = 0;
    for i in 0..nrows {
        largest = max(traverse(&matrix, (i, 0), Direction::Right, None).len(), largest);
        largest = max(traverse(&matrix, (i, ncols - 1), Direction::Left, None).len(), largest);
    }
    for i in 0..ncols {
        largest = max(traverse(&matrix, (0, i), Direction::Down, None).len(), largest);
        largest = max(traverse(&matrix, (nrows - 1, i), Direction::Up, None).len(), largest);
    }
    largest
}

fn main() {
    assert_eq!(solution("example.txt"), 46);
    assert_eq!(solution("input.txt"), 6921);
    assert_eq!(solution_2("example.txt"), 51);
    assert_eq!(solution_2("input.txt"), 7594);
}
