use crate::foundation::vector::{Vector, rand_in_unit_circle};
use crate::foundation::cam::*;

// essentially just directional vector even tho vectors are already directional

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Ray {
    pub origin: Vector,
    pub direction: Vector,
}

impl Ray {
    pub fn new(origin: Vector, direction: Vector) -> Ray {
        Ray { origin, direction }
    }
    pub fn at(&self, f: f32) -> Vector {
        self.origin + f * self.direction
    }
}

// get ray releative to a point from camera pov
pub fn get_ray(cam: &Camera, u: f32, v: f32) -> Ray {
    let radius = cam.lens_radius * rand_in_unit_circle();
    let offset = (cam.u * radius.x) + (cam.v * radius.y);
    let origin  = cam.origin + offset;
    Ray::new(
        origin,
        cam.lower_left_corner + u * cam.horizontal + v * cam.vertical - origin,
    )

}

// ray tests
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ray_at() {
        let r = Ray::new(Vector::new(0., 0., 0.), Vector::new(0., 0., 1.));
        assert_eq!(r.at(0.), Vector::new(0., 0., 0.));
        assert_eq!(r.at(1.), Vector::new(0., 0., 1.));
        assert_eq!(r.at(-1.), Vector::new(0., 0., -1.));
    }
}

