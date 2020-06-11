use std::collections::VecDeque;
use std::fs;

fn main() {
    let m = 10;
    let n = 10;
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let cells = contents.chars();
    let mut maze: Vec<Vec<char>> = vec![vec!['0'; n]; m];
    let mut cnt = 0;
    for cell in cells {
        if cell == '\n' {
            continue;
        }
        maze[cnt / n][cnt % n] = cell;
        cnt += 1;
    }
    println!("{}", solve(maze));
}

#[derive(Debug)]
struct Position {
    row: u8,
    col: u8,
    cnt: u32,
}

impl Position {
    fn new(row: u8, col: u8, cnt: u32) -> Position {
        return Position { row, col, cnt };
    }
}

fn solve(maze: Vec<Vec<char>>) -> u32 {
    let mut maze = maze.clone();

    let mut queue: VecDeque<Position> = VecDeque::new();
    for (i, row) in maze.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if *cell == 'S' {
                queue.push_back(Position::new(i as u8, j as u8, 1))
            }
        }
    }

    while queue.len() > 0 {
        if let Some(target_cell) = queue.pop_front() {
            let cnt = target_cell.cnt;
            if maze[target_cell.row as usize][target_cell.col as usize] == '0' {
                continue;
            } else {
                maze[target_cell.row as usize][target_cell.col as usize] = '0';
            }
            // 上
            if is_goal(&maze, target_cell.row as i16 - 1, target_cell.col as i16) {
                return cnt;
            }
            if is_aisle(&maze, target_cell.row as i16 - 1, target_cell.col as i16) {
                queue.push_back(Position::new(target_cell.row - 1, target_cell.col, cnt + 1));
            }
            // 右
            if is_goal(&maze, target_cell.row as i16, target_cell.col as i16 + 1) {
                return cnt;
            }
            if is_aisle(&maze, target_cell.row as i16, target_cell.col as i16 + 1) {
                queue.push_back(Position::new(target_cell.row, target_cell.col + 1, cnt + 1));
            }
            // 下
            if is_goal(&maze, target_cell.row as i16 + 1, target_cell.col as i16) {
                return cnt;
            }
            if is_aisle(&maze, target_cell.row as i16 + 1, target_cell.col as i16) {
                queue.push_back(Position::new(target_cell.row + 1, target_cell.col, cnt + 1));
            }
            // 左
            if is_goal(&maze, target_cell.row as i16, target_cell.col as i16 - 1) {
                return cnt;
            }
            if is_aisle(&maze, target_cell.row as i16, target_cell.col as i16 - 1) {
                queue.push_back(Position::new(target_cell.row, target_cell.col - 1, cnt + 1));
            }
        }
    }
    0
}

// TODO: 範囲外かどうかの validation が is_goal と重複しているので共通化
fn is_aisle(maze: &Vec<Vec<char>>, row: i16, col: i16) -> bool {
    if row >= maze.len() as i16 || row < 0 {
        return false;
    }
    if col >= maze[0].len() as i16 || col < 0 {
        return false;
    }
    if maze[row as usize][col as usize] == '.' {
        return true;
    } else {
        return false;
    }
}

fn is_goal(maze: &Vec<Vec<char>>, row: i16, col: i16) -> bool {
    if row >= maze.len() as i16 || row < 0 {
        return false;
    }
    if col >= maze[0].len() as i16 || col < 0 {
        return false;
    }
    if maze[row as usize][col as usize] == 'G' {
        return true;
    } else {
        return false;
    }
}
