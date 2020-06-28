use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    println!("Hello, world!");
}

fn solve(l: &Vec<i32>) -> i32 {
    let mut ans = 0;
    let mut heap = BinaryHeap::new();
    for i in l {
        heap.push(Reverse(*i));
    }
    loop {
        if heap.len() == 1 {
            break;
        }

        if let Some(Reverse(min1)) = heap.pop() {
            if let Some(Reverse(min2)) = heap.pop() {
                ans += min1 + min2;
                heap.push(Reverse(min1 + min2));
            }
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn it_works() {
        let n = 3;
        let l = vec![8, 5, 8];
        assert_eq!(solve(&l), 34)
    }

    #[test]
    fn learn_binary_heap() {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        let mut heap = BinaryHeap::from(vec![Reverse(1), Reverse(2), Reverse(4)]);

        println!("{:?}", heap);

        if let Some(Reverse(hoge)) = heap.pop() {
            if let Some(Reverse(huga)) = heap.pop() {
                heap.push(Reverse(hoge + huga));
            }
        }
        println!("{:?}", heap);
    }
}
