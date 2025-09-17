mod torus;
use torus::torus::{Torus};


fn main() {
    let torus = Torus::gen_random(5);

    for i in &torus {
        println!("{:?}", i.value);
    }

    // add two torus elements
    let t1 = torus.set[1];
    let t2 = torus.set[2];
    let result = t1 + t2;
    println!("t1 {:.4}", t1.value);
    println!("t2 {:.4}", t2.value);
    println!("result {:.4}", result.value);
}