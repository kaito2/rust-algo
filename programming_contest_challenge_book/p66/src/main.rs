fn main() {
    println!("Hello, world!");
}

fn solve(n: i32, m: i32, M: i32) -> i32 {
    let mut dp: Vec<Vec<i32>> = vec![vec![0; (n + 1) as usize]; (m + 1) as usize];
    dp[0][0] = 1;
    for i in 1..=m as usize {
        for j in 0..=n as usize {
            if j >= i {
                dp[i][j] = dp[i - 1][j] + dp[i][j - i];
            } else {
                dp[i][j] = dp[i - 1][j];
            }
        }
    }
    dp[m as usize][n as usize]
}

#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn it_works() {
        let n = 4;
        let m = 3;
        let M = 10000;
        assert_eq!(solve(n, m, M), 4);
    }
}
