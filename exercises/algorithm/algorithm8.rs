#[derive(Debug)]
pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.elements.push(value)
    }

    pub fn dequeue(&mut self) -> Result<T, &str> {
        if !self.elements.is_empty() {
            Ok(self.elements.remove(0usize))
        } else {
            Err("Queue is empty")
        }
    }

    pub fn peek(&self) -> Result<&T, &str> {
        match self.elements.first() {
            Some(value) => Ok(value),
            None => Err("Queue is empty"),
        }
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }
}

pub struct myStack<T> {
    // q1作为主队列，存储栈的元素，队首是栈顶
    q1: Queue<T>,
    // q2作为辅助队列，用于压入元素时的临时存储
    q2: Queue<T>,
}

impl<T> myStack<T> {
    pub fn new() -> Self {
        Self {
            q1: Queue::<T>::new(),
            q2: Queue::<T>::new(),
        }
    }

    /// 压入元素到栈顶（后进先出）
    pub fn push(&mut self, elem: T) {
        // 1. 将新元素加入辅助队列q2
        self.q2.enqueue(elem);
        // 2. 将q1中的所有元素转移到q2中
        while let Ok(val) = self.q1.dequeue() {
            self.q2.enqueue(val);
        }
        // 3. 交换q1和q2，此时q1的队首就是栈顶元素
        std::mem::swap(&mut self.q1, &mut self.q2);
    }

    /// 弹出栈顶元素，返回Result（失败时返回"Stack is empty"）
    pub fn pop(&mut self) -> Result<T, &str> {
        // 直接弹出q1的队首元素（栈顶），如果q1为空则返回栈空错误
        self.q1.dequeue().map_err(|_| "Stack is empty")
    }

    /// 判断栈是否为空
    pub fn is_empty(&self) -> bool {
        // 只需判断主队列q1是否为空
        self.q1.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue() {
        let mut s = myStack::<i32>::new();
        assert_eq!(s.pop(), Err("Stack is empty"));
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Ok(3));
        assert_eq!(s.pop(), Ok(2));
        s.push(4);
        s.push(5);
        assert_eq!(s.is_empty(), false);
        assert_eq!(s.pop(), Ok(5));
        assert_eq!(s.pop(), Ok(4));
        assert_eq!(s.pop(), Ok(1));
        assert_eq!(s.pop(), Err("Stack is empty"));
        assert_eq!(s.is_empty(), true);
    }
}
