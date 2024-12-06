use crate::Queue::traits::QueueTraits;

pub(crate) struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

pub(crate) struct Queue<T> {
    first: Option<Box<Node<T>>>,
    last: Option<Box<Node<T>>>,
    size: usize,

}

impl<T> QueueTraits<T> for Queue<T> {
    fn new() -> Self {
        Queue {
            first: None,
            last: None,
            size: 0,
        }
    }

    fn size(&self) -> Option<usize> {
        match self.first {
            Some(_) => Some(self.size),
            None => None,
        }
    }
    
    fn isEmpty(&self) -> bool {
        self.first.is_none()
    }

    fn peek(&self) -> Option<&T> {
        match &self.first {
            Some(ref node) => Some(&node.data),
            None => None,
        }
    }

    fn offer(&mut self, data: T) {
        let node: Box<Node<T>> = Box::new(Node {data, next: None,});

        if self.size().unwrap_or(0) == 0 {
           self.first = Some(node);

        } else {
            if let Some(ref mut last) = self.last {
                last.next = Some(node);
            }
        }
        self.last = self.last.as_mut().unwrap().next.take();

        self.size += 1;
    }

    fn poll(&mut self) -> Option<T> {
        if let Some(mut old_first) = self.first.take() {
            self.first = old_first.next.take();
    
            if self.first.is_none() {
                self.last = None;
            }
            
            self.size -= 1;
            Some(old_first.data)
        } else {
            None
        }
    }
    
}