pub struct QueueNode<T> {
    value: T,
    next: Box<Option<QueueNode<T>>>,
}

impl<T> QueueNode<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            next: Box::new(None),
        }
    }

    pub fn push(&mut self, value: T) {
        if let Some(node) = self.next.as_mut() {
            node.push(value);
        } else {
            self.next = Box::new(Some(QueueNode::new(value)));
        }
    }

    fn down(&mut self) {
        if let Some(node) = self.next.take() {
            self.value = node.value;
            self.next = node.next;
        }
    }
}

pub struct CircularBuffer<T> {
    capacity: usize,
    len: usize,
    head: Option<QueueNode<T>>,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            len: 0,
            head: None,
        }
    }

    pub fn write(&mut self, value: T) -> Result<(), Error> {
        if self.len < self.capacity {
            self.overwrite(value);
            Ok(())
        } else {
            Err(Error::FullBuffer)
        }
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if let Some(head) = self.head.take() {
            self.head = *head.next;
            self.len -= 1;
            Ok(head.value)
        } else {
            Err(Error::EmptyBuffer)
        }
    }

    pub fn clear(&mut self) {
        self.head.take();
        self.len = 0;
    }

    pub fn overwrite(&mut self, value: T) {
        if let Some(head) = self.head.as_mut() {
            if self.len < self.capacity {
                self.len += 1;
            } else {
                head.down();
            }
            head.push(value);
        } else {
            self.head = Some(QueueNode::new(value));
            self.len += 1;
        }
    }
}
