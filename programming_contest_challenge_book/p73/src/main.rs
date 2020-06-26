use std::collections::BinaryHeap;

fn main() {
    println!("Hello, world!");
}

fn solve(l: i32, p: i32, a: &Vec<i32>, b: &Vec<i32>) -> i32 {
    let mut current_l = 0;
    let mut current_p = p;
    let mut stands = BinaryHeap::new();
    let mut use_stand_cnt = 0;
    for i in 0..a.len() {
        stands.push(b[i]);
        let delta = a[i] - current_l;
        current_l = a[i];
        current_p -= delta;
        if current_p < 0 {
            while current_p < 0 {
                if let Some(c) = stands.pop() {
                    current_p += c;
                    use_stand_cnt += 1;
                } else {
                    return -1;
                }
            }
        }
    }
    use_stand_cnt
}

#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn it_works() {
        let n = 4;
        let l = 25;
        let p = 10;
        let a = vec![10, 14, 20, 21];
        let b = vec![10, 5, 2, 4];
        assert_eq!(solve(l, p, &a, &b), 2);
    }

    #[test]
    fn learn_binary_heap() {
        use std::collections::BinaryHeap;
        let mut heap = BinaryHeap::new();
        assert_eq!(heap.peek(), None);

        // Let's add some scores...
        heap.push(1);
        heap.push(5);
        heap.push(2);

        // Now peek shows the most important item in the heap.
        assert_eq!(heap.peek(), Some(&5));
        assert_eq!(heap.pop(), Some(5));

        // We can check the length of a heap.
        assert_eq!(heap.len(), 2);
    }
}
