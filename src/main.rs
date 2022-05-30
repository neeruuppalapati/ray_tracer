mod chapter1;
use crate::chapter1::point_and_vector_test::*;
fn main() {
    let proj = Projectile::new(Tuple::point(0.0, 1.0, 0.0), 
                                            Tuple::vector(1.0, 1.0, 0.0).normalize());
    let env = Environment::new(Tuple::point(0.0, -0.1, 0.0), 
                                            Tuple::vector(-0.01, 0.0, 0.0));
    let mut t = proj.position.y();
    let mut proggers = proj;
    while t > 0.0 {
        proggers = tick(&env, &proggers);
        t = proggers.position.y();
        println!("{:?}", proggers.position);
    }
}