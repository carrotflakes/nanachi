pub mod normal;

pub trait Compositor<T> {
    fn composite(&self, a: &T, b: &T, alpha: f64) -> T;
}
