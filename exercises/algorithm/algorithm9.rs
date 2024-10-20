/*
	heap
	This question requires you to implement a binary heap function
*/


use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default + Ord + Clone,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default + Ord + Clone,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: Vec::new(),
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        self.items.push(value);
        self.count += 1;
        self.bubble_up(self.count - 1);
    }

    fn parent_idx(&self, idx: usize) -> usize {
        (idx - 1) / 2
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        2 * idx + 1
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        2 * idx + 2
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        let left_child_idx = self.left_child_idx(idx);
        let right_child_idx = self.right_child_idx(idx);

        if right_child_idx >= self.count {
            return left_child_idx;
        }

        if (self.comparator)(&self.items[left_child_idx], &self.items[right_child_idx]) {
            left_child_idx
        } else {
            right_child_idx
        }
    }

    fn bubble_up(&mut self, mut idx: usize) {
        while idx > 0 {
            let parent_idx = self.parent_idx(idx);
            if !(self.comparator)(&self.items[parent_idx], &self.items[idx]) {
                self.items.swap(parent_idx, idx);
                idx = parent_idx;
            } else {
                break;
            }
        }
    }

    fn bubble_down(&mut self, mut idx: usize) {
        while self.left_child_idx(idx) < self.count {
            let smallest_child_idx = self.smallest_child_idx(idx);
            if !(self.comparator)(&self.items[smallest_child_idx], &self.items[idx]) {
                self.items.swap(smallest_child_idx, idx);
                idx = smallest_child_idx;
            } else {
                break;
            }
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        self.items.swap(0, self.count - 1);
        let result = self.items.pop();
        self.count -= 1;
        self.bubble_down(0);
        result
    }
}

impl<T> Heap<T>
where
    T: Default + Ord + Clone,
{
    pub fn new_min() -> Self {
        Self::new(|a, b| a <= b)
    }

    pub fn new_max() -> Self {
        Self::new(|a, b| a >= b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default + Ord + Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.pop()
    }
}

pub struct MinHeap;

impl MinHeap {
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord + Clone,
    {
        Heap::new_min()
    }
}

pub struct MaxHeap;

impl MaxHeap {
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord + Clone,
    {
        Heap::new_max()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heap() {
        let mut min_heap = MinHeap::new::<i32>();
        min_heap.add(3);
        min_heap.add(1);
        min_heap.add(6);
        assert_eq!(min_heap.pop(), Some(1));
        assert_eq!(min_heap.pop(), Some(6));
        assert_eq!(min_heap.pop(), Some(3));

        let mut max_heap = MaxHeap::new::<i32>();
        max_heap.add(3);
        max_heap.add(1);
        max_heap.add(6);
        assert_eq!(max_heap.pop(), Some(6));
        assert_eq!(max_heap.pop(), Some(1));
        assert_eq!(max_heap.pop(), Some(3));
    }
}