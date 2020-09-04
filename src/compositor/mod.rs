pub mod basic;

pub trait Compositor<T> {
    fn composite(&self, dst: &T, src: &T, alpha: f64) -> T;
}
