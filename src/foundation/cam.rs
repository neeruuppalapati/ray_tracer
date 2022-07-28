#[allow(dead_code)]
#[allow(unused_imports)]
use crate::foundation::vector::*;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct CameraActions {
    pub origin: Vector,
    pub direction: Vector,
    pub vertical: Vector,
    pub horizontal: f32,
    pub aspect: f32,
    pub aperture: f32,
    pub dist: f32,
}
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Camera {
    pub actions: CameraActions,
    pub lower_left_corner: Vector,
    pub horizontal: Vector,
    pub vertical: Vector,
    pub origin: Vector,
    pub u: Vector,
    pub v: Vector,
    pub w: Vector,
    pub lens_radius: f32,
}

impl Camera {
    pub fn new(actions: CameraActions) -> Camera {
        let CameraActions {
            origin,
            direction,
            vertical,
            horizontal,
            aspect,
            aperture,
            dist,
        } = actions;
        let theta = horizontal.to_radians();
        let h_width = (theta / 2.0).tan();
        let h_height = h_width / aspect;

        let w = direction.normalize();
        let u = vertical.cross(&w).normalize();
        let v = w.cross(&u);

        Camera {
            actions,
            lower_left_corner: origin - dist * (h_width * u + h_height * v + w),
            vertical: 2.0 * h_height * v,
            horizontal: 2.0 * h_width * u,
            origin,
            u,
            v,
            w,
            lens_radius: aperture / 2.0, 
        }
    }

    pub fn actions(&self) -> CameraActions {
        self.actions
    }
}

