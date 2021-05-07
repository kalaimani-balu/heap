pub struct Heap<T> where T: Ord {
    nodes: Vec<T>,
    order: HeapOrder
}

enum HeapOrder {
    MIN, MAX
}

impl<T> Heap<T> where T: Ord {
    pub fn min_heap() -> Self {
        Heap {nodes: vec![], order: HeapOrder::MIN}
    }

    pub fn max_heap() -> Self {
        Heap {nodes: vec![], order: HeapOrder::MAX}
    }

    pub fn push(&mut self, elem: T) {
        self.nodes.push(elem);
        self.heapify_up(self.nodes.len() - 1);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len() == 0 {
            return None;
        }
        let elem = self.nodes.swap_remove(0);
        self.heapify_down(0);
        Some(elem)
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    #[inline]
    fn parent(&self, index: usize) -> usize {
        (index - 1) / 2
    }

    #[inline]
    fn left(&self, index: usize) -> usize {
        index * 2 + 1
    }

    #[inline]
    fn right(&self, index: usize) -> usize {
        index * 2 + 2
    }

    fn compare(&self, index1: usize, index2: usize) -> bool {
        match self.order {
            HeapOrder::MAX => self.nodes[index1] > self.nodes[index2],
            HeapOrder::MIN => self.nodes[index1] < self.nodes[index2],
        }
    }

    fn heapify_up(&mut self, mut index: usize) {
        if index == 0 {
            return;
        }

        let mut parent = self.parent(index);

        while self.compare(index, parent) {
            self.nodes.swap(index, parent);
            index = parent;
            if index == 0 {
                return;
            }
            parent = self.parent(index);
        }
    }

    fn heapify_down(&mut self, mut index: usize) {
        if self.len() <= 1 {
            return;
        }

        let mut left = self.left(index);
        let mut right= self.right(index);
        let last_index = self.len() - 1;

        while left <= last_index {
            let child_to_compare  = if left == last_index {
                left
            } else if self.compare(left, right) {
                left
            } else {
                right
            };

            if self.compare(child_to_compare, index)  {
                self.nodes.swap(index, child_to_compare);
                index   = child_to_compare;
                left    = self.left(index);
                right   = self.right(index);
            } else {
                return;
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::Heap;

    #[test]
    fn it_works() {
        let numbers = vec![23, 43, 53, 11, 10, 34, 77];
        let mut heap = Heap::max_heap();

        for i in numbers {
            heap.push(i);
        }

        assert_eq!(Some(77), heap.pop());
        assert_eq!(Some(53), heap.pop());
        assert_eq!(Some(43), heap.pop());
        assert_eq!(Some(34), heap.pop());
        assert_eq!(Some(23), heap.pop());
        assert_eq!(Some(11), heap.pop());
        assert_eq!(Some(10), heap.pop());
        assert_eq!(None, heap.pop());

        let numbers = vec![23, 43, 53, 11, 10, 34, 77];
        let mut heap = Heap::min_heap();

        for i in numbers {
            heap.push(i);
        }

        assert_eq!(Some(10), heap.pop());
        assert_eq!(Some(11), heap.pop());
        assert_eq!(Some(23), heap.pop());
        assert_eq!(Some(34), heap.pop());
        assert_eq!(Some(43), heap.pop());
        assert_eq!(Some(53), heap.pop());
        assert_eq!(Some(77), heap.pop());
        assert_eq!(None, heap.pop());
    }
}
