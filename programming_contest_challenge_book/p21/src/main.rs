use std::collections::VecDeque;
use std::iter::FromIterator;

fn main() {
    println!("Hello, world!");
}

pub fn get_max_triangle_len(a: &Vec<u32>) -> u32 {
    // REVIEW: clone 使っていいの?
    let mut a_vec = a.clone();
    a_vec.sort();
    a_vec.reverse();
    let mut a_deque = VecDeque::from_iter(a_vec);
    // sort desc
    let mut left_edges = a_deque.split_off(3);
    let mut candidate_edges = a_deque;
    let mut len = 0;
    loop {
        if judge_triangle(&candidate_edges) {
            // REVIEW: 関数化
            len = candidate_edges[0] + candidate_edges[1] + candidate_edges[2];
            break;
        }
        candidate_edges.pop_front();
        match left_edges.pop_front() {
            None => break,
            Some(edge) => candidate_edges.push_back(edge),
        }
    }
    len
}

// NOTE: 前提
// * 降順にソートされている前提
// * 長さが3のベクトル
fn judge_triangle(edges: &VecDeque<u32>) -> bool {
    edges[0] < (edges[1] + edges[2])
}

#[cfg(test)]
mod test {
    use super::get_max_triangle_len;
    #[test]
    fn it_works() {
        let a: Vec<u32> = vec![10, 5, 4, 3, 2];
        assert_eq!(get_max_triangle_len(&a), 12);
        let a: Vec<u32> = vec![2, 3, 4, 5, 10];
        assert_eq!(get_max_triangle_len(&a), 12);
        let a: Vec<u32> = vec![4, 5, 10, 20];
        assert_eq!(get_max_triangle_len(&a), 0);
    }

    use std::collections::VecDeque;
    #[test]
    fn vec_deque() {
        // dequeue enqueue の挙動を確認
        let mut a = VecDeque::from(vec![0, 1, 2, 3, 4]);
        a.pop_front();
        assert_eq!(a, VecDeque::from(vec![1, 2, 3, 4]));
        a.push_back(5);
        // split_off の挙動を確認
        assert_eq!(a, VecDeque::from(vec![1, 2, 3, 4, 5]));
        let got = a.split_off(3);
        print!("a: {:?}, got: {:?}\n", a, got);

        // cast の挙動を確認
        let origin = vec![0, 1, 2, 3, 4];
        let casted = VecDeque::from(origin.clone());
        assert_eq!(casted, VecDeque::from(vec![0, 1, 2, 3, 4]));
    }
}
