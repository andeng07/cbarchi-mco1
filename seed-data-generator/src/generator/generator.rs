pub trait Generator<T> {
    fn generate(&mut self) -> T;
}
