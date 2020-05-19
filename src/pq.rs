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

    pub fn top(&mut self) -> T {
        todo!();
    }

    pub fn peek(&self) -> T {
        todo!();
    }

    pub fn insert(_item: T, _priority: i8) {
        todo!();
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
        todo!();
    }

    fn highest_priority_child(&self, index: usize) -> usize {
        let indices = (1..self.num_child + 1)
            .map(|i| self.num_child * index + i)
            .collect::<Vec<usize>>();
        *indices.iter().max().unwrap()
    }
}


fn get_parent_index(index: usize, num_child: usize) -> usize {
    (index - 1) / num_child
}


#[cfg(test)]
mod tests {
    use crate::pq::get_parent_index;

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
}
