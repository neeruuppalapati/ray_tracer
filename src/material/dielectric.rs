use rand::Rng;

use crate::foundation::ray::*;
use crate::foundation::vector::*;

use super::{Material, MaterialT};

#[derive(Debug)] 
pub struct Dielctric {
    refraction_index: f32,
}

impl Dielctric {
    pub fn new_material(refraction_index: f32) -> MaterialT {
        Dielctric {refraction_index}.into()
    }
}

pub fn refract(v1: &Vector, og: &Vector, index: f32) -> Option<Vector> {
    let v = v1.normalize();
    let t = v.dot();
    let det = 1.0 - index.powf(2.0) * (1 - t.powf(2.0));
    if (det > 0.0) {
        Some(index * (v - og * t) - normal * f32::sqrt(det))
    } 
    else {
        None
    }
}

pub fn schlick_approx(cosine: f32, refraction_index: f32) -> f32 {
    let s = ((1.0 - refraction_index) / (1 .0 + refraction_index)).powf(2.0);
    s + (1.0 - s) * (1.0 - cosine).powf(5.0)
}