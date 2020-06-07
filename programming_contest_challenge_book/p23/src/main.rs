fn main() {
    let ant = Ant::new(30, true);
    println!("{:?}", ant);

    let mut ants = Ants::new(vec![Ant::new(30, true), Ant::new(31, false)], 100);
    println!("{:?}", ants);
    &ants.move_1step();
    println!("{:?}", ants);
}

#[derive(Debug)]
struct Ant {
    distance_from_left: i32,
    is_facing_right: bool,
}

impl Ant {
    pub fn new(distance_from_left: i32, is_facing_right: bool) -> Ant {
        Ant {
            distance_from_left,
            is_facing_right,
        }
    }

    fn move_delta(&self) -> i32 {
        if self.is_facing_right {
            1
        } else {
            -1
        }
    }

    fn next_distance_from_left(&self) -> i32 {
        self.distance_from_left + &self.move_delta()
    }

    fn move_position(&mut self) {
        self.distance_from_left = self.next_distance_from_left();
    }

    fn reverse_direction(&mut self) {
        self.is_facing_right = !self.is_facing_right;
    }
}

#[derive(Debug)]
struct Ants {
    ants: Vec<Ant>,
    rod_length: i32,
}

impl Ants {
    pub fn new(ants: Vec<Ant>, rod_length: i32) -> Ants {
        Ants { ants, rod_length }
    }

    fn move_1step(&mut self) {
        let mut i = 0;
        loop {
            if i + 1 >= self.ants.len() {
                break;
            }
            // if i+1 == ants.len() の場合の分岐
            if Ants::judge_collision(&self.ants[i], &self.ants[i + 1]) {
                if let Some(ant) = self.ants.get_mut(i) {
                    ant.reverse_direction();
                }
                if let Some(ant) = self.ants.get_mut(i + 1) {
                    ant.reverse_direction();
                }
                i += 1;
            } else {
                if let Some(ant) = self.ants.get_mut(i) {
                    ant.move_position();
                }
                if let Some(ant) = self.ants.get_mut(i + 1) {
                    ant.move_position();
                }
            }
            i += 1;
        }
    }

    fn judge_collision(ant1: &Ant, ant2: &Ant) -> bool {
        ant1.next_distance_from_left() == ant2.distance_from_left
            && ant1.distance_from_left == ant2.next_distance_from_left()
    }
}

#[cfg(test)]
mod tests {
    use super::Ant;
    use super::Ants;

    #[test]
    fn test_collision_detect() {
        let ant1 = Ant::new(30, true);
        let ant2 = Ant::new(31, false);
        assert_eq!(Ants::judge_collision(&ant1, &ant2), true);
    }
}
