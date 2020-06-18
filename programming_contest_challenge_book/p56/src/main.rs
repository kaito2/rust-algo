// use std::fs::File;
// use std::io::{self, BufRead, BufReader};

fn main() {
    let q: usize = {
        let mut s = String::new(); // バッファを確保
        std::io::stdin().read_line(&mut s).unwrap(); // 一行読む。失敗を無視
        s.trim_right().to_owned().parse().unwrap() // 改行コードが末尾にくっついてくるので削る
    };
    let mut pairs: Vec<(String, String)> = vec![];
    for i in 0..q {
        let s = read_line_from_stdin();
        let t = read_line_from_stdin();
        pairs.push((s, t));
    }
    println!("{:?}", pairs);
}

fn read_line_from_stdin() -> String {
    let mut s = String::new(); // バッファを確保
    std::io::stdin().read_line(&mut s).unwrap(); // 一行読む。失敗を無視
    s.trim_right().to_owned() // 改行コードが末尾にくっついてくるので削る
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(1 + 1, 2);
    }
}
