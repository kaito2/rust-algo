use std::cmp;

fn main() {
    println!("Hello, world!");
}

fn solve(a: &Vec<i32>, m: i32, M: i32) -> i32 {
    let mut dp = vec![vec![0; (m + 1) as usize]; a.len() + 1];
    for i in 0..=a.len() {
        dp[i][0] = 1;
    }
    for i in 0..a.len() {
        for j in 1..=m as usize {
            // dp[i + 1][j] =
            for k in 0..=cmp::min(j, a[i] as usize) {
                dp[i + 1][j] += dp[i][j - k];
            }
        }
    }
    println!("{:?}", dp);
    dp[a.len()][m as usize]
}

#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn it_works() {
        let n = 3;
        let m = 3;
        let a = vec![1, 2, 3];
        let M = 10000;
        assert_eq!(solve(&a, m, M), 2);
    }
}
