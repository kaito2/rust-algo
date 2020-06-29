#[derive(Debug)]
struct Node {
    value: i32,
    lch: Option<Box<Node>>,
    rch: Option<Box<Node>>,
}

impl Node {
    pub fn new(num: i32) -> Self {
        Self {
            value: num,
            lch: None,
            rch: None,
        }
    }

    fn insert(&mut self, value: i32) {
        if value >= self.value {
            if let Some(rch_node) = &mut self.rch {
                rch_node.insert(value)
            } else {
                self.rch = Some(Box::new(Self::new(value)));
            }
        } else {
            if let Some(lch_node) = &mut self.lch {
                lch_node.insert(value)
            } else {
                self.lch = Some(Box::new(Self::new(value)));
            }
        }
    }

    fn search(&self, value: i32) -> bool {
        if value == self.value {
            true
        } else if value > self.value {
            if let Some(rch_node) = &self.rch {
                if rch_node.value == value {
                    true
                } else {
                    rch_node.search(value)
                }
            } else {
                false
            }
        } else {
            if let Some(lch_node) = &self.lch {
                if lch_node.value == value {
                    true
                } else {
                    lch_node.search(value)
                }
            } else {
                false
            }
        }
    }

    fn delete(&mut self, value: i32) {
        // TODO: implement
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::Node;

    #[test]
    fn it_works() {
        let mut node = Node::new(3);
        node.insert(5);
        node.insert(2);
        node.insert(6);
        node.insert(4);

        println!("{:?}", node);

        assert_eq!(node.search(3), true);
        assert_eq!(node.search(5), true);
        assert_eq!(node.search(2), true);
        assert_eq!(node.search(6), true);
        assert_eq!(node.search(4), true);

        assert_eq!(node.search(1), false);
        assert_eq!(node.search(7), false);
    }
}
