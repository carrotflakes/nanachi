pub mod basic;
pub mod basic_enum;

pub trait Compositor<T> {
    fn composite(&self, dst: &T, src: &T, alpha: f64) -> T;
}
