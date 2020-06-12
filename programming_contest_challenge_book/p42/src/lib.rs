fn solve (coins: Vec<i32>, a: i32) -> u32 {
    let coin_values: Vec<i32> = vec![500, 100, 50, 10, 5, 1];
    
    let mut left = a;
    let mut cnt = 0;
    for (coin_cnt, coin_value) in coins.iter().zip(coin_values.iter()) {
        println!("coin_value: {}, coint_cnt: {}", coin_value, *coin_cnt);
        let mut current_coin_used = 0;
        let mut current_coin_cnt = *coin_cnt;
        while left >= *coin_value && current_coin_cnt > 0 {
            left -= *coin_value;
            current_coin_cnt -= 1;
            current_coin_used += 1;
            println!("left: {}, coin_value: {}, coint_cnt: {}, used: {}", left, coin_value, current_coin_cnt, current_coin_used);
        }
        cnt += current_coin_used;
    }
    cnt
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn it_works() {
        let coins = vec![2,0,3,1,2,3];
        assert_eq!(solve(coins, 620), 6);
    }
}
