use crate::vector::Vector;

pub struct Line<Type, const COUNT: usize> {
    pub origin: Vector<Type, COUNT>,
    direction: Vector<Type, COUNT>,
}
