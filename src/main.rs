// use the the files from basics/vector.rs and basics/ray.rs to test your code.

pub mod foundation;

fn main() {
    let v1 = foundation::vector::Vector::new(1., 0., 0.);
    let v2 = foundation::vector::Vector::new(0., 1., 0.);
    let v3 = v1 + v2;
    println!("{:?}", v3);
}



