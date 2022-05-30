// Chapter 1
// Tuples, Points, and Vectors

// creating structs

pub mod point_and_vector_test {
    static EPSILON: f64 = 0.00001;
    use std::*;
    #[derive(Debug, Copy, Clone)]
    pub struct Tuple {
         x: f64,
         y: f64,
         z: f64,
         w : f64,
    }

    pub struct Projectile {
        pub position: Tuple,
        pub velocity: Tuple,
    }
    pub struct Environment {
        pub gravity: Tuple,
        pub wind: Tuple,
    }

    impl Projectile {
        pub fn new(position: Tuple, velocity: Tuple) -> Projectile {
            Projectile {
                position,
                velocity,
            }
        }
    }
    impl Environment {
        pub fn new(gravity: Tuple, wind: Tuple) -> Environment {
            Environment {
                gravity,
                wind,
            }
        }
    }
    pub fn tick(env: &Environment, projectile: &Projectile) -> Projectile {
        Projectile::new(projectile.position + projectile.velocity,
                        projectile.velocity + env.gravity + env.wind)
    }


    impl Tuple {
        pub fn point(x: f64, y: f64, z: f64) -> Tuple {
            Tuple {
                x: x,
                y: y,
                z: z,
                w: 1.0,
            }
        }
        pub fn vector(x: f64, y: f64, z: f64) -> Tuple {
            Tuple {
                x: x,
                y: y,
                z: z,
                w: 0.0,
            }
        }

        // getters
        pub fn x(&self) -> f64 {
            self.x
        }
        pub fn y(&self) -> f64 {
            self.y
        }
        pub fn z(&self) -> f64 {
            self.z
        }
        pub fn w(&self) -> f64 {
            self.w
        }

        // setters
        pub fn is_point(&self) -> bool {
            self.w == 1.0
        }
        pub fn is_vector(&self) -> bool {
            self.w == 0.0
        }
        pub fn negate(&self) -> Tuple {
            Tuple {
                x: -self.x,
                y: -self.y,
                z: -self.z,
                w: -self.w,
            }
        }
        pub fn magnitude(&self) -> f64 {
            (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
        }
        pub fn normalize(&mut self) -> Tuple {
            let m = self.magnitude();
            Tuple {
                x: self.x / m,
                y: self.y / m,
                z: self.z / m,
                w: self.w / m,
            }
        }
        pub fn dot(&self, other: Tuple) -> f64 {
            self.x * other.x + self.y * other.y + self.z * other.z
        }
        // cross product does not modify vector status
        pub fn cross(&self, other: Tuple) -> Tuple {
            Tuple {
                x: self.y * other.z - self.z * other.y,
                y: self.z * other.x - self.x * other.z,
                z: self.x * other.y - self.y * other.x,
                w: 0.0,
            }
        }
        
    }
   
    // overloading operators
    impl ops::Add for Tuple {
        type Output = Tuple;
        fn add(self, _rhs: Tuple) -> Tuple {
            Tuple {
                x: self.x + _rhs.x,
                y: self.y + _rhs.y,
                z: self.z + _rhs.z,
                w: self.w + _rhs.w,
            }
        }
    }

    impl ops::Sub for Tuple {
        type Output = Tuple;
        fn sub(self, other: Tuple) -> Tuple {
            Tuple {
                x: self.x - other.x,
                y: self.y - other.y,
                z: self.z - other.z,
                w: self.w - other.w,
            }
        }
    }
    // scalar multiplication
    impl ops::Mul<f64> for Tuple {
        type Output = Tuple;
        fn mul(self, k: f64) -> Tuple {
            Tuple {
                x: self.x * k,
                y: self.y * k,
                z: self.z * k,
                w: self.w * k,
            }
        }
    }
    impl ops::Mul<Tuple> for Tuple {
        type Output = Tuple;
        fn mul(self, other: Tuple) -> Tuple {
            Tuple {
                x: self.x * other.x,
                y: self.y * other.y,
                z: self.z * other.z,
                w: self.w * other.w,
            }
        }
    }
    // scalar division
    impl ops::Div<f64> for Tuple {
        type Output = Tuple;
        fn div(self, k: f64) -> Tuple {
            Tuple {
                x: self.x / k,
                y: self.y / k,
                z: self.z / k,
                w: self.w / k,
            }
        }
    }
    impl ops::Div<Tuple> for Tuple {
        type Output = Tuple;
        fn div(self, other: Tuple) -> Tuple {
            Tuple {
                x: self.x / other.x,
                y: self.y / other.y,
                z: self.z / other.z,
                w: self.w / other.w,
            }
        }
    }
    impl PartialEq for Tuple {
        fn eq(&self, other: &Tuple) -> bool {
            (self.x - other.x).abs() < EPSILON &&
            (self.y - other.y).abs() < EPSILON &&
            (self.z - other.z).abs() < EPSILON &&
            (self.w - other.w).abs() < EPSILON
        }
    }

            
        
    

  

    #[test]
    fn test_point_new() {
        let p = Tuple::point(4.3, -4.2, 3.1);
        assert_eq!(p.x, 4.3);
        assert_eq!(p.y, -4.2);
        assert_eq!(p.z, 3.1);
        assert_eq!(p.w, 1.0);
        assert_ne!(p.w, 0.0);
        assert_eq!(p.is_point(), true);
    }

    #[test]
    fn test_vector_new() {
        let v = Tuple::vector(4.3, -4.2, 3.1);
        assert_eq!(v.x, 4.3);
        assert_eq!(v.y, -4.2);
        assert_eq!(v.z, 3.1);
        assert_eq!(v.w, 0.0);
        assert_ne!(v.w, 1.0);
        assert_eq!(v.is_vector(), true);
    }

    #[test]
    fn test_point_add() {
        let p1 = Tuple::point(3.0, -2.0, 5.0);
        let p2 = Tuple::point(-2.0, 3.0, 1.0);
        let p3 = p1 + p2;
        assert_eq!(p3.x, 1.0);
        assert_eq!(p3.y, 1.0);
        assert_eq!(p3.z, 6.0);
        assert_eq!(p3.w, 2.0);
        assert_eq!(p3.is_point(), false);
    }

    #[test]
    fn test_vector_add() {
        let v1 = Tuple::vector(3.0, -2.0, 5.0);
        let v2 = Tuple::vector(-2.0, 3.0, 1.0);
        let v3 = v1 + v2;
        assert_eq!(v3.x, 1.0);
        assert_eq!(v3.y, 1.0);
        assert_eq!(v3.z, 6.0);
        assert_eq!(v3.w, 0.0);
        assert_eq!(v3.is_vector(), true);
    }

    #[test]
    fn test_vector_plus_point_is_point() {
        let v1 = Tuple::vector(1.0, 2.0, 3.0);
        let p1 = Tuple::point(4.0, 5.0, 6.0);
        let p2 = v1 + p1;
        assert_eq!(p2.is_point(), true);
    }

    #[test]
    fn test_point_sub() {
        let p1 = Tuple::point(3.0, 2.0, 1.0);
        let p2 = Tuple::point(5.0, 6.0, 7.0);
        let p3 = p1 - p2;
        assert_eq!(p3.x, -2.0);
        assert_eq!(p3.y, -4.0);
        assert_eq!(p3.z, -6.0);
        assert_eq!(p3.w, 0.0);
        assert_eq!(p3.is_vector(), true);
    }
    #[test]
    fn test_vector_sub() {
        let v1 = Tuple::vector(3.0, 2.0, 1.0);
        let v2 = Tuple::vector(5.0, 6.0, 7.0);
        let v3 = v1 - v2;
        assert_eq!(v3.x, -2.0);
        assert_eq!(v3.y, -4.0);
        assert_eq!(v3.z, -6.0);
        assert_eq!(v3.w, 0.0);
        assert_eq!(v3.is_vector(), true);
    }
    #[test]
    fn test_point_sub_vector_is_point() {
        let p1 = Tuple::point(3.0, 2.0, 1.0);
        let v1 = Tuple::vector(5.0, 6.0, 7.0);
        let p2 = p1 - v1;
        assert_eq!(p2.x, -2.0);
        assert_eq!(p2.y, -4.0);
        assert_eq!(p2.z, -6.0);
        assert_eq!(p2.is_point(), true);
    }
    #[test]
    fn sub_vec_from_zero_vec() {
        let v0 = Tuple::vector(0.0, 0.0, 0.0);
        let v1 = Tuple::vector(1.0, -2.0, 3.0);
        let v2 = v0 - v1;
        assert_eq!(v2.x, -1.0);
        assert_eq!(v2.y, 2.0);
        assert_eq!(v2.z, -3.0);
        assert_eq!(v2.is_vector(), true);
    }
    #[test]
    fn tuple_negate() {
        let mut v1 = Tuple::vector(1.0, -2.0, 3.0);
        let mut p1 = Tuple::point(-1.0, 2.0, -3.0);
        v1 = v1.negate();
        p1 = p1.negate();
        assert_eq!(v1.x, -1.0);
        assert_eq!(v1.y, 2.0);
        assert_eq!(v1.z, -3.0);
        assert_eq!(v1.is_vector(), true);
        assert_eq!(p1.x, 1.0);
        assert_eq!(p1.y, -2.0);
        assert_eq!(p1.z, 3.0);
    }
    #[test]
    fn test_scalar_mul() {
        let v1 = Tuple::vector(1.0, -2.0, 3.0);
        let v2 = v1 * 3.5;
        assert_eq!(v2.x, 3.5);
        assert_eq!(v2.y, -7.0);
        assert_eq!(v2.z, 10.5);
        assert_eq!(v2.is_vector(), true);
    }
    #[test]
   fn test_regular_mul() {
        let v1 = Tuple::vector(1.0, -2.0, 3.0);
        let v2 = v1 * v1;
        assert_eq!(v2.x, 1.0);
        assert_eq!(v2.y, 4.0);
        assert_eq!(v2.z, 9.0);
        assert_eq!(v2.is_vector(), true);
   }
   #[test]
   fn test_scalar_div() {
        let v1 = Tuple::vector(1.0, -2.0, 3.0);
        let v2 = v1 / 2.0;
        assert_eq!(v2.x, 0.5);
        assert_eq!(v2.y, -1.0);
        assert_eq!(v2.z, 1.5);
   }
   #[test]
   fn test_simple_mag() {
        let v1 = Tuple::vector(1.0, 0.0, 0.0);
        let v1mag = v1.magnitude();
        assert_eq!(v1mag, 1.0);
   }
   #[test]
   fn test_magnitude() {
        let v1 = Tuple::vector(1.0, 2.0, 3.0);
        let v1mag = v1.magnitude();
        let v1mag_approx = 3.74165;
        assert_eq!((v1mag - v1mag_approx).abs() < EPSILON, true);
   }
  
   #[test]
   fn test_simple_dot_product() {
        let v1 = Tuple::vector(1.0, 2.0, 3.0);
        let v2 = Tuple::vector(2.0, 3.0, 4.0);
        let v1dotv2 = v1.dot(v2);
        assert_eq!(v1dotv2, 20.0);
   }
    #[test]
    fn test_complex_dot_product() {
        let v1 = Tuple::vector(1.0, 2.0, 3.0);
        let v2 = Tuple::vector(2.0, 3.23, 4.23);
        let v1dotv2 = v1.dot(v2);
        let v1dotv2_approx = 21.15000;
        assert_eq!((v1dotv2 - v1dotv2_approx).abs() < EPSILON, true);
    }
   #[test]
   fn test_simple_cross_product() {
        let v1 = Tuple::vector(1.0, 2.0, 3.0);
        let v2 = Tuple::vector(2.0, 3.0, 4.0);
        let v1crossv2 = v1.cross(v2);
        let v1crossv2_approx = Tuple::vector(-1.0, 2.0, -1.0);
        assert_eq!(v1crossv2, v1crossv2_approx);
   }
    #[test]
    fn test_complex_cross_product() {
        let v1 = Tuple::vector(1.0, 2.0, 3.0);
        let v2 = Tuple::vector(2.0, 3.23, 4.23);
        let v1crossv2 = v1.cross(v2);
        let v1crossv2_approx = Tuple::vector(-1.23, 1.77, -0.77);
        assert_eq!(v1crossv2, v1crossv2_approx);
    }
 
}