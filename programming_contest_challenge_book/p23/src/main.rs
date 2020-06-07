use std::cmp;

fn main() {
    println!("hello, world!");
}

fn calc_max_ant(rod_length: i32, ant_positions: &Vec<i32>) -> i32 {
    let mut max = 0;
    for ant_position in ant_positions {
        let current_ant_max = cmp::max(*ant_position, rod_length - ant_position);
        max = cmp::max(max, current_ant_max);
    }
    max
}

fn calc_min_ant(rod_length: i32, ant_positions: &Vec<i32>) -> i32 {
    let mut min = 0;
    for ant_position in ant_positions {
        let current_ant_max = cmp::min(*ant_position, rod_length - ant_position);
        min = cmp::max(min, current_ant_max);
    }
    min
}

#[cfg(test)]
mod tests {
    use super::calc_max_ant;
    use super::calc_min_ant;

    #[test]
    fn it_works() {
        let rod_length = 10;
        let ant_positions = vec![2, 6, 7];
        assert_eq!(calc_min_ant(rod_length, &ant_positions), 4);
        assert_eq!(calc_max_ant(rod_length, &ant_positions), 8);
    }
}
