use std::cmp::Ordering;
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

    let mut a: Vec<i32> = vec![];
    for _i in 0..n {
        a.push(iter.next().unwrap().parse().unwrap());
    }
    // println!("{:?}", w);
    // println!("{:?}", v);
    println!("{}", solve(&a));
}

fn solve(a: &Vec<i32>) -> i32 {
    let mut dp = vec![std::i32::MAX; a.len()];
    for i in 0..a.len() {
        let index = dp.lower_bound(&a[i]).clone();
        dp[index] = a[i];
    }
    dp.lower_bound(&std::i32::MAX) as i32
}

// copy from: https://github.com/hatoo/competitive-rust-snippets/blob/master/src/binary_search.rs
// ref: https://qiita.com/hatoo@github/items/fa14ad36a1b568d14f3e#%E4%BA%8C%E5%88%86%E6%8E%A2%E7%B4%A2%E3%81%AB%E3%81%A4%E3%81%84%E3%81%A6
/// Equivalent to std::lowerbound and std::upperbound in c++
pub trait BinarySearch<T> {
    fn lower_bound(&self, x: &T) -> usize;
    fn upper_bound(&self, x: &T) -> usize;
}

impl<T: Ord> BinarySearch<T> for [T] {
    fn lower_bound(&self, x: &T) -> usize {
        let mut low = 0;
        let mut high = self.len();

        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                Ordering::Less => {
                    low = mid + 1;
                }
                Ordering::Equal | Ordering::Greater => {
                    high = mid;
                }
            }
        }
        low
    }

    fn upper_bound(&self, x: &T) -> usize {
        let mut low = 0;
        let mut high = self.len();

        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                Ordering::Less | Ordering::Equal => {
                    low = mid + 1;
                }
                Ordering::Greater => {
                    high = mid;
                }
            }
        }
        low
    }
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn it_works() {
        let n = 5;
        let a = vec![4, 2, 3, 1, 5];
        assert_eq!(solve(&a), 3);
    }
}
