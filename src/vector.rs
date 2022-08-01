use std::{
    ops,
    convert,
    default,
};
// use crate::line::Line;

#[derive(Debug, Copy, Clone)]
pub struct Vector<Type, const COUNT: usize> {
    pub values: [Type; COUNT],
}

impl<Type, const COUNT: usize> convert::From<[Type; COUNT]> for Vector<Type, COUNT> {
    fn from(to_convert: [Type; COUNT]) -> Self {
        Vector { values: to_convert }
    }
}

impl<Type, const COUNT: usize> ops::Index<usize> for Vector<Type, COUNT> {
    type Output = Type;
    fn index(&self, index: usize) -> &Type {
        &self.values[index]
    }
}

impl<Type, const COUNT: usize> ops::IndexMut<usize> for Vector<Type, COUNT> {
    fn index_mut(&mut self, index: usize) -> &mut Type {
        &mut self.values[index]
    }
}

impl<Type, const COUNT: usize> ops::AddAssign for Vector<Type, COUNT>
    where Type: ops::AddAssign + Copy {
    fn add_assign(&mut self, to_add: Self) {
        for index in 0..COUNT {
            self[index] += to_add[index];
        }
    }
}

impl<Type, const COUNT: usize> ops::Add for Vector<Type, COUNT>
    where Type: ops::AddAssign + Copy {
    type Output = Self;
    fn add(self, to_add: Self) -> Self {
        let mut output = self;
        for index in 0..COUNT {
            output[index] += to_add[index];
        }
        output
    }
}

impl<Type, const COUNT: usize> ops::SubAssign          for Vector<Type, COUNT>
    where Type: ops::SubAssign + Copy {
    fn sub_assign(&mut self, to_subtract: Self) {
        for index in 0..COUNT {
            self[index] -= to_subtract[index];
        }
    }
}

impl<Type, const COUNT: usize> ops::Sub for Vector<Type, COUNT>
    where Type: ops::SubAssign + Copy {
    type Output = Self;
    fn sub(self, to_subtract: Self) -> Self {
        let mut output = self;
        for index in 0..COUNT {
            output[index] -= to_subtract[index];
        }
        output
    }
}

impl<Type, const COUNT: usize> Vector<Type, COUNT>
    where Type: default::Default + Copy {
    fn remove(&self, index: usize) -> Vector<Type, { COUNT - 1 }> {
        let mut output = Vector { values: [ Type::default(); {COUNT - 1}] };
        for index2 in 0..index { output[index2] = self[index2] }
        for index2 in index+1..COUNT { output[index2 - 1] = self[index2] }
        output
    }
    fn insert(&self, index: usize, value: &Type) -> Vector<Type, { COUNT + 1 }> {
        let mut output = Vector { values: [ Type::default(); {COUNT + 1}] };
        for index2 in 0..index { output[index2] = self[index2] }
        output[index] = *value;
        for index2 in index+1..COUNT { output[index2] = self[index2 - 1] }
        output
    }
}

impl<const COUNT: usize> Vector<f64, COUNT> {
    fn length(&self) -> f64 {
        let mut length: f64 = 0.0;
        for value in self.values { length += value * value }
        length.sqrt()
    }
}

impl<Type> Vector<Type, 2>
    where Type: Copy {
    pub fn x(&self) -> Type { self[0] }
    pub fn y(&self) -> Type { self[1] }
    pub fn x_mut(&mut self) -> &mut Type { &mut self[0] }
    pub fn y_mut(&mut self) -> &mut Type { &mut self[1] }
}

impl<Type> Vector<Type, 3>
    where Type: Copy {
    pub fn x(&self) -> Type { self[0] }
    pub fn y(&self) -> Type { self[1] }
    pub fn z(&self) -> Type { self[2] }
    pub fn x_mut(&mut self) -> &mut Type { &mut self[0] }
    pub fn y_mut(&mut self) -> &mut Type { &mut self[1] }
    pub fn z_mut(&mut self) -> &mut Type { &mut self[2] }
}

#[macro_export]
macro_rules! vector {
    [$($value:expr),*] => (Vector::from([$($value),*]))
}

impl Vector<f64, 2> {
    pub fn rotate(&self, angle: f64) -> Self {
        vector![
            self.x() * angle.cos() - self.y() * angle.sin(),
            self.x() * angle.sin() + self.y() * angle.cos()
        ]
    }
    pub fn rotate_around(&self, angle: f64, pivot: &Self) -> Self {
        (*self - *pivot).rotate(angle) + *pivot
    }
}

impl Vector<f64, 3> {
    pub fn rotate(&self, angle: f64, normal: Self) -> Self {
        let mut output = self.clone();
        println!("{:?}", &output);
        output = output
            .remove(0)
            .rotate(-(normal.z()/normal.y()).atan())
            .insert(0, &output[0]);
        println!("{:?}", &output);
        output = output
            .remove(2)
            .rotate(-(-normal.remove(0).length()/normal.x()).atan())
            .insert(2, &output[2]);
        println!("{:?}", &output);
        output
    }
}
