use std::cmp::Ordering;

#[derive(Debug)]
pub enum BST<T: Ord> {
    Leaf {
        value: T,
        left: Box<BST<T>>,
        right: Box<BST<T>>,
    },
    Empty,
}

impl<T: Ord> BST<T> {
    pub fn new() -> Self {
        BST::Empty
    }

    pub fn create(value: T) -> Self {
        BST::Leaf {
            value,
            left: Box::new(BST::Empty),
            right: Box::new(BST::Empty),
        }
    }

    pub fn has_left(&self) -> bool {
        return match self {
            Self::Empty => false,
            Self::Leaf {
                ref value,
                ref left,
                ref right,
            } => match **left {
                BST::Empty => false,
                BST::Leaf { value, left, right } => true,
            },
        };
    }

    pub fn insert(&mut self, new_value: T) {
        match self {
            BST::Leaf {
                ref value,
                ref mut left,
                ref mut right,
            } => match new_value.cmp(value) {
                Ordering::Less => left.insert(new_value),
                Ordering::Greater => right.insert(new_value),
                Ordering::Equal => return,
            },
            BST::Empty => *self = BST::create(new_value),
        }
    }

    pub fn delete(&mut self, value: T) {
        /*
        let mut all_none = false;
        match self {
            BST::Empty => {}
            BST::Leaf {
                ref value,
                ref mut left,
                ref mut right,
            } => {}
        }
        */
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_has_left() {
        let n = super::BST::create(3);
        assert_eq!(n.has_left(), false);
    }
    /*

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
    */
}
