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
    for i in 0..n {
        v.push(iter.next().unwrap().parse().unwrap());
        w.push(iter.next().unwrap().parse().unwrap());
    }
    // println!("{:?}", w);
    // println!("{:?}", v);
    println!("{}", solve(&w, &v, max_w));
}

fn solve(w: &Vec<i32>, v: &Vec<i32>, max_w: i32) -> i32 {
    let mut dp: Vec<Vec<i32>> = vec![];
    for i in 0..w.len() + 1 {
        dp.push(vec![0; (max_w + 1) as usize]);
    }
    for i in 0..w.len() {
        for j in 0..max_w + 1 {
            let mut max = 0;
            let mut k = 0;
            while j - k * w[i] >= 0 {
                let candidate = dp[i][(j - k * w[i]) as usize] + k * v[i];
                if candidate > max {
                    max = candidate;
                }
                k += 1;
            }
            dp[(i + 1) as usize][j as usize] = max;
        }
    }
    // println!("{:?}", dp);
    dp[w.len()][max_w as usize]
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn it_works() {
        let n = 3;
        let max_w = 7;
        let w = vec![3, 4, 2];
        let v = vec![4, 5, 3];
        assert_eq!(solve(&w, &v, max_w), 10);
    }
}
