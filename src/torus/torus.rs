use std::ops::Add;
use rand::Rng; // bring the trait into scope


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TorusElement {

    // todo - I need to perform a check that the initialized value is in the correct range or convert implicitly.
    pub value: f32,
}

#[derive(Debug, Clone)]
pub struct Torus {
    pub set: Vec<TorusElement>,
    element_position: usize,
}

impl Torus {
    //create a new torus group of size n, default elements zero
    pub fn new(n: usize) -> Self {
        let set: Vec<TorusElement> = (0..n)
            .map(|_| TorusElement { value: 0 as f32})
            .collect();
        Torus { set, element_position: 0 }
    }

    //create a new torus group of size n of random elements 
    pub fn gen_random(n: usize) -> Self {
        let mut rng = rand::rng();
        let set: Vec<TorusElement> = (0..n)
            .map(|_| TorusElement { value: rng.random_range(0.0..1.0) as f32})
            .collect();
        Torus { set, element_position: 0 }
    }
}

impl Iterator for Torus {
    type Item = TorusElement;

    fn next(&mut self) -> Option<Self::Item> {

        if self.element_position < self.set.len() {
            let item = self.set[self.element_position];
            self.element_position += 1;
            Some(item)
        } else {
            None
        }
    }
}

impl<'a> IntoIterator for &'a Torus {
    type Item = &'a TorusElement;
    type IntoIter = std::slice::Iter<'a, TorusElement>;

    fn into_iter(self) -> Self::IntoIter {
        self.set.iter()
    }
}
