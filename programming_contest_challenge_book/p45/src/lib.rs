use std::collections::VecDeque;

fn solve(s: &str) -> String {
    let mut t = String::new();
    // &str => VecDeque<char> に変換
    // OPTIMIZE: もっといい方法がある気がする
    let mut s_deque = VecDeque::new();
    for c in s.chars() {
        s_deque.push_back(c)
    }
    // ---
    loop {
        if s_deque.len() == 0 {
            break;
        }
        // 先頭か末尾化を選ぶ
        // XXX: None 対応していない
        let head = s_deque[0];
        let tail = s_deque[s_deque.len() - 1];
        if head < tail {
            t.push(s_deque.pop_front().unwrap());
        } else {
            t.push(s_deque.pop_back().unwrap());
        }
    }
    t
}

#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn it_works() {
        let s = "ACDBDB";
        assert_eq!(solve(s), "ABCBCD");
    }
}
