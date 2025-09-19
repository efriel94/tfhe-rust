mod torus;
use torus::torus::{Torus};


fn main() {
    let mut torus = Torus::gen_random(5);

    for i in &torus {
        println!("{:?}", i.value);
    }
    
    println!("size of torus: {}", torus.set.len());
    torus.push(0.375);
    torus.push(0.54);
    torus.push(0.875);
    println!("size of torus: {}", torus.set.len());
    
    let scalar_product = 0 * torus.set[1];
    println!("result: {:?}", scalar_product);
}