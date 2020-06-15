fn solve(r: i32, x: Vec<i32>) -> i32 {
    let mut valid_min = -1;
    let mut valid_max = -1;
    let mut mark_cnt = 0;
    for i in 0..x.len() {
        println!("*** i: {}, value: {} ***", i, x[i]);
        if x[i] < valid_min || valid_max < x[i] {
            let mut candidate_index = i + 1;
            loop {
                if let Some(candidate) = x.get(candidate_index) {
                    if candidate - x[i] > r {
                        break;
                    }
                } else {
                    break;
                }
                candidate_index += 1;
            }
            let max_mark_index = candidate_index - 1;
            valid_min = x[max_mark_index] - r;
            valid_max = x[max_mark_index] + r;
            mark_cnt += 1;
            println!(
                "{} is marked!, min: {}, max{}",
                x[max_mark_index], valid_min, valid_max
            );
        }
    }
    mark_cnt
}

#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn it_works() {
        let r = 10;
        let x = vec![1, 7, 15, 20, 30, 50];
        assert_eq!(solve(r, x), 3);
    }
}
