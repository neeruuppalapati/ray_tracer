
use std::ops::{Add, Div, Mul, Neg, Sub};
use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,

}

impl Vector {
    // init vector
    pub fn new(x: f32, y: f32, z: f32) -> Vector {
        Vector    { x,
                    y,
                    z }
    }

    // size of vector
    #[inline]
    pub fn size(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    // orthonormalized vector
    #[inline]
    pub fn normalize(&self) -> Vector {
        let size = self.size();
        Vector {
            x: self.x / size,
            y: self.y / size,
            z: self.z / size,
        }
    }

    // dot product
    #[inline]
    pub fn dot(&self, other: &Vector) -> f32 {
        let v1: &Vector = self;
        let v2: &Vector = other;
        v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
    }

    // cross product
    #[inline]
    pub fn cross(&self, other: &Vector) -> Vector {
        let v1: &Vector = self;
        let v2: &Vector = other;
        Vector {
            x: v1.y * v2.z - v1.z * v2.y,
            y: v1.z * v2.x - v1.x * v2.z,
            z: v1.x * v2.y - v1.y * v2.x,
        }
    }

    // reflect vector
    #[inline]
    pub fn reflect(&self, og: &Vector) -> Vector {
        *self - 2. * self.dot(og) * og
    }

}

// negation
impl Neg for Vector {
    type Output = Vector;

    #[inline]
    fn neg(self) -> Vector {
        Vector {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

// operloader overating using rust macros
macro_rules! vector_operator {
    ($name:ident, $function:ident, $operator:tt) => {
        impl $name for Vector {
            type Output = Vector;

            #[inline]
            fn $function(self, other: Vector) -> Vector {
                Vector {
                    x: self.x $operator other.x,
                    y: self.y $operator other.y,
                    z: self.z $operator other.z,
                }
            }
        }
    }
}
vector_operator!(Add, add, +);
vector_operator!(Sub, sub, -);
vector_operator!(Mul, mul, *);
vector_operator!(Div, div, /);

// operator assignment 
macro_rules! vector_assign_operator {
    ($name:ident, $function:ident, $operator:tt) => {
        impl $name for Vector {
            #[inline]
            fn $function(&mut self, v2: Vector) {
                *self = *self $operator v2;
            }
        }
    };
}
vector_assign_operator!(AddAssign, add_assign, +);
vector_assign_operator!(SubAssign, sub_assign, -);
vector_assign_operator!(MulAssign, mul_assign, *);
vector_assign_operator!(DivAssign, div_assign, /);

macro_rules! impl_f32Op {
    ($vector:ty, $name:ident, $function:ident, $operator:tt) => {
        impl $name<f32> for $vector {
            type Output = Vector;

            #[inline]
            fn $function(self, f: f32) -> Vector {
                Vector {
                    x: self.x $operator f,
                    y: self.y $operator f,
                    z: self.z $operator f,
                }
            }
        }

        impl $name<$vector> for f32 {
            type Output = Vector;

            #[inline]
            fn $function(self, v: $vector) -> Vector {
                Vector {
                    x: v.x $operator self,
                    y: v.y $operator self,
                    z: v.z $operator self,
                }
            }
        }
    };
}

impl_f32Op!(Vector, Mul, mul, *);
impl_f32Op!(Vector, Div, div, /);
impl_f32Op!(&Vector, Mul, mul, *);
impl_f32Op!(&Vector, Div, div, /);

macro_rules! impl_f32OpAssign {
    ($name:ident, $function:ident, $operator:tt) => {
        impl $name<f32> for Vector {
            #[inline]
            fn $function(&mut self, f: f32) {
                *self = *self $operator f;
            }
        }
    };
}

impl_f32OpAssign!(MulAssign, mul_assign, *);
impl_f32OpAssign!(DivAssign, div_assign, /);

// vector tests
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_vector_size() {
        let v = Vector::new(1.0, 2.0, 3.0);
        assert_eq!(v.size(), 3.7416573867739413);
    }
    #[test]
    fn test_vector_mul() {
        let v = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(2.0, 3.0, 4.0);
        assert_eq!(v.mul(v2), Vector::new(2.0, 6.0, 12.0));
    }
    #[test]
    fn test_vector_normalize() {
        let v1 = Vector::new(1.0, 2.0, 2.0);
        let v2 = v1.normalize();
        assert_eq!(v2.y, 0.666666686);

    }
    #[test]
    fn test_vector_dot() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(4.0, 5.0, 6.0);
        assert_eq!(v1.dot(&v2), 32.0);
    }
    #[test]
    fn test_vector_cross() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(4.0, 5.0, 6.0);
        let v3 = v1.cross(&v2);
        assert_eq!(v3.x, -3.0);
        assert_eq!(v3.y, 6.0);
        assert_eq!(v3.z, -3.0);
    }
    #[test]
    fn test_vector_add() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(4.0, 5.0, 6.0);
        let v3 = v1.add(v2);
        assert_eq!(v3.x, 5.0);
    }
}

