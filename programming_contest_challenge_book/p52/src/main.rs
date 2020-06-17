use std::cmp;
use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

fn solve(wv: &Vec<(i32, i32)>, w: i32) -> i32 {
    let mut cache: HashMap<(i32, i32), i32> = HashMap::new();
    solve_iter(0, w, wv, &mut cache)
}

// i 番目移行で重さ j 以下になるような最大値
fn solve_iter(i: i32, j: i32, wv: &Vec<(i32, i32)>, cache: &mut HashMap<(i32, i32), i32>) -> i32 {
    if let Some(v) = cache.get(&(i, j)) {
        return *v;
    }
    let res: i32;
    if i as usize == wv.len() {
        res = 0;
    } else if j < wv[i as usize].0 {
        res = solve_iter(i + 1, j, wv, cache);
    } else {
        res = cmp::max(
            solve_iter(i + 1, j, wv, cache),
            solve_iter(i + 1, j - wv[i as usize].0, wv, cache) + wv[i as usize].1,
        );
    }
    cache.insert((i, j), res);
    res
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{self, AtomicI32};
    #[test]
    fn atomic_test() {
        // Atomic を用いれば global な変数を定義できそうだが、
        // 並列化したいわけではないので &mut にして引数で渡すほうが丸そう。
        static CNT: AtomicI32 = AtomicI32::new(0);
        fn closure() -> i32 {
            CNT.fetch_add(1, atomic::Ordering::SeqCst)
        }
        assert_eq!(closure(), 0);
        assert_eq!(closure(), 1);
        assert_eq!(closure(), 2);
    }

    use super::solve;
    #[test]
    fn it_works() {
        let wv = vec![(2, 3), (1, 2), (3, 4), (2, 2)];
        let w = 5;
        assert_eq!(solve(&wv, w), 7);
    }
}
