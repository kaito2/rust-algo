#![feature(test)]

extern crate test;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_for(b: &mut Bencher) {
        b.iter(|| {
            let n = test::black_box(1_000_000_000);
            let v = vec![0; n];
            for i in 0..n {
                let _ = v[i];
            }
        });
    }

    #[bench]
    fn bench_for_each(b: &mut Bencher) {
        let n = 1_000_000_000;
        b.iter(|| {
            let v = test::black_box(vec![0; n]);
            for i in v {
                let _ = i;
            }
        });
    }
}
