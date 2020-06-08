fn main() {
    println!("Hello, world!");
}

fn binary_search(v: &Vec<i32>, x: i32) -> bool {
    let mut left = 0;
    let mut right = v.len();

    while right - left >= 1 {
        let center = (left + right) / 2;
        if x == v[center] {
            return true;
        } else if x > v[center] {
            left = center + 1;
        } else {
            right = center;
        };
    }
    false
}

fn solve(v: Vec<i32>, m: i32) -> bool {
    let mut kk = vec![0; v.len() * v.len() as usize];
    for c in 0..v.len() {
        for d in 0..v.len() {
            kk[c * v.len() + d] = v[c] + v[d];
        }
    }

    kk.sort();

    for a in 0..v.len() {
        for b in 0..v.len() {
            if binary_search(&kk, m - v[a] - v[b]) {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::binary_search;
    use super::solve;

    #[test]
    fn test_solve() {
        assert_eq!(solve(vec![1, 3, 5], 10), true);
        assert_eq!(solve(vec![1, 3, 5], 9), false);
    }

    #[test]
    fn it_works() {
        let v: Vec<i32> = vec![2, 4, 6, 9, 11, 13];
        assert_eq!(binary_search(&v, 2), true);
        assert_eq!(binary_search(&v, 9), true);
        println!("*** 13 ***");
        assert_eq!(binary_search(&v, 13), true);
        println!("*** 10 ***");
        assert_eq!(binary_search(&v, 10), false);
        println!("*** 1 ***");
        assert_eq!(binary_search(&v, 1), false);
        println!("*** 15 ***");
        assert_eq!(binary_search(&v, 15), false);
    }

    #[test]
    fn divide() {
        assert_eq!(3 / 2, 1);
        assert_eq!(3.0 / 2.0, 1.5);
    }
}
