fn dfs(a: &Vec<i32>, k: i32) -> bool {
    dfs_iter(a, k, 0, 0)
}

fn dfs_iter(a: &Vec<i32>, k: i32, i: usize, sum: i32) -> bool {
    if i == a.len() {
        return sum == k;
    }
    if dfs_iter(a, k, i + 1, sum) {
        return true;
    }
    if dfs_iter(a, k, i + 1, sum + a[i]) {
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::dfs;
    #[test]
    fn it_works() {
        let a = vec![1, 2, 4, 7];
        let k = 13;
        assert_eq!(dfs(&a, k), true);
        let k = 15;
        assert_eq!(dfs(&a, k), false);
    }
}
