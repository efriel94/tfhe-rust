use std::ops::{Add,Neg,Sub,Mul};
use rand::Rng; 


#[derive(Debug, Clone, Copy)]
pub struct TorusElement {
    pub value: f32,
}

impl TorusElement {
    fn new(torus_element: f32) -> TorusElement {

        let check = (torus_element > 0.0) && (torus_element < 1.0);
        if check {
            TorusElement { value: torus_element}
        } else {
            //perform implicit conversion mod 1 so that its in the format R/Z mod 1
            TorusElement { value: ((torus_element).rem_euclid(1.0)) }
        }
    }
}


// Torus structure used to represent the group structure which is represented here as a set of TorusElements
#[derive(Debug, Clone)]
pub struct TorusGroup {
    pub set: Vec<TorusElement>,
    element_position: usize,
}

impl TorusGroup {
    //create a new torus group of size n, default elements zero
    pub fn new(n: usize) -> Self {
        let set: Vec<TorusElement> = (0..n)
            .map(|_| TorusElement { value: 0 as f32})
            .collect();
        TorusGroup { set, element_position: 0 }
    }

    //create a new torus group of size n of random elements 
    pub fn gen_random(n: usize) -> Self {
        let mut rng = rand::rng(); // Updated from `thread_rng` to `rng`
        let set: Vec<TorusElement> = (0..n)
            .map(|_| TorusElement { value: rng.random_range(0.0..1.0) as f32 }) // Updated from `gen_range` to `random_range`
            .collect();
        TorusGroup { set, element_position: 0 }
    }

    //Expand torus object with a new torus element, perform implicit conversion if supplied value is not in the required torus range (0,1] 
    pub fn push(&mut self, new_torus_element: f32) {

        //perform implicit conversion if supplied torus value is not in the range 0,1
        let new_element = TorusElement::new(new_torus_element);
        self.set.push(new_element);
        self.element_position = self.set.len() -1;
    }
}


// Iterator trait for Torus used to iterate over owned values in the Torus struct
impl Iterator for TorusGroup {
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

// Iterator trait for Torus used to iterate over borrowed values in the Torus struct
impl<'a> IntoIterator for &'a TorusGroup {
    type Item = &'a TorusElement;
    type IntoIter = std::slice::Iter<'a, TorusElement>;

    fn into_iter(self) -> Self::IntoIter {
        self.set.iter()
    }
}


impl Add for TorusElement {
    type Output = TorusElement;

    fn add(self, other: TorusElement) -> Self::Output {
        let result = (self.value + other.value).rem_euclid(1.0); 
        TorusElement { value: (result) }
    }
}

impl Neg for TorusElement {
    type Output = TorusElement;

    fn neg(self) -> Self::Output {
        TorusElement { value: (-self.value).rem_euclid(1.0) }
    }
}

impl Sub for TorusElement {
    type Output = TorusElement;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl PartialEq for TorusElement  {
    fn eq(&self, other: &Self) -> bool {
        let epsilon = 1e-6;
        let diff = (self.value - other.value).abs();
        (diff < epsilon) || ((1.0 - diff).abs() < epsilon)
    }
    fn ne(&self, other: &Self) -> bool {
        let epsilon = 1e-6;
        let diff = (self.value - other.value).abs();
        (diff > epsilon) || ((1.0 - diff).abs() > epsilon)
    }
}


//external product operation between integer k and torus element t
//t * k
// if k < 0, then -t * -k
impl Mul<i32> for TorusElement
{
    type Output = TorusElement;

    fn mul(self, rhs: i32) -> Self::Output {

        if rhs < 0 {
            let minus_k = (-rhs) as f32;
            let result = ((-self.value) * (minus_k)).rem_euclid(1.0);
            TorusElement { value: result }
        } else {
            let result = (self.value * rhs as f32).rem_euclid(1.0);
            TorusElement { value: result }
        }
    }
}

//k * t
//if k < 0, -k * -t
impl Mul<TorusElement> for i32
{
    type Output = TorusElement;

    fn mul(self, rhs: TorusElement) -> Self::Output {

        if self < 0 { 
            // -k * -t
            let minus_k = (-self) as f32;
            let result = (minus_k * (-rhs.value)).rem_euclid(1.0);
            TorusElement { value: result }
        } else {
            // k * t
            let result = (self as f32 * (rhs.value)).rem_euclid(1.0);
            TorusElement { value: result }
        }
    }
}