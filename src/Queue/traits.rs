pub trait QueueTraits<T> {
    fn new() -> Self;
    fn size(&self) -> Option<usize>;
    fn isEmpty(&self) -> bool;
    fn peek(&self) -> Option<&T>;
    fn offer(&mut self, data: T);
    fn poll(&mut self) -> Option<T>;
}