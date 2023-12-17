use nalgebra::DMatrix;
use std::{fs::File, io::{BufReader, BufRead}, collections::HashSet};

const MIRROR_RIGHT: char = '/';
const MIRROR_LEFT: char = '\\';
const SPLITTER_VERTICAL: char = '|';
const SPLITTER_HORIZONTAL: char = '-';

#[derive(Debug, PartialEq, Eq)]
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

fn traverse(matrix: &DMatrix<char>, start: (i32, i32), direction: Direction) -> HashSet<(i32, i32)> {
    let mut direction = direction;
    let mut set = HashSet::new();
    let mut current = start;
    let nrows = matrix.nrows() as i32;
    let ncols = matrix.ncols() as i32;
    while current.0 < nrows && current.0 > -1 && current.1 < ncols && current.1 > -1 {
        println!("{:?}", current);
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
                    set.extend(traverse(matrix, (current.0 - 1, current.1), Direction::Up));
                    set.extend(traverse(matrix, (current.0 + 1, current.1), Direction::Down));
                    break;
                }
            }
            SPLITTER_HORIZONTAL => {
                if direction == Direction::Right || direction == Direction::Left {
                    direction.travel_forward(&mut current);
                } else {
                    set.extend(traverse(matrix, (current.0, current.1 - 1), Direction::Left));
                    set.extend(traverse(matrix, (current.0, current.1 + 1), Direction::Right));
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

// fn display_energized(set: &HashSet<(i32, i32)>, nrows: usize, ncols: usize) {
//     let mut matrix = DMatrix::from_element(nrows, ncols, '.');
//     for (i, j) in set {
//         matrix[(*i as usize, *j as usize)] = '#';
//     }
//     println!("{}", matrix);
// }

fn solution(filename: &str) -> usize {
    let matrix = build_matrix(filename);
    traverse(&matrix, (0, 0), Direction::Right).len()
}

fn main() {
    assert_eq!(solution("example.txt"), 46);
    // assert_eq!(solution("input.txt"), 0);
}
