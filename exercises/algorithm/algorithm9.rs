use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()], // 索引0不使用，从1开始
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// 添加元素到堆中，并执行上浮操作维持堆特性
    pub fn add(&mut self, value: T) {
        // 1. 将新元素添加到数组末尾（堆的最后位置）
        self.count += 1;
        self.items.push(value);
        // 2. 从当前位置（count）向上浮，维持堆特性
        let mut idx = self.count;
        while idx > 1 {
            let parent_idx = self.parent_idx(idx);
            // 比较当前节点和父节点，若符合比较规则则交换（如最小堆：当前节点 < 父节点）
            if (self.comparator)(&self.items[idx], &self.items[parent_idx]) {
                self.items.swap(idx, parent_idx);
                idx = parent_idx;
            } else {
                // 满足堆特性，退出循环
                break;
            }
        }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    /// 找到当前节点的子节点中，符合堆比较规则的子节点索引
    /// （最小堆返回最小子节点，最大堆返回最大子节点）
    fn smallest_child_idx(&self, idx: usize) -> usize {
        let left_idx = self.left_child_idx(idx);
        let right_idx = self.right_child_idx(idx);

        // 只有左子节点，直接返回左子节点索引
        if right_idx > self.count {
            return left_idx;
        }

        // 比较左右子节点，返回符合比较规则的那个索引
        if (self.comparator)(&self.items[left_idx], &self.items[right_idx]) {
            left_idx
        } else {
            right_idx
        }
    }

    /// 下沉操作：从指定索引开始，向下调整维持堆特性
    fn sink(&mut self, mut idx: usize) {
        while self.children_present(idx) {
            let child_idx = self.smallest_child_idx(idx);
            // 比较当前节点和子节点，若不符合比较规则则交换
            if !(self.comparator)(&self.items[idx], &self.items[child_idx]) {
                self.items.swap(idx, child_idx);
                idx = child_idx;
            } else {
                // 满足堆特性，退出循环
                break;
            }
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default + Clone, // 注意：如果是非Copy类型，需要Clone；若允许转移所有权，可改为take
{
    type Item = T;

    /// 弹出堆顶元素（迭代器的next方法），并执行下沉操作维持堆特性
    fn next(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        // 1. 保存堆顶元素（要返回的结果）
        let top = self.items[1].clone(); // 若为非Clone类型，可使用std::mem::replace
        // 2. 将最后一个元素移到堆顶
        let last = self.items.pop().unwrap();
        self.count -= 1;

        if !self.is_empty() {
            self.items[1] = last;
            // 3. 从堆顶开始下沉，维持堆特性
            self.sink(1);
        }

        Some(top)
    }
}

// 注意：原代码中Iterator的next如果要支持非Clone类型，可替换为以下实现（使用take）：
// 需将items改为Vec<Option<T>>，这里为了兼容测试用例，使用Clone更简单

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}
