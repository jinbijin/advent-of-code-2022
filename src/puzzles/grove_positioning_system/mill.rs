use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

pub struct MillNode<T> {
    contents: T,
    previous: RefCell<Option<Weak<MillNode<T>>>>,
    next: RefCell<Option<Weak<MillNode<T>>>>,
}

/// A circular linked list with immutable iterator.
pub struct Mill<T> {
    nodes: RefCell<Vec<Rc<MillNode<T>>>>,
}

impl<T> Mill<T>
where
    T: Copy + PartialEq,
{
    pub fn len(&self) -> usize {
        self.nodes.borrow().len()
    }

    pub fn find_value(&self, value: T) -> Option<usize> {
        let nodes = self.nodes.borrow();
        let node = nodes
            .iter()
            .enumerate()
            .find(|(_, node)| node.contents == value);
        node.map(|(index, _)| index)
    }

    pub fn get_value(&self, index: usize) -> T {
        self.nodes.borrow()[index].contents
    }

    pub fn get_value_next_from(&self, index: usize, count: usize) -> T {
        let count = count % self.len();
        let mut node = self.nodes.borrow()[index].clone();

        for _ in 0..count {
            if let Some(next_node) = &*node.clone().next.borrow() {
                if let Some(next_node) = next_node.upgrade() {
                    node = next_node;
                }
            }
        }

        node.contents
    }

    pub fn get_value_previous_from(&self, index: usize, count: usize) -> T {
        let count = count % self.len();
        let mut node = self.nodes.borrow()[index].clone();

        for _ in 0..count {
            if let Some(previous_node) = &*node.clone().previous.borrow() {
                if let Some(previous_node) = previous_node.upgrade() {
                    node = previous_node;
                }
            }
        }

        node.contents
    }

    pub fn move_value_next(&self, index: usize, count: u64) {
        let count = count % ((self.len() - 1) as u64);
        if count == 0 {
            return ();
        }

        let node = self.nodes.borrow()[index].clone();

        // Short-circuit where node is removed
        let previous_node = if let Some(previous_node) = &*node.clone().previous.borrow() {
            if let Some(previous_node) = previous_node.upgrade() {
                previous_node
            } else {
                unreachable!("if self.count >= 2");
            }
        } else {
            unreachable!("if self.count >= 2");
        };

        let next_node = if let Some(next_node) = &*node.clone().next.borrow() {
            if let Some(next_node) = next_node.upgrade() {
                next_node
            } else {
                unreachable!("if self.count >= 2");
            }
        } else {
            unreachable!("if self.count >= 2");
        };

        *previous_node.next.borrow_mut() = Some(Rc::downgrade(&next_node));
        *next_node.previous.borrow_mut() = Some(Rc::downgrade(&previous_node));

        // If nothing happened, we'd just insert the node back between these two...
        let mut previous_node = previous_node;
        let mut next_node = next_node;

        // ...so we can move this spot over count places...
        for _ in 0..count {
            if let Some(next_next_node) = &*next_node.clone().next.borrow() {
                if let Some(next_next_node) = next_next_node.upgrade() {
                    previous_node = next_node;
                    next_node = next_next_node;
                }
            }
        }

        // ...and tie the list back together.
        *previous_node.next.borrow_mut() = Some(Rc::downgrade(&node));
        *next_node.previous.borrow_mut() = Some(Rc::downgrade(&node));
        *node.next.borrow_mut() = Some(Rc::downgrade(&next_node));
        *node.previous.borrow_mut() = Some(Rc::downgrade(&previous_node));
    }

    pub fn move_value_previous(&self, index: usize, count: u64) {
        let count = count % ((self.len() - 1) as u64);
        if count == 0 {
            return ();
        }

        let node = self.nodes.borrow()[index].clone();

        // Short-circuit where node is removed
        let previous_node = if let Some(previous_node) = &*node.clone().previous.borrow() {
            if let Some(previous_node) = previous_node.upgrade() {
                previous_node
            } else {
                unreachable!("if self.count >= 2");
            }
        } else {
            unreachable!("if self.count >= 2");
        };

        let next_node = if let Some(next_node) = &*node.clone().next.borrow() {
            if let Some(next_node) = next_node.upgrade() {
                next_node
            } else {
                unreachable!("if self.count >= 2");
            }
        } else {
            unreachable!("if self.count >= 2");
        };

        *previous_node.next.borrow_mut() = Some(Rc::downgrade(&next_node));
        *next_node.previous.borrow_mut() = Some(Rc::downgrade(&previous_node));

        // If nothing happened, we'd just insert the node back between these two...
        let mut previous_node = previous_node;
        let mut next_node = next_node;

        // ...so we can move this spot over count places...
        for _ in 0..count {
            if let Some(previous_previous_node) = &*previous_node.clone().previous.borrow() {
                if let Some(previous_previous_node) = previous_previous_node.upgrade() {
                    next_node = previous_node;
                    previous_node = previous_previous_node;
                }
            }
        }

        // ...and tie the list back together.
        *previous_node.next.borrow_mut() = Some(Rc::downgrade(&node));
        *next_node.previous.borrow_mut() = Some(Rc::downgrade(&node));
        *node.next.borrow_mut() = Some(Rc::downgrade(&next_node));
        *node.previous.borrow_mut() = Some(Rc::downgrade(&previous_node));
    }

    fn new() -> Mill<T> {
        Mill {
            nodes: RefCell::new(Vec::new()),
        }
    }

    fn push(&mut self, item: T) {
        let first_node = self.nodes.borrow().first().map(Rc::downgrade);
        let last_node = self.nodes.borrow().last().map(Rc::downgrade);
        let new_node = Rc::new(MillNode {
            contents: item,
            previous: RefCell::new(last_node.clone()),
            next: RefCell::new(first_node.clone()),
        });
        if let Some(last_node) = last_node {
            if let Some(last_node) = last_node.upgrade() {
                *last_node.next.borrow_mut() = Some(Rc::downgrade(&new_node));
            }
        }
        if let Some(first_node) = first_node {
            if let Some(first_node) = first_node.upgrade() {
                *first_node.previous.borrow_mut() = Some(Rc::downgrade(&new_node));
            }
        }
        self.nodes.borrow_mut().push(new_node);
    }
}

impl<T> From<Vec<T>> for Mill<T>
where
    T: Copy + PartialEq,
{
    fn from(items: Vec<T>) -> Self {
        let mut mill: Mill<T> = Mill::new();
        for item in items {
            mill.push(item);
        }
        mill
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create() {
        let mill: Mill<usize> = vec![].into();

        assert_eq!(mill.len(), 0);
    }

    #[test]
    fn should_push_one() {
        let mill: Mill<usize> = vec![24].into();

        assert_eq!(mill.len(), 1);
    }

    #[test]
    fn should_push_two() {
        let mill: Mill<usize> = vec![24, 42].into();

        assert_eq!(mill.len(), 2);
    }

    #[test]
    fn should_push_three() {
        let mill: Mill<usize> = vec![24, 42, 12].into();

        assert_eq!(mill.len(), 3);
    }

    #[test]
    fn should_get_value() {
        let mill: Mill<usize> = vec![24, 42, 12].into();

        assert_eq!(mill.get_value(0), 24);
        assert_eq!(mill.get_value(1), 42);
        assert_eq!(mill.get_value(2), 12);
    }

    #[test]
    fn should_get_value_next() {
        let mill: Mill<usize> = vec![24, 42, 12, 13].into();

        assert_eq!(mill.get_value_next_from(0, 0), 24);
        assert_eq!(mill.get_value_next_from(0, 1), 42);
        assert_eq!(mill.get_value_next_from(0, 2), 12);
        assert_eq!(mill.get_value_next_from(0, 3), 13);
    }

    #[test]
    fn should_get_value_next_overflow() {
        let mill: Mill<usize> = vec![24, 42, 12, 13].into();

        assert_eq!(mill.get_value_next_from(2, 0), 12);
        assert_eq!(mill.get_value_next_from(2, 1), 13);
        assert_eq!(mill.get_value_next_from(2, 2), 24);
        assert_eq!(mill.get_value_next_from(2, 3), 42);
    }

    #[test]
    fn should_get_value_previous() {
        let mill: Mill<usize> = vec![24, 42, 12, 13].into();

        assert_eq!(mill.get_value_previous_from(0, 0), 24);
        assert_eq!(mill.get_value_previous_from(0, 1), 13);
        assert_eq!(mill.get_value_previous_from(0, 2), 12);
        assert_eq!(mill.get_value_previous_from(0, 3), 42);
    }

    #[test]
    fn should_move_value_next_values_not_modified() {
        let mill: Mill<usize> = vec![24, 42, 12, 13].into();
        mill.move_value_next(0, 1);

        assert_eq!(mill.get_value(0), 24);
        assert_eq!(mill.get_value(1), 42);
        assert_eq!(mill.get_value(2), 12);
        assert_eq!(mill.get_value(3), 13);
    }

    #[test]
    fn should_move_value_next_cycle_correct() {
        let mill: Mill<usize> = vec![24, 42, 12, 13].into();
        mill.move_value_next(0, 1);

        assert_eq!(mill.get_value_next_from(0, 0), 24);
        assert_eq!(mill.get_value_next_from(0, 1), 12);
        assert_eq!(mill.get_value_next_from(0, 2), 13);
        assert_eq!(mill.get_value_next_from(0, 3), 42);

        assert_eq!(mill.get_value_previous_from(0, 0), 24);
        assert_eq!(mill.get_value_previous_from(0, 1), 42);
        assert_eq!(mill.get_value_previous_from(0, 2), 13);
        assert_eq!(mill.get_value_previous_from(0, 3), 12);
    }

    #[test]
    fn should_move_value_previous_values_not_modified() {
        let mill: Mill<usize> = vec![24, 42, 12, 13].into();
        mill.move_value_previous(0, 1);

        assert_eq!(mill.get_value(0), 24);
        assert_eq!(mill.get_value(1), 42);
        assert_eq!(mill.get_value(2), 12);
        assert_eq!(mill.get_value(3), 13);
    }

    #[test]
    fn should_move_value_previous_cycle_correct() {
        let mill: Mill<usize> = vec![24, 42, 12, 13].into();
        mill.move_value_previous(0, 1);

        assert_eq!(mill.get_value_next_from(0, 0), 24);
        assert_eq!(mill.get_value_next_from(0, 1), 13);
        assert_eq!(mill.get_value_next_from(0, 2), 42);
        assert_eq!(mill.get_value_next_from(0, 3), 12);

        assert_eq!(mill.get_value_previous_from(0, 0), 24);
        assert_eq!(mill.get_value_previous_from(0, 1), 12);
        assert_eq!(mill.get_value_previous_from(0, 2), 42);
        assert_eq!(mill.get_value_previous_from(0, 3), 13);
    }
}
