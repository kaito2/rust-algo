use std::io::Read;
use std::mem;

fn main() {
    let mut buf = String::new();

    // 標準入力から全部bufに読み込む
    std::io::stdin().read_to_string(&mut buf).unwrap();

    // 読み込んだStringを空白で分解する
    // println!("buf: {:?}", buf);
    buf = buf.replace("\n", " ");
    buf = buf.replace("\r", " ");
    let mut iter = buf.split_whitespace();
    // println!("iter: {:?}", iter);

    let q: usize = iter.next().unwrap().parse().unwrap();
    // println!("{}", q);
    // println!("iter: {:?}", iter);

    let lrt: Vec<u64> = (0..q)
        .map(|_| iter.next().unwrap().parse().unwrap())
        .collect();
    // println!("input: {:?}", lrt);
    println!("{}", solve(&lrt));
}

fn solve(l: &Vec<u64>) -> u64 {
    let mut l_halfway = l.clone();
    let mut cost = 0;
    if l_halfway.len() == 1 {
        return l_halfway[1];
    }
    while l_halfway.len() != 1 {
        let mut min1_i = 0;
        let mut min2_i = 1;
        if l_halfway[min1_i] > l_halfway[min2_i] {
            mem::swap(&mut min1_i, &mut min2_i);
        }
        for i in 2..l_halfway.len() {
            if l_halfway[i] < l_halfway[min2_i] {
                min2_i = i;
            }
            if l_halfway[min2_i] < l_halfway[min1_i] {
                mem::swap(&mut min1_i, &mut min2_i);
            }
        }
        l_halfway[min1_i] += l_halfway[min2_i];
        l_halfway.remove(min2_i);
        cost += l_halfway[min1_i];
    }
    cost
}
