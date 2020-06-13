use std::i32;

fn solve(s: &Vec<i32>, t: &Vec<i32>) -> i32 {
    let mut current_time = 0;
    let mut cnt = 0;
    loop {
        if let Some(current_index) = get_min_index(s, t, current_time) {
            current_time = t[current_index];
            cnt += 1;
        } else {
            return cnt;
        }
    }
}

fn get_min_index(s: &Vec<i32>, t: &Vec<i32>, current_time: i32) -> Option<usize> {
    let mut valid_indices = vec![];
    for (i, si) in s.iter().enumerate() {
        if *si > current_time {
            valid_indices.push(i);
        }
    }
    if valid_indices.len() == 0 {
        return None;
    }
    let mut min = i32::MAX;
    let mut min_index = 0;
    for i in valid_indices {
        if min > t[i] {
            min = t[i];
            min_index = i;
        }
    }
    Some(min_index)
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn it_works() {
        let s = vec![1, 2, 4, 6, 8];
        let t = vec![3, 5, 7, 9, 10];
        assert_eq!(solve(&s, &t), 3);
    }
}
