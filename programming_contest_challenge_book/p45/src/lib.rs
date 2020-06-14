use std::collections::VecDeque;

fn solve(s: &str) -> String {
    let mut t = String::new();
    // &str => VecDeque<char> に変換
    // OPTIMIZE: もっといい方法がある気がする
    let mut s_deque = VecDeque::new();
    for c in s.chars() {
        s_deque.push_back(c)
    }
    loop {
        if s_deque.len() == 0 {
            break;
        }
        // 先頭か末尾化を選ぶ
        // XXX: None 対応していない
        let mut head_index = 0;
        let mut tail_index = s_deque.len() - 1;
        println!("---");
        loop {
            if head_index >= tail_index {
                // 決め打ちで後ろを採用する
                t.push(s_deque.pop_back().unwrap());
                break;
            }

            let head = s_deque[head_index];
            let tail = s_deque[tail_index];
            println!("compare head: {}, tail{}", head, tail);
            if head == tail {
                head_index = head_index + 1;
                tail_index = tail_index - 1;
            } else if head < tail {
                t.push(s_deque.pop_front().unwrap());
                break;
            } else {
                t.push(s_deque.pop_back().unwrap());
                break;
            }
        }
    }
    t
}

#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn it_works() {
        let s = "ACDBCB";
        assert_eq!(solve(s), "ABCBCD");
    }
}
