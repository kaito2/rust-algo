fn main() {
    let m = 40;
    let k = vec![1, 2, 3, 4, 10];
    println!("{}", if lottery(m, k) { "yes" } else { "no" });
}

fn lottery(m: u32, k: Vec<u32>) -> bool {
    for ka in &k {
        for kb in &k {
            for kc in &k {
                for kd in &k {
                    if (ka + kb + kc + kd) == m {
                        return true;
                    }
                }
            }
        }
    }
    false
}
