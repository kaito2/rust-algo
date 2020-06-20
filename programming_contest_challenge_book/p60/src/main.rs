use std::cmp;
use std::io::Read;

fn main() {
    let mut buf = String::new();

    // 標準入力から全部bufに読み込む
    std::io::stdin().read_to_string(&mut buf).unwrap();

    // 読み込んだStringを空白で分解する
    // println!("buf: {:?}", buf);
    buf = buf.replace("\n", " ");
    buf = buf.replace("\r", " ");
    let mut iter = buf.split_whitespace();

    let n: i32 = iter.next().unwrap().parse().unwrap();
    let max_w: i32 = iter.next().unwrap().parse().unwrap();

    let mut w: Vec<i32> = vec![];
    let mut v: Vec<i32> = vec![];
    for _i in 0..n {
        v.push(iter.next().unwrap().parse().unwrap());
        w.push(iter.next().unwrap().parse().unwrap());
    }
    // println!("{:?}", w);
    // println!("{:?}", v);
    println!("{}", solve(&w, &v, max_w));
}

static MAX_N: i32 = 100;
static MAX_V: i32 = 100;
static INF: i32 = std::i32::MAX - 10000000;

fn solve(w: &Vec<i32>, v: &Vec<i32>, w_max: i32) -> i32 {
    // init dp matrix
    let mut dp: Vec<Vec<i32>> = vec![];
    dp.push(vec![INF; (MAX_N * MAX_V + 1) as usize]);
    for _i in 0..MAX_N + 1 {
        dp.push(vec![0; (MAX_N * MAX_V + 1) as usize]);
    }
    // println!("{:?}", dp);
    dp[0][0] = 0;

    for i in 0..w.len() {
        for j in 0..(MAX_N * MAX_V + 1) {
            if j < v[i] {
                dp[(i + 1) as usize][j as usize] = dp[i as usize][j as usize];
            } else {
                dp[(i + 1) as usize][j as usize] = cmp::min(
                    dp[i as usize][j as usize],
                    dp[i as usize][(j - v[i]) as usize] + w[i],
                );
            }
        }
    }
    let mut ans = 0;
    for i in 0..MAX_N * MAX_V + 1 {
        if dp[w.len() as usize][i as usize] <= w_max {
            ans = i;
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn it_works() {
        let w = vec![2, 1, 3, 2];
        let v = vec![3, 2, 4, 2];
        let w_max = 5;
        assert_eq!(solve(&w, &v, w_max), 7);
    }
}
