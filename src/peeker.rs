pub trait Peeker<T> {
    fn peek(&self, offset: usize) -> Option<T>;
    fn consume(&mut self, offset: usize) -> Option<T>;
}
