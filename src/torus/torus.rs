use std::vec;

use rand::Rng; // bring the trait into scope


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

    //Expand torus object with a new torus element, perform implicit conversion if supplied value is not in the required torus range (0,1] 
    pub fn push(&mut self, new_torus_element: f32) {

        //perform implicit conversion if supplied torus value is not in the range 0,1
        let new_element = TorusElement::new(new_torus_element);
        self.set.push(new_element);
        self.element_position = self.set.len() -1;
    }
}


// Iterator trait for Torus used to iterate over owned values in the Torus struct
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

// Iterator trait for Torus used to iterate over borrowed values in the Torus struct
impl<'a> IntoIterator for &'a Torus {
    type Item = &'a TorusElement;
    type IntoIter = std::slice::Iter<'a, TorusElement>;

    fn into_iter(self) -> Self::IntoIter {
        self.set.iter()
    }
}



// Torus Polynominals struct
// Considering the polynomial rings RN [X] := R[X]/(X^N +1) and ZN [X] := Z[X]/(X^N + 1), this defines the ZN [X]-module
// TN [X] := RN [X]/ZN [X] = T[X]/(X^N + 1)
// Elements of TN [X] can therefore be seen as polynomials modulo XN + 1 with coefficients in T
#[derive(Debug)]
pub struct TorusPolynominal {
    pub n: usize,                     //degree
    pub m: usize,                     //cyclotomic index (X^N + 1)
    pub coeffs: Vec<TorusElement>     //set of torus elements
}

impl TorusPolynominal {

    // constructing a zero polynominal of degree n
    pub fn zero(degree: usize, cyclotomic_index: usize) -> Self {
        Self { 
            n: degree, 
            m: cyclotomic_index, 
            coeffs: vec![TorusElement {value: 0.0}; degree],
        }
    }

    // constructing a new polynominal from an existing vector
    pub fn from_coeffs(input_coeffs: Vec<TorusElement>, cyclotomic_index: usize) -> Self {
        let n = input_coeffs.len();
        Self { n: n, m: cyclotomic_index, coeffs: input_coeffs }
    }

    //construct a random torus polynominal of order n and cyclotomic order m
    pub fn new_random(degree: usize, cyclotomic_index: usize) -> Self {
        let mut rng = rand::rng();
        let set: Vec<TorusElement> = (0..degree)
            .map(|_| TorusElement { value:  rng.random_range(0.0..1.0)})
            .collect();

        Self { n: degree, m: cyclotomic_index, coeffs: set }
    }
}

