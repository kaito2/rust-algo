use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut garden: Vec<Vec<char>> = vec![];

    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(line_str) = line {
                garden.push(parse_line(&line_str))
            }
        }
    };
    let answer = solve(&garden);
    println!("{}", answer)
}

fn parse_line(line: &str) -> Vec<char> {
    line.chars().collect()
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn solve(garden: &Vec<Vec<char>>) -> u32 {
    let mut garden2 = garden.clone();

    // TODO: validate garden shape
    let row_num = garden.len();
    let col_num = garden[0].len();
    println!("row: {}, col: {}", row_num, col_num);

    let mut count = 0;
    for i in 0..row_num {
        for j in 0..col_num {
            if garden2[i][j] == 'w' {
                count += 1;
                let i_i32 = i as i32;
                let j_i32 = j as i32;
                mark_as_counted_iter(&mut garden2, i_i32, j_i32);
            }
        }
    }
    count
}

fn mark_as_counted_iter(garden: &mut Vec<Vec<char>>, row: i32, col: i32) {
    // TODO: validate row & col
    if row >= garden.len() as i32 || row < 0 {
        return;
    }
    if col >= garden[0].len() as i32 || col < 0 {
        return;
    }
    let row_usize = row as usize;
    let col_usize = col as usize;

    if garden[row_usize][col_usize] == 'w' {
        garden[row_usize][col_usize] = '.';
        // 左上から時計回りに
        mark_as_counted_iter(garden, row - 1, col - 1);
        mark_as_counted_iter(garden, row - 1, col);
        mark_as_counted_iter(garden, row - 1, col + 1);
        mark_as_counted_iter(garden, row, col + 1);
        mark_as_counted_iter(garden, row + 1, col + 1);
        mark_as_counted_iter(garden, row + 1, col);
        mark_as_counted_iter(garden, row + 1, col - 1);
        mark_as_counted_iter(garden, row, col - 1);
    } else {
        return;
    }
}
