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
    let mut cost = l_halfway.iter().sum();
    if l_halfway.len() == 1 {
        return cost;
    }
    while l_halfway.len() != 1 {
        let mut max1_i = 0;
        let mut max2_i = 1;
        if l_halfway[max1_i] < l_halfway[max2_i] {
            mem::swap(&mut max1_i, &mut max2_i);
        }
        for i in 2..l_halfway.len() {
            if l_halfway[i] > l_halfway[max2_i] {
                max2_i = i;
            }
            if l_halfway[max2_i] > l_halfway[max1_i] {
                mem::swap(&mut max1_i, &mut max2_i);
            }
        }
        l_halfway[max1_i] += l_halfway[max2_i];
        cost += l_halfway[max1_i];
        l_halfway.remove(max2_i);
    }
    cost
}
