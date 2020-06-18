use std::cmp;

fn main() {
    let q: usize = {
        let mut s = String::new(); // バッファを確保
        std::io::stdin().read_line(&mut s).unwrap(); // 一行読む。失敗を無視
        s.trim_right().to_owned().parse().unwrap() // 改行コードが末尾にくっついてくるので削る
    };
    let mut pairs: Vec<(String, String)> = vec![];
    for i in 0..q {
        let s = read_line_from_stdin();
        let t = read_line_from_stdin();
        pairs.push((s, t));
    }
    for pair in pairs {
        println!("{}", solve(&pair.0, &pair.1));
    }
}

fn read_line_from_stdin() -> String {
    let mut s = String::new(); // バッファを確保
    std::io::stdin().read_line(&mut s).unwrap(); // 一行読む。失敗を無視
    s.trim_right().to_owned() // 改行コードが末尾にくっついてくるので削る
}

fn solve(s: &str, t: &str) -> i32 {
    // initialize vec
    let mut dp = vec![];
    for _i in 0..s.len() + 1 {
        dp.push(vec![0; t.len() + 1])
    }
    let s: Vec<char> = s.chars().collect();
    let t: Vec<char> = t.chars().collect();
    for i in 0..s.len() {
        for j in 0..t.len() {
            // ** 添字がややこしいので注意 **
            // s[i] と s[j] の比較結果は dp[i + 1][j + 1] に対応する
            // (dp は空文字の添字を 0 としているため)
            if s[i] == t[j] {
                dp[i + 1][j + 1] = dp[i][j] + 1;
            } else {
                dp[i + 1][j + 1] = cmp::max(dp[i + 1][j], dp[i][j + 1]);
            }
        }
    }
    // println!("{:?}", dp);
    dp[s.len()][t.len()]
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn it_works() {
        let s = "abcd";
        let t = "becd";
        assert_eq!(solve(s, t), 3);
    }
}
