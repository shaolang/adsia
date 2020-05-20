#[derive(Debug)]
struct Pair<T> {
    item: T,
    priority: i8,
}

#[derive(Debug)]
pub struct Heap<T> {
    pairs: Vec<Pair<T>>,
    num_child: usize,
}

impl <T> Heap<T> {
    pub fn new(num_child: usize) -> Heap<T> {
        Heap { num_child, pairs: Vec::new() }
    }

    pub fn top(&mut self) -> Option<T> {
        match self.pairs.len() {
            0 => None,
            1 => Some(self.pairs.pop().unwrap().item),
            n => {
                self.pairs.swap(0, n - 1);
                let item = self.pairs.pop().unwrap().item;
                self.push_down(0);
                Some(item)
            }
        }
    }

    pub fn peek(&self) -> Option<&T> {
        if self.pairs.is_empty() {
            None
        } else {
            Some(&self.pairs.get(0).unwrap().item)
        }
    }

    pub fn insert(&mut self, item: T, priority: i8) {
        let pair = Pair { item, priority };
        self.pairs.push(pair);
        self.bubble_up(self.pairs.len() - 1);
    }

    pub fn remove(_item: T) {
        todo!();
    }

    pub fn update(_item: T, _new_priority: i8) {
        todo!();
    }

    fn bubble_up(&mut self, index: usize) {
        let mut parent_index = index;

        while parent_index > 0 {
            let cur_index = parent_index;
            parent_index = get_parent_index(parent_index, self.num_child);

            if self.priority(parent_index) < self.priority(cur_index) {
                self.pairs.swap(parent_index, cur_index);
            } else {
                break;
            }
        }
    }

    fn priority(&self, index: usize) -> i8 {
        self.pairs[index].priority
    }

    fn push_down(&mut self, index: usize) {
        let mut cur_index = index;

        while cur_index < self.first_leaf_index() {
            let child_index = self.highest_priority_child(cur_index);

            if self.priority(child_index) > self.priority(cur_index) {
                self.pairs.swap(child_index, cur_index);
                cur_index = child_index;
            } else {
                break;
            }
        }
    }

    fn first_leaf_index(&self) -> usize {
        if self.pairs.len() < 2 {
            0
        } else {
            (self.pairs.len() - 2) / self.num_child + 1
        }
    }

    fn highest_priority_child(&self, index: usize) -> usize {
        let indices = (0..self.num_child)
            .map(|i| self.num_child * index + i)
            .collect::<Vec<usize>>();
        *indices.iter()
            .fold(&indices[0], |acc, x|
                  if self.pairs[*acc].priority > self.pairs[*x].priority {
                      acc
                  } else {
                      x
                  })
    }
}


fn get_parent_index(index: usize, num_child: usize) -> usize {
    (index - 1) / num_child
}


#[cfg(test)]
mod tests {
    use crate::pq::{Heap, get_parent_index};

    #[test]
    fn get_parent_index_for_num_child_as_two() {
        let num_child = 2;
        assert_eq!(get_parent_index(7, num_child), 3);
        assert_eq!(get_parent_index(8, num_child), 3);
    }

    #[test]
    fn get_parent_index_for_num_child_as_three() {
        let num_child = 3;
        assert_eq!(get_parent_index(10, num_child), 3);
        assert_eq!(get_parent_index(11, num_child), 3);
        assert_eq!(get_parent_index(12, num_child), 3);
    }

    #[test]
    fn insert_keeps_root_intact_when_new_item_has_lower_priority() {
        let mut h: Heap<String> = Heap::new(2);
        h.insert("hello".to_string(), 10);
        h.insert("world".to_string(), 9);

        assert_eq!(h.peek().unwrap(), "hello");
    }

    #[test]
    fn insert_changes_root_when_new_item_has_higher_priority() {
        let mut h: Heap<String> = Heap::new(2);
        h.insert("hello".to_string(), 9);
        h.insert("world".to_string(), 10);

        assert_eq!(h.peek().unwrap(), "world");
    }

    #[test]
    fn insert_keeps_root_intact_when_third_item_is_only_higher_than_second() {
        let mut h: Heap<String> = Heap::new(2);
        h.insert("hello".to_string(), 10);
        h.insert("world".to_string(), 8);
        h.insert("universe".to_string(), 9);

        assert_eq!(h.peek().unwrap(), "hello");
    }

    #[test]
    fn top_removes_first_item_in_heap() {
        let mut h: Heap<i8> = Heap::new(2);
        h.insert(100, 10);
        h.insert(90, 9);

        h.top().unwrap();
        assert_eq!(h.peek().unwrap(), &90);
    }

    #[test]
    fn top_ensures_highest_priority_child_gets_bubbled_up() {
        let mut h: Heap<i8> = Heap::new(3);
        h.insert(100, 10);
        h.insert(80, 8);
        h.insert(40, 4);
        h.insert(70, 7);

        h.insert(60, 6);
        h.insert(50, 5);
        h.insert(90, 9);

        h.insert(30, 3);

        h.top().unwrap();
        println!("{:?}", h);
        assert_eq!(h.peek().unwrap(), &90);
    }
}
