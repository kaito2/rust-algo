fn main() {
    println!("Hello, world!");
}

fn solve(a: &Vec<i32>, m: &Vec<i32>, k: i32) -> bool {
    let mut dp: Vec<i32> = vec![-1; (k + 1) as usize];
    dp[0] = 0;
    for i in 0..a.len() {
        for j in 0..k + 1 {
            if dp[j as usize] >= 0 {
                dp[j as usize] = m[i];
            } else if j < a[i] || dp[(j - a[i]) as usize] <= 0 {
                // ^^^ `dp[j - a[j]] == 0` の場合ももうa[i]がないので合計がkにはならない
                dp[j as usize] = -1;
            } else {
                dp[j as usize] = dp[(j - a[i]) as usize] - 1;
            }
        }
        println!("{:?}", dp);
    }
    dp[k as usize] >= 0
}

#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn it_works() {
        // let n = 3;
        let a = vec![3, 5, 8];
        let m = vec![3, 2, 2];
        let k = 17;
        assert_eq!(solve(&a, &m, k), true);
    }
}
